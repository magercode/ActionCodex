use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Clear, Paragraph};
use ratatui::Frame;

use super::types::{ContextMenuDialog, ContextMenuHit};

pub fn render_context_menu(frame: &mut Frame, dialog: &ContextMenuDialog) -> ContextMenuHit {
    let area = menu_rect(frame.area(), dialog.x, dialog.y, 20, 5);
    frame.render_widget(Clear, area);
    frame.render_widget(Block::default().borders(Borders::ALL), area);

    let lines = vec![
        Line::styled(
            " Select All",
            Style::default().fg(Color::Black).bg(Color::Cyan),
        ),
        Line::styled(" Copy", Style::default().fg(Color::Black).bg(Color::Green)),
        Line::styled(
            " Paste",
            Style::default().fg(Color::Black).bg(Color::Yellow),
        ),
    ];
    let content = Rect::new(
        area.x + 1,
        area.y + 1,
        area.width.saturating_sub(2),
        area.height.saturating_sub(2),
    );
    frame.render_widget(Paragraph::new(lines), content);

    ContextMenuHit {
        select_all_rect: Rect::new(content.x, content.y, content.width, 1),
        copy_rect: Rect::new(content.x, content.y + 1, content.width, 1),
        paste_rect: Rect::new(content.x, content.y + 2, content.width, 1),
    }
}

fn menu_rect(bounds: Rect, x: u16, y: u16, w: u16, h: u16) -> Rect {
    let width = w.min(bounds.width.saturating_sub(1)).max(10);
    let height = h.min(bounds.height.saturating_sub(1)).max(4);
    let max_x = bounds.x + bounds.width.saturating_sub(width);
    let max_y = bounds.y + bounds.height.saturating_sub(height);
    Rect::new(x.min(max_x), y.min(max_y), width, height)
}
