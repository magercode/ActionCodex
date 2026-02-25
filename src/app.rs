use std::fs;
use std::io::Stdout;
use std::path::{Component, Path, PathBuf};
use std::time::Duration;

use anyhow::{anyhow, Result};
use arboard::Clipboard;
use crossterm::event::{
    self, Event, KeyCode, KeyEventKind, KeyModifiers, MouseButton, MouseEventKind,
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use crate::editor::Editor;
use crate::file_tree::FileTree;
use crate::manager::cargo_manager;
use crate::manager::rust_formatter;
use crate::manager::workspace_manager;
use crate::signature::write_signature_blob_for_file;
use crate::tab_manager::TabManager;
use crate::ui;

pub fn run(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    startup_files: &[PathBuf],
) -> Result<()> {
    let mut tabs = TabManager::new();
    let mut file_tree = FileTree::new(std::env::current_dir()?)?;
    let mut focus = ui::FocusPane::Editor;
    let mut save_dialog: Option<ui::SaveDialog> = None;
    let mut search_dialog: Option<ui::SearchDialog> = None;
    let mut manager_dialog: Option<ui::ManagerDialog> = None;
    let mut help_dialog_open = false;
    let mut start_menu = if startup_files.is_empty() {
        Some(ui::StartMenuDialog::default())
    } else {
        None
    };
    let mut editor_context_menu: Option<ui::ContextMenuDialog> = None;
    let mut explorer_context_menu: Option<ui::ExplorerContextMenuDialog> = None;
    let mut explorer_input_dialog: Option<ui::ExplorerInputDialog> = None;
    let mut show_file_tree = true;
    let mut clipboard = String::new();
    let mut system_clipboard = Clipboard::new().ok();
    let mut dragging_editor_selection = false;
    let mut theme = ui::ThemeMode::Dark;
    let mut ui_state = ui::UiState::default();
    let mut status_message =
        String::from("Ctrl+S simpan | Ctrl+F search | Ctrl+Z/Y undo redo | Ctrl+C/V/X editor");

    if !startup_files.is_empty() {
        tabs.clear_tabs();
        let mut opened = 0usize;
        let mut failed = 0usize;
        let mut last_error = String::new();

        for path in startup_files {
            if let Err(err) = tabs.open_or_create_file(path) {
                failed += 1;
                last_error = format!("{} ({})", path.to_string_lossy(), err);
            } else {
                opened += 1;
            }
        }

        if tabs.tabs().is_empty() {
            tabs.new_tab();
        } else {
            tabs.set_active(0);
        }

        status_message = if failed == 0 {
            format!("Membuka {} file via CLI", opened)
        } else {
            format!(
                "Membuka {} file, {} gagal (terakhir: {})",
                opened, failed, last_error
            )
        };
    }

    loop {
        terminal.draw(|frame| {
            let viewport_height = ui::editor_viewport_height(frame.area().height);
            tabs.active_editor_mut()
                .ensure_cursor_visible(viewport_height.max(1));
            file_tree.ensure_visible(viewport_height.max(1));
            ui_state = ui::render(
                frame,
                tabs.active_editor(),
                save_dialog.as_ref(),
                search_dialog.as_ref(),
                manager_dialog.as_ref(),
                help_dialog_open,
                start_menu.as_ref(),
                editor_context_menu.as_ref(),
                explorer_context_menu.as_ref(),
                explorer_input_dialog.as_ref(),
                &status_message,
                &tabs,
                &file_tree,
                focus,
                show_file_tree,
                theme,
            );
        })?;

        if !event::poll(Duration::from_millis(200))? {
            continue;
        }

        let ev = event::read()?;
        if let Event::Key(key) = ev {
            if key.kind != KeyEventKind::Press {
                continue;
            }

            if let Some(menu) = start_menu.as_mut() {
                match key.code {
                    KeyCode::Esc => {
                        start_menu = None;
                        status_message = String::from("Start menu ditutup");
                    }
                    KeyCode::Up => menu.move_up(),
                    KeyCode::Down => menu.move_down(),
                    KeyCode::Enter => {
                        match menu.current_action() {
                            ui::StartMenuAction::NewTab => {
                                tabs.new_tab();
                                focus = ui::FocusPane::Editor;
                                status_message = String::from("Tab baru dibuat");
                            }
                            ui::StartMenuAction::FocusFileTree => {
                                if show_file_tree {
                                    focus = ui::FocusPane::FileTree;
                                    status_message = String::from("Fokus ke file tree");
                                } else {
                                    status_message = String::from("File tree sedang disembunyikan");
                                }
                            }
                            ui::StartMenuAction::OpenManager => {
                                manager_dialog = Some(ui::ManagerDialog::default());
                                status_message = String::from("Rust/Cargo manager dibuka");
                            }
                            ui::StartMenuAction::ContinueEditor => {
                                focus = ui::FocusPane::Editor;
                                status_message = String::from("Lanjut ke editor");
                            }
                            ui::StartMenuAction::ShowHelp => {
                                help_dialog_open = true;
                                status_message = String::from("Bantuan dibuka");
                            }
                        }
                        start_menu = None;
                    }
                    _ => {}
                }
                continue;
            }

            if help_dialog_open {
                match key.code {
                    KeyCode::Esc | KeyCode::Enter | KeyCode::F(1) => {
                        help_dialog_open = false;
                        status_message = String::from("Bantuan ditutup");
                    }
                    _ => {}
                }
                continue;
            }

            if let Some(dialog) = manager_dialog.as_mut() {
                match dialog.mode {
                    ui::ManagerMode::Menu => match key.code {
                        KeyCode::Esc => {
                            manager_dialog = None;
                            status_message = String::from("Rust/Cargo manager ditutup");
                        }
                        KeyCode::Up => dialog.move_up(),
                        KeyCode::Down => dialog.move_down(),
                        KeyCode::Enter => match dialog.current_action() {
                            ui::ManagerAction::FormatRust => {
                                let result = format_active_rust_file(&mut tabs);
                                match result {
                                    Ok((message, changed)) => {
                                        if changed {
                                            tabs.mark_active_dirty();
                                        }
                                        dialog.output = message.clone();
                                        dialog.mode = ui::ManagerMode::Output;
                                        status_message = message;
                                    }
                                    Err(err) => {
                                        dialog.output = err.to_string();
                                        dialog.mode = ui::ManagerMode::Output;
                                        status_message = String::from("Format Rust gagal");
                                    }
                                }
                            }
                            ui::ManagerAction::CargoSearch
                            | ui::ManagerAction::CargoAdd
                            | ui::ManagerAction::CargoRemove
                            | ui::ManagerAction::WorkspaceAddMember
                            | ui::ManagerAction::WorkspaceRemoveMember => {
                                dialog.input.clear();
                                dialog.output.clear();
                                dialog.mode = ui::ManagerMode::Input(dialog.current_action());
                            }
                            ui::ManagerAction::Close => {
                                manager_dialog = None;
                                status_message = String::from("Rust/Cargo manager ditutup");
                            }
                        },
                        _ => {}
                    },
                    ui::ManagerMode::Input(action) => match key.code {
                        KeyCode::Esc => {
                            dialog.mode = ui::ManagerMode::Menu;
                            dialog.input.clear();
                        }
                        KeyCode::Backspace => {
                            dialog.input.pop();
                        }
                        KeyCode::Enter => {
                            let input = dialog.input.trim().to_string();
                            match action {
                                ui::ManagerAction::CargoSearch => {
                                    match cargo_manager::search_crate_packages(&input) {
                                        Ok(results) => {
                                            let installed = cargo_manager::installed_packages()
                                                .unwrap_or_default();
                                            dialog.set_search_results(
                                                results
                                                    .into_iter()
                                                    .map(|pkg| ui::CargoSearchItem {
                                                        name: pkg.name,
                                                        version: pkg.version,
                                                        description: pkg.description,
                                                        installed: false,
                                                    })
                                                    .collect::<Vec<_>>(),
                                                installed,
                                            );
                                            status_message = String::from(
                                                "Pilih paket lalu Enter untuk install/hapus",
                                            );
                                        }
                                        Err(err) => {
                                            dialog.set_output(err.to_string());
                                            status_message = String::from("Search crate gagal");
                                        }
                                    }
                                }
                                _ => match run_manager_input_action(action, &input) {
                                    Ok(message) => {
                                        dialog.set_output(message.clone());
                                        status_message = message;
                                        if matches!(
                                            action,
                                            ui::ManagerAction::CargoAdd
                                                | ui::ManagerAction::CargoRemove
                                                | ui::ManagerAction::WorkspaceAddMember
                                                | ui::ManagerAction::WorkspaceRemoveMember
                                        ) {
                                            let _ = file_tree.refresh();
                                        }
                                    }
                                    Err(err) => {
                                        dialog.set_output(err.to_string());
                                        status_message = format!("Manager gagal: {}", err);
                                    }
                                },
                            }
                        }
                        KeyCode::Char(ch) => {
                            if !key.modifiers.contains(KeyModifiers::CONTROL) {
                                dialog.input.push(ch);
                            }
                        }
                        _ => {}
                    },
                    ui::ManagerMode::SearchResults => match key.code {
                        KeyCode::Esc => {
                            dialog.mode = ui::ManagerMode::Menu;
                        }
                        KeyCode::Up => {
                            dialog.move_search_up();
                        }
                        KeyCode::Down => {
                            dialog.move_search_down(8);
                        }
                        KeyCode::Left => {
                            dialog.search_scroll_x = dialog.search_scroll_x.saturating_sub(2);
                        }
                        KeyCode::Right => {
                            dialog.search_scroll_x = dialog.search_scroll_x.saturating_add(2);
                        }
                        KeyCode::Enter => {
                            if let Some(name) = dialog.selected_search_name() {
                                let installed = dialog.installed_packages.contains(&name);
                                let result = if installed {
                                    cargo_manager::remove_crate(&name)
                                } else {
                                    cargo_manager::add_crate(&name)
                                };
                                match result {
                                    Ok(message) => {
                                        dialog.mark_installed(&name, !installed);
                                        dialog.output = message;
                                        dialog.output_scroll_y = 0;
                                        dialog.output_scroll_x = 0;
                                        status_message = if installed {
                                            format!("Paket dihapus: {}", name)
                                        } else {
                                            format!("Paket di-install: {}", name)
                                        };
                                    }
                                    Err(err) => {
                                        dialog.output = err.to_string();
                                        dialog.output_scroll_y = 0;
                                        dialog.output_scroll_x = 0;
                                        status_message = format!("Aksi paket gagal: {}", err);
                                    }
                                }
                            }
                        }
                        _ => {}
                    },
                    ui::ManagerMode::Output => match key.code {
                        KeyCode::Esc | KeyCode::Enter => {
                            dialog.mode = ui::ManagerMode::Menu;
                        }
                        KeyCode::Up => {
                            dialog.output_scroll_y = dialog.output_scroll_y.saturating_sub(1);
                        }
                        KeyCode::Down => {
                            dialog.output_scroll_y = dialog.output_scroll_y.saturating_add(1);
                        }
                        KeyCode::Left => {
                            dialog.output_scroll_x = dialog.output_scroll_x.saturating_sub(2);
                        }
                        KeyCode::Right => {
                            dialog.output_scroll_x = dialog.output_scroll_x.saturating_add(2);
                        }
                        _ => {}
                    },
                }
                continue;
            }

            if let Some(dialog) = save_dialog.as_mut() {
                match key.code {
                    KeyCode::Esc => {
                        save_dialog = None;
                        status_message = String::from("Simpan dibatalkan");
                    }
                    KeyCode::Tab | KeyCode::Left | KeyCode::Right => dialog.toggle_button(),
                    KeyCode::Backspace => dialog.pop_char(),
                    KeyCode::Enter => {
                        if dialog.selected == ui::DialogButton::Cancel {
                            save_dialog = None;
                            status_message = String::from("Simpan dibatalkan");
                        } else {
                            let filename = dialog.filename.trim().to_string();
                            save_to_filename(
                                &mut tabs,
                                &mut file_tree,
                                &mut save_dialog,
                                &mut status_message,
                                filename,
                            )?;
                        }
                    }
                    KeyCode::Char(ch) => {
                        if !key.modifiers.contains(KeyModifiers::CONTROL) {
                            dialog.push_char(ch);
                        }
                    }
                    _ => {}
                }
                continue;
            }

            if let Some(dialog) = explorer_input_dialog.as_mut() {
                match key.code {
                    KeyCode::Esc => {
                        explorer_input_dialog = None;
                        status_message = String::from("Dialog explorer ditutup");
                    }
                    KeyCode::Tab | KeyCode::Left | KeyCode::Right => dialog.toggle_button(),
                    KeyCode::Backspace => dialog.pop_char(),
                    KeyCode::Enter => {
                        if dialog.selected == ui::DialogButton::Cancel {
                            explorer_input_dialog = None;
                            status_message = String::from("Aksi explorer dibatalkan");
                        } else {
                            let input = dialog.input.trim().to_string();
                            let mode = dialog.mode;
                            let base_dir = dialog.base_dir.clone();
                            let target_path = dialog.target_path.clone();
                            match run_explorer_input_action(
                                mode,
                                &base_dir,
                                target_path.as_deref(),
                                &input,
                                file_tree.root(),
                            ) {
                                Ok(message) => {
                                    explorer_input_dialog = None;
                                    explorer_context_menu = None;
                                    status_message = message;
                                    file_tree.refresh()?;
                                }
                                Err(err) => {
                                    status_message = format!("Aksi explorer gagal ({})", err);
                                }
                            }
                        }
                    }
                    KeyCode::Char(ch) => {
                        if !key.modifiers.contains(KeyModifiers::CONTROL) {
                            dialog.push_char(ch);
                        }
                    }
                    _ => {}
                }
                continue;
            }

            if editor_context_menu.is_some() || explorer_context_menu.is_some() {
                match key.code {
                    KeyCode::Esc => {
                        editor_context_menu = None;
                        explorer_context_menu = None;
                        status_message = String::from("Menu klik kanan ditutup");
                    }
                    _ => {}
                }
                continue;
            }

            if let Some(dialog) = search_dialog.as_mut() {
                match key.code {
                    KeyCode::Esc => {
                        search_dialog = None;
                        status_message = String::from("Search ditutup");
                    }
                    KeyCode::Backspace => dialog.pop_char(),
                    KeyCode::Enter => {
                        let query = dialog.query.trim().to_string();
                        if query.is_empty() {
                            status_message = String::from("Keyword search tidak boleh kosong");
                        } else if let Some((row, col)) =
                            find_next_match(tabs.active_editor(), &query)
                        {
                            tabs.active_editor_mut().set_cursor(row, col);
                            focus = ui::FocusPane::Editor;
                            status_message = format!(
                                "Ditemukan '{}' di baris {}, kolom {}",
                                query,
                                row + 1,
                                col + 1
                            );
                        } else {
                            status_message = format!("Keyword '{}' tidak ditemukan", query);
                        }
                    }
                    KeyCode::Char(ch) => {
                        if !key.modifiers.contains(KeyModifiers::CONTROL) {
                            dialog.push_char(ch);
                        }
                    }
                    _ => {}
                }
                continue;
            }

            match key.code {
                KeyCode::Esc => break,
                KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => break,
                KeyCode::F(1) => {
                    help_dialog_open = true;
                    status_message = String::from("Bantuan dibuka");
                }
                KeyCode::Char('m') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    start_menu = Some(ui::StartMenuDialog::default());
                    status_message = String::from("Start menu dibuka");
                }
                KeyCode::Char('f')
                    if key.modifiers.contains(KeyModifiers::CONTROL)
                        && !key.modifiers.contains(KeyModifiers::SHIFT) =>
                {
                    search_dialog = Some(ui::SearchDialog::default());
                    status_message = String::from("Search keyword dibuka");
                }
                KeyCode::Char(ch)
                    if focus == ui::FocusPane::Editor
                        && key.modifiers.contains(KeyModifiers::CONTROL)
                        && key.modifiers.contains(KeyModifiers::SHIFT)
                        && ch.eq_ignore_ascii_case(&'f') =>
                {
                    match format_active_rust_file(&mut tabs) {
                        Ok((message, changed)) => {
                            if changed {
                                tabs.mark_active_dirty();
                            }
                            status_message = message;
                        }
                        Err(err) => {
                            status_message = format!("Format Rust gagal: {}", err);
                        }
                    }
                }
                KeyCode::Char('k') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    manager_dialog = Some(ui::ManagerDialog::default());
                    status_message = String::from("Rust/Cargo manager dibuka");
                }
                KeyCode::Char('n') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    tabs.new_tab();
                    focus = ui::FocusPane::Editor;
                    status_message = String::from("Tab baru dibuat");
                }
                KeyCode::Char('w') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    tabs.close_active_tab();
                    focus = ui::FocusPane::Editor;
                    status_message = format!("Tab ditutup, aktif: {}", tabs.active_tab_title());
                }
                KeyCode::Char('b') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    show_file_tree = !show_file_tree;
                    if !show_file_tree && focus == ui::FocusPane::FileTree {
                        focus = ui::FocusPane::Editor;
                    }
                    status_message = if show_file_tree {
                        String::from("File tree ditampilkan")
                    } else {
                        String::from("File tree disembunyikan")
                    };
                }
                KeyCode::Char('t') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    theme = theme.toggle();
                    status_message = format!("Tema diubah: {}", theme.label());
                }
                KeyCode::Tab if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    tabs.next_tab();
                    status_message = format!("Tab aktif: {}", tabs.active_tab_title());
                }
                KeyCode::BackTab if key.modifiers.contains(KeyModifiers::SHIFT) => {
                    tabs.prev_tab();
                    status_message = format!("Tab aktif: {}", tabs.active_tab_title());
                }
                KeyCode::Tab => {
                    focus = match focus {
                        ui::FocusPane::Editor if show_file_tree => ui::FocusPane::FileTree,
                        ui::FocusPane::Editor => ui::FocusPane::Tabs,
                        ui::FocusPane::FileTree => ui::FocusPane::Tabs,
                        ui::FocusPane::Tabs => ui::FocusPane::Editor,
                    };
                    status_message = match focus {
                        ui::FocusPane::Editor => String::from("Fokus ke editor"),
                        ui::FocusPane::FileTree => String::from("Fokus ke file tree"),
                        ui::FocusPane::Tabs => String::from("Fokus ke tabs"),
                    };
                }
                KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    let mut dialog = ui::SaveDialog::default();
                    dialog.filename = tabs.active_suggested_filename();
                    save_dialog = Some(dialog);
                    status_message = String::from("Dialog simpan terbuka");
                }
                KeyCode::Enter if focus == ui::FocusPane::FileTree => {
                    if let Some(path) = file_tree.selected_path().map(ToOwned::to_owned) {
                        if path.is_dir() {
                            file_tree.toggle_selected_dir()?;
                        } else if let Err(err) = tabs.open_file(&path) {
                            status_message = format!("Gagal buka file ({})", err);
                        } else {
                            focus = ui::FocusPane::Editor;
                            status_message = format!("File dibuka: {}", path.to_string_lossy());
                        }
                    }
                }
                KeyCode::Up if focus == ui::FocusPane::FileTree => file_tree.move_up(),
                KeyCode::Down if focus == ui::FocusPane::FileTree => file_tree.move_down(),
                KeyCode::Left if focus == ui::FocusPane::Tabs => {
                    tabs.prev_tab();
                    status_message = format!("Tab aktif: {}", tabs.active_tab_title());
                }
                KeyCode::Right if focus == ui::FocusPane::Tabs => {
                    tabs.next_tab();
                    status_message = format!("Tab aktif: {}", tabs.active_tab_title());
                }
                KeyCode::Enter if focus == ui::FocusPane::Tabs => {
                    focus = ui::FocusPane::Editor;
                    status_message = String::from("Fokus ke editor");
                }
                KeyCode::Char('a')
                    if focus == ui::FocusPane::Editor
                        && key.modifiers.contains(KeyModifiers::CONTROL) =>
                {
                    tabs.active_editor_mut().select_all();
                    status_message = String::from("Select all aktif");
                }
                KeyCode::Up
                    if focus == ui::FocusPane::Editor
                        && key.modifiers.contains(KeyModifiers::ALT) =>
                {
                    if tabs.active_editor_mut().move_selected_lines_up() {
                        tabs.mark_active_dirty();
                        status_message = String::from("Blok/baris dipindah ke atas");
                    }
                }
                KeyCode::Down
                    if focus == ui::FocusPane::Editor
                        && key.modifiers.contains(KeyModifiers::ALT) =>
                {
                    if tabs.active_editor_mut().move_selected_lines_down() {
                        tabs.mark_active_dirty();
                        status_message = String::from("Blok/baris dipindah ke bawah");
                    }
                }
                KeyCode::Char(ch)
                    if focus == ui::FocusPane::Editor
                        && key.modifiers.contains(KeyModifiers::CONTROL)
                        && ch.eq_ignore_ascii_case(&'z') =>
                {
                    let did_change = if key.modifiers.contains(KeyModifiers::SHIFT) {
                        tabs.active_editor_mut().redo()
                    } else {
                        tabs.active_editor_mut().undo()
                    };
                    if did_change {
                        tabs.mark_active_dirty();
                        status_message = if key.modifiers.contains(KeyModifiers::SHIFT) {
                            String::from("Redo berhasil")
                        } else {
                            String::from("Undo berhasil")
                        };
                    } else {
                        status_message = if key.modifiers.contains(KeyModifiers::SHIFT) {
                            String::from("Tidak ada redo")
                        } else {
                            String::from("Tidak ada undo")
                        };
                    }
                }
                KeyCode::Char('y')
                    if focus == ui::FocusPane::Editor
                        && key.modifiers.contains(KeyModifiers::CONTROL) =>
                {
                    if tabs.active_editor_mut().redo() {
                        tabs.mark_active_dirty();
                        status_message = String::from("Redo berhasil");
                    } else {
                        status_message = String::from("Tidak ada redo");
                    }
                }
                KeyCode::Char('c')
                    if focus == ui::FocusPane::Editor
                        && key.modifiers.contains(KeyModifiers::CONTROL) =>
                {
                    let copied = if let Some(selected) = tabs.active_editor().selected_text() {
                        selected
                    } else {
                        tabs.active_editor().current_line_text()
                    };
                    let wrote_system =
                        write_clipboard_text(&mut system_clipboard, &mut clipboard, copied);
                    status_message = if wrote_system {
                        String::from("Teks disalin ke clipboard")
                    } else {
                        String::from("Teks disalin (fallback clipboard internal)")
                    };
                }
                KeyCode::Char('v')
                    if focus == ui::FocusPane::Editor
                        && key.modifiers.contains(KeyModifiers::CONTROL) =>
                {
                    let paste_text = read_clipboard_text(&mut system_clipboard, &clipboard);
                    if paste_text.is_empty() {
                        status_message = String::from("Clipboard kosong");
                    } else {
                        tabs.active_editor_mut().insert_text(&paste_text);
                        tabs.mark_active_dirty();
                        status_message = String::from("Paste berhasil");
                    }
                }
                KeyCode::Char('x')
                    if focus == ui::FocusPane::Editor
                        && key.modifiers.contains(KeyModifiers::CONTROL) =>
                {
                    let cut_text = tabs.active_editor_mut().cut_current_line();
                    let wrote_system =
                        write_clipboard_text(&mut system_clipboard, &mut clipboard, cut_text);
                    tabs.mark_active_dirty();
                    status_message = if wrote_system {
                        String::from("Teks dipotong ke clipboard")
                    } else {
                        String::from("Teks dipotong (fallback clipboard internal)")
                    };
                }
                KeyCode::Char(ch) if focus == ui::FocusPane::Editor => {
                    tabs.active_editor_mut().type_char_smart(ch);
                    tabs.mark_active_dirty();
                }
                KeyCode::Enter if focus == ui::FocusPane::Editor => {
                    tabs.active_editor_mut().insert_newline_smart();
                    tabs.mark_active_dirty();
                }
                KeyCode::Backspace if focus == ui::FocusPane::Editor => {
                    tabs.active_editor_mut().backspace();
                    tabs.mark_active_dirty();
                }
                KeyCode::Left if focus == ui::FocusPane::Editor => tabs
                    .active_editor_mut()
                    .move_left_selecting(key.modifiers.contains(KeyModifiers::SHIFT)),
                KeyCode::Right if focus == ui::FocusPane::Editor => tabs
                    .active_editor_mut()
                    .move_right_selecting(key.modifiers.contains(KeyModifiers::SHIFT)),
                KeyCode::Up if focus == ui::FocusPane::Editor => tabs
                    .active_editor_mut()
                    .move_up_selecting(key.modifiers.contains(KeyModifiers::SHIFT)),
                KeyCode::Down if focus == ui::FocusPane::Editor => tabs
                    .active_editor_mut()
                    .move_down_selecting(key.modifiers.contains(KeyModifiers::SHIFT)),
                _ => {}
            }
        } else if let Event::Mouse(mouse) = ev {
            if start_menu.is_some()
                || help_dialog_open
                || search_dialog.is_some()
                || manager_dialog.is_some()
            {
                continue;
            }

            if let Some(dialog) = save_dialog.as_mut() {
                if mouse.kind == MouseEventKind::Down(MouseButton::Left) {
                    if let Some(hit) = ui_state.save_dialog_hit {
                        let point = (mouse.column, mouse.row);
                        if point_in_rect(point, hit.save_button_rect) {
                            dialog.selected = ui::DialogButton::Save;
                            let filename = dialog.filename.trim().to_string();
                            save_to_filename(
                                &mut tabs,
                                &mut file_tree,
                                &mut save_dialog,
                                &mut status_message,
                                filename,
                            )?;
                        } else if point_in_rect(point, hit.cancel_button_rect) {
                            dialog.selected = ui::DialogButton::Cancel;
                            save_dialog = None;
                            status_message = String::from("Simpan dibatalkan");
                        } else if point_in_rect(point, hit.input_rect) {
                            dialog.selected = ui::DialogButton::Save;
                        }
                    }
                }
                continue;
            }

            if let Some(dialog) = explorer_input_dialog.as_mut() {
                if mouse.kind == MouseEventKind::Down(MouseButton::Left) {
                    if let Some(hit) = ui_state.explorer_input_dialog_hit {
                        let point = (mouse.column, mouse.row);
                        if point_in_rect(point, hit.save_button_rect) {
                            dialog.selected = ui::DialogButton::Save;
                            let input = dialog.input.trim().to_string();
                            let mode = dialog.mode;
                            let base_dir = dialog.base_dir.clone();
                            let target_path = dialog.target_path.clone();
                            match run_explorer_input_action(
                                mode,
                                &base_dir,
                                target_path.as_deref(),
                                &input,
                                file_tree.root(),
                            ) {
                                Ok(message) => {
                                    explorer_input_dialog = None;
                                    explorer_context_menu = None;
                                    status_message = message;
                                    file_tree.refresh()?;
                                }
                                Err(err) => {
                                    status_message = format!("Aksi explorer gagal ({})", err);
                                }
                            }
                        } else if point_in_rect(point, hit.cancel_button_rect) {
                            dialog.selected = ui::DialogButton::Cancel;
                            explorer_input_dialog = None;
                            status_message = String::from("Aksi explorer dibatalkan");
                        } else if point_in_rect(point, hit.input_rect) {
                            dialog.selected = ui::DialogButton::Save;
                        }
                    }
                }
                continue;
            }

            if let Some(hit) = ui_state.context_menu_hit {
                if mouse.kind == MouseEventKind::Down(MouseButton::Left) {
                    let point = (mouse.column, mouse.row);
                    if point_in_rect(point, hit.select_all_rect) {
                        tabs.active_editor_mut().select_all();
                        editor_context_menu = None;
                        status_message = String::from("Select all aktif");
                    } else if point_in_rect(point, hit.copy_rect) {
                        let copied = if let Some(selected) = tabs.active_editor().selected_text() {
                            selected
                        } else {
                            tabs.active_editor().current_line_text()
                        };
                        let wrote_system =
                            write_clipboard_text(&mut system_clipboard, &mut clipboard, copied);
                        editor_context_menu = None;
                        status_message = if wrote_system {
                            String::from("Teks disalin ke clipboard")
                        } else {
                            String::from("Teks disalin (fallback clipboard internal)")
                        };
                    } else if point_in_rect(point, hit.paste_rect) {
                        let paste_text = read_clipboard_text(&mut system_clipboard, &clipboard);
                        if paste_text.is_empty() {
                            status_message = String::from("Clipboard kosong");
                        } else {
                            tabs.active_editor_mut().insert_text(&paste_text);
                            tabs.mark_active_dirty();
                            status_message = String::from("Paste berhasil");
                        }
                        editor_context_menu = None;
                    } else {
                        editor_context_menu = None;
                    }
                } else if mouse.kind == MouseEventKind::Down(MouseButton::Right) {
                    editor_context_menu = None;
                }
                continue;
            }

            if let Some(hit) = ui_state.explorer_context_menu_hit {
                if mouse.kind == MouseEventKind::Down(MouseButton::Left) {
                    let point = (mouse.column, mouse.row);
                    let clicked_add_folder = hit
                        .add_folder_rect
                        .is_some_and(|rect| point_in_rect(point, rect));
                    let clicked_add_file = hit
                        .add_file_rect
                        .is_some_and(|rect| point_in_rect(point, rect));
                    let clicked_rename_entry = hit
                        .rename_entry_rect
                        .is_some_and(|rect| point_in_rect(point, rect));
                    let clicked_delete_entry = hit
                        .delete_entry_rect
                        .is_some_and(|rect| point_in_rect(point, rect));

                    if clicked_add_folder
                        || clicked_add_file
                        || clicked_rename_entry
                        || clicked_delete_entry
                    {
                        if let Some(menu) = explorer_context_menu.as_ref() {
                            if clicked_add_folder {
                                let base_dir = menu.base_dir.clone();
                                explorer_input_dialog = Some(ui::ExplorerInputDialog::new(
                                    ui::ExplorerInputMode::AddFolder,
                                    base_dir,
                                    None,
                                ));
                                status_message = String::from("Input nama folder dibuka");
                            } else if clicked_add_file {
                                let base_dir = menu.base_dir.clone();
                                explorer_input_dialog = Some(ui::ExplorerInputDialog::new(
                                    ui::ExplorerInputMode::AddFile,
                                    base_dir,
                                    None,
                                ));
                                status_message = String::from("Input nama file dibuka");
                            } else if clicked_rename_entry {
                                if let Some(target_path) = menu.target_path.as_ref() {
                                    let rename_base = target_path
                                        .parent()
                                        .map(ToOwned::to_owned)
                                        .unwrap_or_else(|| file_tree.root().to_path_buf());
                                    explorer_input_dialog = Some(ui::ExplorerInputDialog::new(
                                        ui::ExplorerInputMode::RenameEntry,
                                        rename_base,
                                        Some(target_path.clone()),
                                    ));
                                    status_message = if menu.target_is_dir {
                                        String::from("Input ubah nama folder dibuka")
                                    } else {
                                        String::from("Input ubah nama file dibuka")
                                    };
                                } else {
                                    status_message = String::from("Target rename tidak valid");
                                }
                            } else if clicked_delete_entry {
                                if let Some(target_path) = menu.target_path.as_ref() {
                                    match delete_entry_from_explorer(target_path, file_tree.root())
                                    {
                                        Ok(message) => {
                                            status_message = message;
                                            file_tree.refresh()?;
                                        }
                                        Err(err) => {
                                            status_message = format!("Hapus objek gagal ({})", err);
                                        }
                                    }
                                } else {
                                    status_message = String::from("Target hapus tidak valid");
                                }
                            }
                        }
                        explorer_context_menu = None;
                    } else {
                        explorer_context_menu = None;
                    }
                } else if mouse.kind == MouseEventKind::Down(MouseButton::Right) {
                    explorer_context_menu = None;
                }
                continue;
            }

            match mouse.kind {
                MouseEventKind::Down(MouseButton::Left) => {
                    dragging_editor_selection = false;
                    let point = (mouse.column, mouse.row);
                    if point_in_rect(point, ui_state.tab_content_rect) {
                        if let Some(hit) = ui_state
                            .tab_hits
                            .iter()
                            .find(|hit| point_in_rect(point, hit.rect))
                        {
                            tabs.set_active(hit.index);
                            focus = ui::FocusPane::Tabs;
                            status_message = format!("Tab aktif: {}", tabs.active_tab_title());
                        }
                    } else if point_in_rect(point, ui_state.tree_content_rect) {
                        if show_file_tree {
                            focus = ui::FocusPane::FileTree;
                            let row =
                                mouse.row.saturating_sub(ui_state.tree_content_rect.y) as usize;
                            let idx = file_tree.scroll() + row;
                            if idx < file_tree.entry_count() {
                                file_tree.set_selected(idx);
                                status_message = String::from("Fokus ke file tree");
                            } else {
                                status_message = String::from("Area kosong explorer");
                            }
                        }
                    } else if point_in_rect(point, ui_state.editor_content_rect) {
                        focus = ui::FocusPane::Editor;
                        let x =
                            mouse.column.saturating_sub(ui_state.editor_content_rect.x) as usize;
                        let y = mouse.row.saturating_sub(ui_state.editor_content_rect.y) as usize;
                        tabs.active_editor_mut().set_cursor_from_screen_selecting(
                            x,
                            y,
                            mouse.modifiers.contains(KeyModifiers::SHIFT),
                        );
                        dragging_editor_selection = true;
                        status_message = String::from("Fokus ke editor");
                    }
                }
                MouseEventKind::Drag(MouseButton::Left) => {
                    let point = (mouse.column, mouse.row);
                    if focus == ui::FocusPane::Editor
                        && point_in_rect(point, ui_state.editor_content_rect)
                        && dragging_editor_selection
                    {
                        let x =
                            mouse.column.saturating_sub(ui_state.editor_content_rect.x) as usize;
                        let y = mouse.row.saturating_sub(ui_state.editor_content_rect.y) as usize;
                        tabs.active_editor_mut()
                            .set_cursor_from_screen_selecting(x, y, true);
                    }
                }
                MouseEventKind::Up(MouseButton::Left) => {
                    dragging_editor_selection = false;
                }
                MouseEventKind::Down(MouseButton::Right) => {
                    let point = (mouse.column, mouse.row);
                    if focus == ui::FocusPane::Editor
                        && point_in_rect(point, ui_state.editor_content_rect)
                    {
                        explorer_context_menu = None;
                        editor_context_menu = Some(ui::ContextMenuDialog {
                            x: mouse.column,
                            y: mouse.row,
                        });
                        status_message = String::from("Menu editor dibuka");
                    } else if show_file_tree && point_in_rect(point, ui_state.tree_content_rect) {
                        let row = mouse.row.saturating_sub(ui_state.tree_content_rect.y) as usize;
                        let idx = file_tree.scroll() + row;
                        if idx < file_tree.entry_count() {
                            file_tree.set_selected(idx);
                        }
                        editor_context_menu = None;
                        explorer_context_menu = Some(build_explorer_context_menu(
                            &file_tree,
                            idx,
                            mouse.column,
                            mouse.row,
                        ));
                        focus = ui::FocusPane::FileTree;
                        status_message = String::from("Menu explorer dibuka");
                    } else {
                        editor_context_menu = None;
                        explorer_context_menu = None;
                    }
                }
                MouseEventKind::ScrollUp => {
                    let point = (mouse.column, mouse.row);
                    if point_in_rect(point, ui_state.tab_content_rect) {
                        tabs.prev_tab();
                        status_message = format!("Tab aktif: {}", tabs.active_tab_title());
                    } else if show_file_tree && point_in_rect(point, ui_state.tree_content_rect) {
                        file_tree.move_up();
                    } else if point_in_rect(point, ui_state.editor_content_rect) {
                        tabs.active_editor_mut().move_up_selecting(false);
                    }
                }
                MouseEventKind::ScrollDown => {
                    let point = (mouse.column, mouse.row);
                    if point_in_rect(point, ui_state.tab_content_rect) {
                        tabs.next_tab();
                        status_message = format!("Tab aktif: {}", tabs.active_tab_title());
                    } else if show_file_tree && point_in_rect(point, ui_state.tree_content_rect) {
                        file_tree.move_down();
                    } else if point_in_rect(point, ui_state.editor_content_rect) {
                        tabs.active_editor_mut().move_down_selecting(false);
                    }
                }
                _ => {}
            }
        }
    }

    Ok(())
}

