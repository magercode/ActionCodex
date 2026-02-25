use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Clear, Paragraph};
use ratatui::Frame;

use super::types::{ExplorerContextMenuDialog, ExplorerContextMenuHit, ExplorerMenuAction};

pub fn render_explorer_context_menu(
    frame: &mut Frame,
    dialog: &ExplorerContextMenuDialog,
) -> ExplorerContextMenuHit {
    let height = (dialog.actions.len() as u16).saturating_add(2).max(4);
    let area = menu_rect(frame.area(), dialog.x, dialog.y, 28, height);
    frame.render_widget(Clear, area);
    frame.render_widget(
        Block::default()
            .title(" Explorer Menu ")
            .borders(Borders::ALL),
        area,
    );

    let content = Rect::new(
        area.x + 1,
        area.y + 1,
        area.width.saturating_sub(2),
        area.height.saturating_sub(2),
    );

    let lines = dialog
        .actions
        .iter()
        .map(|action| {
            let (label, style) = match action {
                ExplorerMenuAction::AddFolder => (
                    " Tambah Folder",
                    Style::default().fg(Color::Black).bg(Color::Green),
                ),
                ExplorerMenuAction::AddFile => (
                    " Tambah File",
                    Style::default().fg(Color::Black).bg(Color::Cyan),
                ),
                ExplorerMenuAction::RenameEntry => (
                    if dialog.target_is_dir {
                        " Ubah Nama Folder"
                    } else {
                        " Ubah Nama File"
                    },
                    Style::default().fg(Color::Black).bg(Color::Yellow),
                ),
                ExplorerMenuAction::DeleteEntry => (
                    if dialog.target_is_dir {
                        " Hapus Folder"
                    } else {
                        " Hapus File"
                    },
                    Style::default().fg(Color::White).bg(Color::Red),
                ),
            };
            Line::styled(label, style)
        })
        .collect::<Vec<_>>();
    frame.render_widget(Paragraph::new(lines), content);

    let mut hit = ExplorerContextMenuHit::default();
    for (idx, action) in dialog.actions.iter().enumerate() {
        let rect = Rect::new(content.x, content.y + idx as u16, content.width, 1);
        match action {
            ExplorerMenuAction::AddFolder => hit.add_folder_rect = Some(rect),
            ExplorerMenuAction::AddFile => hit.add_file_rect = Some(rect),
            ExplorerMenuAction::RenameEntry => hit.rename_entry_rect = Some(rect),
            ExplorerMenuAction::DeleteEntry => hit.delete_entry_rect = Some(rect),
        }
    }

    hit
}

fn menu_rect(bounds: Rect, x: u16, y: u16, w: u16, h: u16) -> Rect {
    let width = w.min(bounds.width.saturating_sub(1)).max(16);
    let height = h.min(bounds.height.saturating_sub(1)).max(4);
    let max_x = bounds.x + bounds.width.saturating_sub(width);
    let max_y = bounds.y + bounds.height.saturating_sub(height);
    Rect::new(x.min(max_x), y.min(max_y), width, height)
}
