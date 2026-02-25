use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Clear, Paragraph};
use ratatui::Frame;

use super::{StartMenuAction, StartMenuDialog};

pub fn render_start_menu(frame: &mut Frame, dialog: &StartMenuDialog) {
    let area = centered_rect(frame.area(), 55, 10);
    frame.render_widget(Clear, area);
    frame.render_widget(
        Block::default().title(" Action Codex  1.3").borders(Borders::ALL),
        area,
    );

    let inner = Layout::vertical([Constraint::Length(1), Constraint::Min(1)])
        .margin(1)
        .split(area);

    frame.render_widget(
        Paragraph::new("Up/Down pilih | Enter konfirmasi | Esc tutup"),
        inner[0],
    );

    let lines = StartMenuDialog::actions()
        .iter()
        .enumerate()
        .map(|(idx, action)| {
            let label = match action {
                StartMenuAction::NewTab => "Buat tab baru",
                StartMenuAction::FocusFileTree => "Fokus ke file tree",
                StartMenuAction::OpenManager => "Buka Rust/Cargo manager",
                StartMenuAction::ShowHelp => "Buka bantuan",
                StartMenuAction::ContinueEditor => "Lanjut ke editor",
            };
            if idx == dialog.selected {
                Line::styled(
                    format!("> {}", label),
                    Style::default().fg(Color::Black).bg(Color::Cyan),
                )
            } else {
                Line::from(format!("  {}", label))
            }
        })
        .collect::<Vec<_>>();
    frame.render_widget(Paragraph::new(lines), inner[1]);
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