fn save_to_filename(
    tabs: &mut TabManager,
    file_tree: &mut FileTree,
    save_dialog: &mut Option<ui::SaveDialog>,
    status_message: &mut String,
    filename: String,
) -> Result<()> {
    if filename.is_empty() {
        *status_message = String::from("Nama file tidak boleh kosong");
        return Ok(());
    }

    let save_path = PathBuf::from(&filename);
    if let Some(parent) = save_path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        if let Err(err) = fs::create_dir_all(parent) {
            *status_message = format!("Gagal membuat folder target {} ({})", parent.display(), err);
            return Ok(());
        }
    }

    match tabs.save_active_to(save_path.clone()) {
        Ok(()) => {
            *save_dialog = None;
            let content = tabs.active_editor().text();
            match write_signature_blob_for_file(&save_path, content.as_bytes()) {
                Ok(marker_path) => {
                    let marker_name = marker_path
                        .file_name()
                        .map(|name| name.to_string_lossy().to_string())
                        .unwrap_or_else(|| marker_path.to_string_lossy().to_string());
                    *status_message =
                        format!("Berhasil simpan: {} | signature: {}", filename, marker_name);
                }
                Err(err) => {
                    *status_message =
                        format!("Berhasil simpan: {} | signature gagal ({})", filename, err);
                }
            }
            file_tree.refresh()?;
        }
        Err(err) => {
            *status_message = format!("Gagal simpan {} ({})", filename, err);
        }
    }
    Ok(())
}

