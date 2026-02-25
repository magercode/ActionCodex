use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};
use ratatui::Frame;

use super::SearchDialog;

pub fn render_search_dialog(frame: &mut Frame, dialog: &SearchDialog) {
    let area = centered_rect(frame.area(), 60, 7);
    frame.render_widget(Clear, area);
    frame.render_widget(
        Block::default().title(" Search Keyword ").borders(Borders::ALL),
        area,
    );

    let inner = Layout::vertical([Constraint::Length(1), Constraint::Length(3)])
        .margin(1)
        .split(area);
    
    frame.render_widget(
        Paragraph::new("Masukkan keyword, Enter: cari berikutnya, Esc: tutup"),
        inner[0],
    );

    frame.render_widget(
        Paragraph::new(dialog.query.as_str())
            .block(Block::default().title(" Keyword ").borders(Borders::ALL)),
        inner[1],
    );

    let input_width = inner[1].width.saturating_sub(2) as usize;
    let cursor_x = inner[1].x + 1 + dialog.query.len().min(input_width.saturating_sub(1)) as u16;
    let cursor_y = inner[1].y + 1;
    frame.set_cursor_position((cursor_x, cursor_y));
}

fn centered_rect(area: Rect, width_percent: u16, height: u16) -> Rect {
    let max_width = area.width.saturating_sub(2).max(1);
    let max_height = area.height.saturating_sub(2).max(1);
    let width = area.width.saturating_mul(width_percent) / 100;
    let width = width.max(20).min(max_width);
    let height = height.max(5).min(max_height);
    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;
    Rect::new(x, y, width, height)
}
