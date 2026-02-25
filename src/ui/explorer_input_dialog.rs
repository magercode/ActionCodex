use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};
use ratatui::Frame;

use super::types::{DialogButton, ExplorerInputDialog, ExplorerInputDialogHit, ExplorerInputMode};

pub fn render_explorer_input_dialog(
    frame: &mut Frame,
    dialog: &ExplorerInputDialog,
) -> ExplorerInputDialogHit {
    let area = centered_rect(frame.area(), 62, 11);
    frame.render_widget(Clear, area);
    frame.render_widget(
        Block::default().title(dialog.title()).borders(Borders::ALL),
        area,
    );

    let inner = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(3),
        Constraint::Length(1),
        Constraint::Length(1),
    ])
    .margin(1)
    .split(area);

    frame.render_widget(
        Paragraph::new(format!("Target: {}", dialog.base_dir.to_string_lossy())),
        inner[0],
    );
    frame.render_widget(
        Paragraph::new("Masukkan nama (boleh subpath), Enter: simpan, Esc: tutup"),
        inner[1],
    );

    let input_title = match dialog.mode {
        ExplorerInputMode::AddFolder => " Folder ",
        ExplorerInputMode::AddFile => " File ",
        ExplorerInputMode::RenameEntry => " Nama Baru ",
    };
    frame.render_widget(
        Paragraph::new(dialog.input.as_str())
            .block(Block::default().title(input_title).borders(Borders::ALL)),
        inner[2],
    );

    let save_label = match dialog.mode {
        ExplorerInputMode::AddFolder => "[ Buat Folder ]",
        ExplorerInputMode::AddFile => "[ Buat File ]",
        ExplorerInputMode::RenameEntry => "[ Simpan Nama ]",
    };
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
        Span::styled(save_label, save_style),
        Span::raw("  "),
        Span::styled("[ Batal ]", cancel_style),
    ]);
    frame.render_widget(Paragraph::new(buttons), inner[3]);
    frame.render_widget(
        Paragraph::new("Tab/Left/Right: pilih tombol | Enter: konfirmasi"),
        inner[4],
    );

    let input_width = inner[2].width.saturating_sub(2) as usize;
    let cursor_x = inner[2].x + 1 + dialog.input.len().min(input_width.saturating_sub(1)) as u16;
    let cursor_y = inner[2].y + 1;
    frame.set_cursor_position((cursor_x, cursor_y));

    let save_width = save_label.len() as u16;
    let buttons_x = inner[3].x;
    let buttons_y = inner[3].y;
    ExplorerInputDialogHit {
        input_rect: inner[2],
        save_button_rect: Rect::new(buttons_x, buttons_y, save_width, 1),
        cancel_button_rect: Rect::new(buttons_x + save_width + 2, buttons_y, 9, 1),
    }
}

fn centered_rect(area: Rect, width_percent: u16, height: u16) -> Rect {
    let max_width = area.width.saturating_sub(2).max(1);
    let max_height = area.height.saturating_sub(2).max(1);
    let width = area.width.saturating_mul(width_percent) / 100;
    let width = width.max(24).min(max_width);
    let height = height.max(7).min(max_height);
    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;
    Rect::new(x, y, width, height)
}