fn build_explorer_context_menu(
    file_tree: &FileTree,
    index: usize,
    x: u16,
    y: u16,
) -> ui::ExplorerContextMenuDialog {
    let mut actions = vec![
        ui::ExplorerMenuAction::AddFolder,
        ui::ExplorerMenuAction::AddFile,
    ];
    let mut base_dir = file_tree.root().to_path_buf();
    let mut target_path = None;
    let mut target_is_dir = false;

    if let Some(entry) = file_tree.entry(index) {
        actions.push(ui::ExplorerMenuAction::RenameEntry);
        actions.push(ui::ExplorerMenuAction::DeleteEntry);
        target_path = Some(entry.path().to_path_buf());
        if entry.is_dir() {
            base_dir = entry.path().to_path_buf();
            target_is_dir = true;
        } else {
            base_dir = entry
                .path()
                .parent()
                .map(ToOwned::to_owned)
                .unwrap_or_else(|| file_tree.root().to_path_buf());
        }
    }

    ui::ExplorerContextMenuDialog {
        x,
        y,
        base_dir,
        target_path,
        target_is_dir,
        actions,
    }
}

fn run_explorer_input_action(
    mode: ui::ExplorerInputMode,
    base_dir: &Path,
    target_path: Option<&Path>,
    input: &str,
    root: &Path,
) -> Result<String> {
    match mode {
        ui::ExplorerInputMode::AddFolder => {
            let target = resolve_explorer_target(base_dir, input)?;
            if target.exists() {
                return Err(anyhow!("Path sudah ada: {}", target.display()));
            }
            fs::create_dir_all(&target)?;
            Ok(format!("Folder dibuat: {}", target.display()))
        }
        ui::ExplorerInputMode::AddFile => {
            let target = resolve_explorer_target(base_dir, input)?;
            if target.exists() {
                return Err(anyhow!("Path sudah ada: {}", target.display()));
            }
            if let Some(parent) = target
                .parent()
                .filter(|parent| !parent.as_os_str().is_empty())
            {
                fs::create_dir_all(parent)?;
            }
            fs::OpenOptions::new()
                .create_new(true)
                .write(true)
                .open(&target)?;
            Ok(format!("File dibuat: {}", target.display()))
        }
        ui::ExplorerInputMode::RenameEntry => {
            let source = target_path.ok_or_else(|| anyhow!("Target rename tidak valid"))?;
            if !source.exists() {
                return Err(anyhow!("Target rename tidak ditemukan"));
            }
            let source_is_dir = source.is_dir();
            let destination = resolve_explorer_target(base_dir, input)?;
            if destination == source {
                return Err(anyhow!("Nama baru sama dengan nama lama"));
            }
            if destination.exists() {
                return Err(anyhow!("Path tujuan sudah ada: {}", destination.display()));
            }
            validate_entry_inside_root(source, root)?;
            let parent = destination
                .parent()
                .ok_or_else(|| anyhow!("Path tujuan rename tidak valid"))?;
            if !parent.exists() {
                return Err(anyhow!(
                    "Folder tujuan rename tidak ditemukan: {}",
                    parent.display()
                ));
            }
            let parent_canonical = parent.canonicalize()?;
            let root_canonical = root.canonicalize()?;
            if !parent_canonical.starts_with(&root_canonical) {
                return Err(anyhow!("Tujuan rename di luar root tidak diizinkan"));
            }
            fs::rename(source, &destination)?;
            let kind = if source_is_dir { "Folder" } else { "File" };
            Ok(format!(
                "{} diubah nama: {}",
                kind,
                destination.to_string_lossy()
            ))
        }
    }
}

