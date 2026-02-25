use ratatui::style::Style;
use ratatui::text::Line;

use super::theme::ThemeMode;

pub fn build_gutter_lines(
    total_lines: usize,
    scroll_y: usize,
    editor_height: usize,
    cursor_y: usize,
    theme: ThemeMode,
) -> Vec<Line<'static>> {
    let palette = theme.palette();
    let width = total_lines.max(1).to_string().len();
    let mut lines = Vec::new();
    for row in 0..editor_height.max(1) {
        let line_no = scroll_y + row + 1;
        if line_no > total_lines {
            lines.push(Line::from(" ".repeat(width)));
            continue;
        }
        let text = format!("{:>width$}", line_no, width = width);
        if line_no - 1 == cursor_y {
            lines.push(Line::styled(
                text,
                Style::default().fg(palette.line_current),
            ));
        } else {
            lines.push(Line::styled(text, Style::default().fg(palette.line_other)));
        }
    }
    lines
}
