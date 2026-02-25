use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};
use ratatui::Frame;

use super::types::{DialogButton, SaveDialog, SaveDialogHit};

pub fn render_save_dialog(frame: &mut Frame, dialog: &SaveDialog) -> SaveDialogHit {
    let area = centered_rect(frame.area(), 60, 9);
    frame.render_widget(Clear, area);

    let popup = Block::default().title(" Simpan File ").borders(Borders::ALL);
    frame.render_widget(popup, area);

    let inner = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(3),
        Constraint::Length(1),
        Constraint::Length(1),
    ])
    .margin(1)
    .split(area);

    frame.render_widget(Paragraph::new("Nama file:"), inner[0]);

    let input = Paragraph::new(dialog.filename.as_str())
        .block(Block::default().borders(Borders::ALL).title(" File "));
    frame.render_widget(input, inner[1]);

    let save_style = if dialog.selected == DialogButton::Save {
        Style::default().fg(Color::Black).bg(Color::Green)
    } else {
        Style::default()
    };
    let cancel_style = if dialog.selected == DialogButton::Cancel {
        Style::default().fg(Color::Black).bg(Color::Yellow)
    } else {
        Style::default()
    };

    let buttons = Line::from(vec![
        Span::styled("[ Simpan ]", save_style),
        Span::raw("  "),
        Span::styled("[ Batal ]", cancel_style),
    ]);
    frame.render_widget(Paragraph::new(buttons), inner[2]);
    frame.render_widget(
        Paragraph::new("Tab/Left/Right: pilih tombol | Enter: konfirmasi | Esc: tutup"),
        inner[3],
    );

    let input_width = inner[1].width.saturating_sub(2) as usize;
    let cursor_x = inner[1].x + 1 + dialog.filename.len().min(input_width.saturating_sub(1)) as u16;
    let cursor_y = inner[1].y + 1;
    frame.set_cursor_position((cursor_x, cursor_y));

    let buttons_x = inner[2].x;
    let buttons_y = inner[2].y;
    let save_button_rect = Rect::new(buttons_x, buttons_y, 10, 1);
    let cancel_button_rect = Rect::new(buttons_x + 12, buttons_y, 9, 1);
    SaveDialogHit {
        input_rect: inner[1],
        save_button_rect,
        cancel_button_rect,
    }
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