fn delete_entry_from_explorer(target: &Path, root: &Path) -> Result<String> {
    if !target.exists() {
        return Err(anyhow!("Objek tidak ditemukan"));
    }
    let was_dir = target.is_dir();

    validate_entry_inside_root(target, root)?;
    let target_canonical = target.canonicalize()?;
    if was_dir {
        fs::remove_dir_all(&target_canonical)?;
        Ok(format!("Folder dihapus: {}", target_canonical.display()))
    } else {
        fs::remove_file(&target_canonical)?;
        Ok(format!("File dihapus: {}", target_canonical.display()))
    }
}

fn validate_entry_inside_root(target: &Path, root: &Path) -> Result<()> {
    let root_canonical = root.canonicalize()?;
    let target_canonical = target.canonicalize()?;
    if target_canonical == root_canonical {
        return Err(anyhow!("Root explorer tidak dapat dimodifikasi"));
    }
    if !target_canonical.starts_with(&root_canonical) {
        return Err(anyhow!("Akses objek di luar root tidak diizinkan"));
    }
    Ok(())
}

fn resolve_explorer_target(base_dir: &Path, input: &str) -> Result<PathBuf> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(anyhow!("Nama tidak boleh kosong"));
    }

    let rel = Path::new(trimmed);
    if rel.is_absolute() {
        return Err(anyhow!("Path absolut tidak diizinkan"));
    }

    let mut has_name = false;
    for component in rel.components() {
        match component {
            Component::Normal(_) => has_name = true,
            Component::CurDir => {}
            Component::ParentDir | Component::RootDir | Component::Prefix(_) => {
                return Err(anyhow!("Path tidak valid"));
            }
        }
    }
    if !has_name {
        return Err(anyhow!("Nama tidak valid"));
    }

    Ok(base_dir.join(rel))
}

