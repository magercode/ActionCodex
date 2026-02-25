use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::Style;
use ratatui::text::Line;
use ratatui::widgets::{
    Block, Borders, Clear, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState,
};
use ratatui::Frame;

use super::theme::ThemeMode;
use super::types::{ManagerAction, ManagerDialog, ManagerMode};

pub fn render_manager_dialog(frame: &mut Frame, dialog: &ManagerDialog, theme: ThemeMode) {
    let palette = theme.palette();
    let area = centered_rect(frame.area(), 74, 18);
    frame.render_widget(Clear, area);
    frame.render_widget(
        Block::default()
            .title(" Rust/Cargo Manager ")
            .borders(Borders::ALL),
        area,
    );

    let inner = Layout::vertical([
        Constraint::Length(1),
        Constraint::Min(5),
        Constraint::Length(4),
    ])
    .margin(1)
    .split(area);

    frame.render_widget(
        Paragraph::new("Up/Down pilih | Enter jalan | Esc tutup | Ctrl+K buka"),
        inner[0],
    );

    match dialog.mode {
        ManagerMode::Menu => {
            let lines = ManagerDialog::actions()
                .iter()
                .enumerate()
                .map(|(idx, action)| {
                    let label = action_label(*action);
                    if idx == dialog.selected {
                        Line::styled(
                            format!("> {}", label),
                            Style::default()
                                .fg(palette.accent_text)
                                .bg(palette.accent),
                        )
                    } else {
                        Line::from(format!("  {}", label))
                    }
                })
                .collect::<Vec<_>>();
            frame.render_widget(Paragraph::new(lines), inner[1]);
        }
        ManagerMode::Input(action) => {
            let prompt = input_prompt(action);
            frame.render_widget(
                Paragraph::new(prompt),
                Rect::new(inner[1].x, inner[1].y, inner[1].width, 1),
            );

            let input_rect = Rect::new(inner[1].x, inner[1].y + 1, inner[1].width, 3);
            frame.render_widget(
                Paragraph::new(dialog.input.as_str())
                    .block(Block::default().title(" Input ").borders(Borders::ALL)),
                input_rect,
            );

            let width = input_rect.width.saturating_sub(2) as usize;
            let cursor_x = input_rect.x + 1 + dialog.input.len().min(width.saturating_sub(1)) as u16;
            let cursor_y = input_rect.y + 1;
            frame.set_cursor_position((cursor_x, cursor_y));
        }
        ManagerMode::SearchResults => {
            let list_rect = inner[1];
            let content_rect = Rect::new(
                list_rect.x + 1,
                list_rect.y + 1,
                list_rect.width.saturating_sub(3),
                list_rect.height.saturating_sub(2),
            );
            let viewport_h = content_rect.height.max(1) as usize;
            let visible = dialog
                .search_results
                .iter()
                .enumerate()
                .skip(dialog.search_scroll_y)
                .take(viewport_h)
                .map(|(idx, item)| {
                    let mark = if item.installed { "[INSTALLED]" } else { "[ ]" };
                    let line = format!(
                        "{} {} {} - {}",
                        mark, item.name, item.version, item.description
                    );
                    if idx == dialog.search_selected {
                        Line::styled(
                            format!("> {}", line),
                            Style::default().fg(palette.accent_text).bg(palette.accent),
                        )
                    } else {
                        Line::from(format!("  {}", line))
                    }
                })
                .collect::<Vec<_>>();

            let content = Paragraph::new(visible)
                .block(Block::default().title(" Hasil Crates ").borders(Borders::ALL))
                .scroll((0, dialog.search_scroll_x as u16));
            frame.render_widget(content, list_rect);

            let mut v_state = ScrollbarState::new(dialog.search_results.len())
                .viewport_content_length(viewport_h)
                .position(dialog.search_scroll_y);
            frame.render_stateful_widget(
                Scrollbar::new(ScrollbarOrientation::VerticalRight),
                list_rect,
                &mut v_state,
            );

            let max_line = dialog
                .search_results
                .iter()
                .map(|item| item.name.len() + item.version.len() + item.description.len() + 24)
                .max()
                .unwrap_or(1);
            let mut h_state = ScrollbarState::new(max_line)
                .viewport_content_length(content_rect.width.max(1) as usize)
                .position(dialog.search_scroll_x);
            frame.render_stateful_widget(
                Scrollbar::new(ScrollbarOrientation::HorizontalBottom),
                list_rect,
                &mut h_state,
            );
        }
        ManagerMode::Output => {
            let out_rect = inner[1];
            let content_rect = Rect::new(
                out_rect.x + 1,
                out_rect.y + 1,
                out_rect.width.saturating_sub(3),
                out_rect.height.saturating_sub(2),
            );
            frame.render_widget(
                Paragraph::new(dialog.output.as_str())
                    .block(Block::default().title(" Output ").borders(Borders::ALL))
                    .scroll((dialog.output_scroll_y as u16, dialog.output_scroll_x as u16)),
                out_rect,
            );

            let total_lines = dialog.output.lines().count().max(1);
            let mut v_state = ScrollbarState::new(total_lines)
                .viewport_content_length(content_rect.height.max(1) as usize)
                .position(dialog.output_scroll_y);
            frame.render_stateful_widget(
                Scrollbar::new(ScrollbarOrientation::VerticalRight),
                out_rect,
                &mut v_state,
            );

            let max_line_len = dialog.output.lines().map(str::len).max().unwrap_or(1);
            let mut h_state = ScrollbarState::new(max_line_len)
                .viewport_content_length(content_rect.width.max(1) as usize)
                .position(dialog.output_scroll_x);
            frame.render_stateful_widget(
                Scrollbar::new(ScrollbarOrientation::HorizontalBottom),
                out_rect,
                &mut h_state,
            );
        }
    }

    let footer = match dialog.mode {
        ManagerMode::Menu => "Enter: eksekusi aksi | Esc: tutup",
        ManagerMode::Input(_) => "Ketik input lalu Enter | Esc: kembali menu",
        ManagerMode::SearchResults => {
            "Up/Down pilih paket | Enter install/hapus | Left/Right scroll | Esc kembali"
        }
        ManagerMode::Output => "Up/Down/Left/Right scroll | Enter/Esc: kembali menu",
    };
    frame.render_widget(
        Paragraph::new(footer)
            .style(Style::default().fg(palette.status_fg).bg(palette.status_bg)),
        inner[2],
    );
}

