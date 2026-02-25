mod context_menu;
mod editor_view;
mod explorer_context_menu;
mod explorer_input_dialog;
mod gutter;
mod help_dialog;
mod manager_dialog;
mod save_dialog;
mod search_dialog;
mod start_menu;
mod theme;
mod types;

use ratatui::Frame;

use crate::editor::Editor;
use crate::file_tree::FileTree;
use crate::tab_manager::TabManager;

pub use theme::ThemeMode;
pub use types::{
    CargoSearchItem, ContextMenuDialog, DialogButton, ExplorerContextMenuDialog,
    ExplorerInputDialog, ExplorerInputMode, ExplorerMenuAction, FocusPane, ManagerAction,
    ManagerDialog, ManagerMode, SaveDialog, SearchDialog, StartMenuAction, StartMenuDialog,
    UiState,
};

pub fn editor_viewport_height(frame_height: u16) -> usize {
    frame_height.saturating_sub(8) as usize
}

pub fn render(
    frame: &mut Frame,
    editor: &Editor,
    save_dialog: Option<&SaveDialog>,
    search_dialog: Option<&SearchDialog>,
    manager_dialog: Option<&ManagerDialog>,
    help_dialog_open: bool,
    start_menu: Option<&StartMenuDialog>,
    context_menu: Option<&ContextMenuDialog>,
    explorer_context_menu: Option<&ExplorerContextMenuDialog>,
    explorer_input_dialog: Option<&ExplorerInputDialog>,
    status_message: &str,
    tabs: &TabManager,
    file_tree: &FileTree,
    focus: FocusPane,
    show_file_tree: bool,
    theme: ThemeMode,
) -> UiState {
    let active_search_keyword = search_dialog
        .map(|dialog| dialog.query.trim())
        .filter(|query| !query.is_empty());
    let parts = editor_view::render_editor(
        frame,
        editor,
        status_message,
        tabs,
        file_tree,
        focus,
        show_file_tree,
        active_search_keyword,
        theme,
    );
    let mut state = UiState {
        tab_hits: parts.tab_hits.clone(),
        tab_content_rect: parts.tab_content_rect,
        tree_content_rect: parts.tree_content_rect,
        editor_content_rect: parts.editor_content_rect,
        save_dialog_hit: None,
        context_menu_hit: None,
        explorer_context_menu_hit: None,
        explorer_input_dialog_hit: None,
    };
    if let Some(dialog) = start_menu {
        start_menu::render_start_menu(frame, dialog);
    } else if let Some(dialog) = manager_dialog {
        manager_dialog::render_manager_dialog(frame, dialog, theme);
    } else if let Some(dialog) = save_dialog {
        state.save_dialog_hit = Some(save_dialog::render_save_dialog(frame, dialog));
    } else if let Some(dialog) = explorer_input_dialog {
        state.explorer_input_dialog_hit = Some(
            explorer_input_dialog::render_explorer_input_dialog(frame, dialog),
        );
    } else if let Some(dialog) = context_menu {
        state.context_menu_hit = Some(context_menu::render_context_menu(frame, dialog));
    } else if let Some(dialog) = explorer_context_menu {
        state.explorer_context_menu_hit = Some(
            explorer_context_menu::render_explorer_context_menu(frame, dialog),
        );
    } else if let Some(dialog) = search_dialog {
        search_dialog::render_search_dialog(frame, dialog);
    } else if help_dialog_open {
        help_dialog::render_help_dialog(frame);
    } else if focus == FocusPane::Editor {
        editor_view::render_editor_cursor(frame, editor, &parts);
    }
    state
}