fn find_next_match(editor: &Editor, query: &str) -> Option<(usize, usize)> {
    if query.is_empty() {
        return None;
    }

    let lines = editor.lines();
    let start_row = editor.cursor_y();
    let start_col = editor.cursor_x().saturating_add(1);
    for row in start_row..lines.len() {
        let line = &lines[row];
        let from = if row == start_row { start_col } else { 0 };
        if from <= line.len() {
            if let Some(pos) = line[from..].find(query) {
                return Some((row, from + pos));
            }
        }
    }
    for (row, line) in lines.iter().enumerate().take(start_row + 1) {
        if let Some(pos) = line.find(query) {
            return Some((row, pos));
        }
    }
    None
}

fn point_in_rect(point: (u16, u16), rect: ratatui::layout::Rect) -> bool {
    let (x, y) = point;
    x >= rect.x
        && x < rect.x.saturating_add(rect.width)
        && y >= rect.y
        && y < rect.y.saturating_add(rect.height)
}

fn write_clipboard_text(
    system_clipboard: &mut Option<Clipboard>,
    fallback_clipboard: &mut String,
    text: String,
) -> bool {
    *fallback_clipboard = text.clone();
    system_clipboard
        .as_mut()
        .and_then(|cb| cb.set_text(text).ok())
        .is_some()
}

