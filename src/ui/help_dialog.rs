use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};
use ratatui::Frame;

pub fn render_help_dialog(frame: &mut Frame) {
    let area = centered_rect(frame.area(), 70, 12);
    frame.render_widget(Clear, area);
    frame.render_widget(
        Block::default().title(" Bantuan ").borders(Borders::ALL),
        area,
    );

    let inner = Layout::vertical([Constraint::Min(1)]).margin(1).split(area);
    let text = [
        "Ctrl+N: tab baru",
        "Ctrl+W: tutup tab aktif",
        "Ctrl+B: toggle file tree",
        "Ctrl+T: ganti tema gelap/terang",
        "Ctrl+Tab / Shift+Tab: pindah tab",
        "Tab: pindah fokus editor/file tree/tabs",
        "Saat fokus tabs: Left/Right pindah tab, Enter ke editor",
        "Ctrl+S: simpan file",
        "Ctrl+F: search keyword",
        "Ctrl+Shift+F: format Rust aktif",
        "Ctrl+K: buka Rust/Cargo manager",
        "Ctrl+A: select all (block text)",
        "Shift+Arrow / drag mouse: block text",
        "Alt+Up / Alt+Down: pindah blok/baris",
        "Ctrl+C/V/X: copy/paste/cut (selection atau baris aktif)",
        "Ctrl+Z: undo, Ctrl+Y atau Ctrl+Shift+Z: redo",
        "Editor: smart word wrap aktif untuk baris panjang",
        "F1: buka bantuan",
        "Ctrl+M: buka start menu",
        "Klik kanan di editor: menu Select All, Copy, Paste",
        "Klik kanan di explorer: tambah folder/file, ubah nama, hapus file/folder",
        "Mouse: klik tab, tree, editor, popup simpan",
        "Esc: keluar dialog / keluar aplikasi",
        "@magercode - 2026",
        "Github: github.com/magercode/action-codex",
    ]
    .join("\n");
    frame.render_widget(Paragraph::new(text), inner[0]);
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