fn action_label(action: ManagerAction) -> &'static str {
    match action {
        ManagerAction::FormatRust => "Format file Rust aktif",
        ManagerAction::CargoSearch => "Cari crate di crates.io",
        ManagerAction::CargoAdd => "Install crate (cargo add)",
        ManagerAction::CargoRemove => "Hapus crate (cargo remove)",
        ManagerAction::WorkspaceAddMember => "Tambah workspace member",
        ManagerAction::WorkspaceRemoveMember => "Hapus workspace member",
        ManagerAction::Close => "Tutup manager",
    }
}

fn input_prompt(action: ManagerAction) -> &'static str {
    match action {
        ManagerAction::CargoSearch => "Keyword crate:",
        ManagerAction::CargoAdd => "Nama crate untuk di-install:",
        ManagerAction::CargoRemove => "Nama crate untuk dihapus:",
        ManagerAction::WorkspaceAddMember => "Path member workspace (mis. crates/core):",
        ManagerAction::WorkspaceRemoveMember => "Path member workspace yang dihapus:",
        _ => "Input:",
    }
}

fn centered_rect(area: Rect, width_percent: u16, height: u16) -> Rect {
    let max_width = area.width.saturating_sub(2).max(1);
    let max_height = area.height.saturating_sub(2).max(1);
    let width = area.width.saturating_mul(width_percent) / 100;
    let width = width.max(40).min(max_width);
    let height = height.max(10).min(max_height);
    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;
    Rect::new(x, y, width, height)
}