fn read_clipboard_text(
    system_clipboard: &mut Option<Clipboard>,
    fallback_clipboard: &str,
) -> String {
    system_clipboard
        .as_mut()
        .and_then(|cb| cb.get_text().ok())
        .unwrap_or_else(|| fallback_clipboard.to_string())
}

fn format_active_rust_file(tabs: &mut TabManager) -> Result<(String, bool)> {
    let source = tabs.active_editor().text();
    let formatted = rust_formatter::format_rust_source(&source)?;
    if formatted == source {
        return Ok((String::from("Rust formatter: tidak ada perubahan"), false));
    }
    tabs.active_editor_mut().replace_text(&formatted);
    Ok((String::from("Rust formatter berhasil diterapkan"), true))
}

fn run_manager_input_action(action: ui::ManagerAction, input: &str) -> Result<String> {
    match action {
        ui::ManagerAction::CargoSearch => cargo_manager::search_crate(input),
        ui::ManagerAction::CargoAdd => cargo_manager::add_crate(input),
        ui::ManagerAction::CargoRemove => cargo_manager::remove_crate(input),
        ui::ManagerAction::WorkspaceAddMember => workspace_manager::add_workspace_member(input),
        ui::ManagerAction::WorkspaceRemoveMember => {
            workspace_manager::remove_workspace_member(input)
        }
        _ => Ok(String::from("Aksi tidak membutuhkan input")),
    }
}
