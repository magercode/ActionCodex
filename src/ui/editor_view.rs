use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{
    Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, Wrap,
};
use ratatui::Frame;

use crate::editor::Editor;
use crate::file_tree::FileTree;
use crate::syntax::highlight::{highlight_rust_document, SyntaxPalette};
use crate::tab_manager::TabManager;
use crate::ui::gutter::build_gutter_lines;

use super::FocusPane;
use crate::ui::theme::ThemeMode;
use crate::ui::types::TabHit;

pub struct EditorRenderParts {
    pub editor_content_rect: Rect,
    pub tree_content_rect: Rect,
    pub tab_content_rect: Rect,
    pub tab_hits: Vec<TabHit>,
    pub editor_height: usize,
    pub text_width: usize,
    pub cursor_visual_x: usize,
    pub cursor_visual_y: usize,
}

pub fn render_editor(
    frame: &mut Frame,
    editor: &Editor,
    status_message: &str,
    tabs: &TabManager,
    file_tree: &FileTree,
    focus: FocusPane,
    show_file_tree: bool,
    search_keyword: Option<&str>,
    theme: ThemeMode,
) -> EditorRenderParts {
    let palette = theme.palette();
    let syntax_palette = if theme == ThemeMode::Dark {
        SyntaxPalette::dark()
    } else {
        SyntaxPalette::light()
    };
    let chunks = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(1),
        Constraint::Length(1),
    ])
    .margin(1)
    .split(frame.area());

    let tab_area = chunks[0];
    let tab_content_rect = Rect::new(
        tab_area.x + 1,
        tab_area.y + 1,
        tab_area.width.saturating_sub(2),
        tab_area.height.saturating_sub(2),
    );
    let tab_title = if focus == FocusPane::Tabs {
        " Tabs [FOCUS] "
    } else {
        " Tabs "
    };
    let tab_block = Block::default().borders(Borders::ALL).title(tab_title);
    frame.render_widget(tab_block, tab_area);

    let mut tab_spans = Vec::new();
    let mut tab_hits = Vec::new();
    let labels = tabs
        .tabs()
        .iter()
        .map(|tab| {
            if tab.dirty() {
                format!(" {}* ", tab.title())
            } else {
                format!(" {} ", tab.title())
            }
        })
        .collect::<Vec<_>>();
    let (start, end) = visible_tab_window(&labels, tabs.active_index(), tab_content_rect.width);

    let mut x = tab_content_rect.x;
    if start > 0 {
        tab_spans.push(Span::styled("<", Style::default().fg(palette.warning)));
        x = x.saturating_add(1);
        if x < tab_content_rect.x + tab_content_rect.width {
            tab_spans.push(Span::raw(" "));
            x = x.saturating_add(1);
        }
    }

    for idx in start..end {
        if x >= tab_content_rect.x + tab_content_rect.width {
            break;
        }
        let remaining = (tab_content_rect.x + tab_content_rect.width).saturating_sub(x);
        if remaining == 0 {
            break;
        }
        let mut label = labels[idx].clone();
        if label.chars().count() as u16 > remaining {
            label = truncate_plain_with_tilde(&label, remaining as usize);
        }
        let label_width = label.chars().count() as u16;
        let style = if idx == tabs.active_index() {
            Style::default().fg(palette.accent_text).bg(palette.accent)
        } else {
            Style::default()
        };
        tab_spans.push(Span::styled(label, style));
        tab_hits.push(TabHit {
            index: idx,
            rect: Rect::new(x, tab_content_rect.y, label_width, 1),
        });
        x = x.saturating_add(label_width);

        if idx + 1 < end && x < tab_content_rect.x + tab_content_rect.width {
            tab_spans.push(Span::raw("|"));
            x = x.saturating_add(1);
        }
    }

    if end < labels.len() && x < tab_content_rect.x + tab_content_rect.width {
        if x + 1 < tab_content_rect.x + tab_content_rect.width {
            tab_spans.push(Span::raw(" "));
            x = x.saturating_add(1);
        }
        if x < tab_content_rect.x + tab_content_rect.width {
            tab_spans.push(Span::styled(">", Style::default().fg(palette.warning)));
        }
    }
    frame.render_widget(Paragraph::new(Line::from(tab_spans)), tab_content_rect);

    let body = if show_file_tree {
        Layout::horizontal([Constraint::Length(34), Constraint::Min(20)]).split(chunks[1])
    } else {
        Layout::horizontal([Constraint::Length(0), Constraint::Min(20)]).split(chunks[1])
    };
    let tree_area = body[0];
    let editor_area = body[1];
    let tree_content_rect = if show_file_tree {
        Rect::new(
            tree_area.x + 1,
            tree_area.y + 1,
            tree_area.width.saturating_sub(2),
            tree_area.height.saturating_sub(2),
        )
    } else {
        Rect::new(0, 0, 0, 0)
    };
    let editor_content_rect = Rect::new(
        editor_area.x + 1,
        editor_area.y + 1,
        editor_area.width.saturating_sub(2),
        editor_area.height.saturating_sub(2),
    );

    let editor_height = editor_area.height.saturating_sub(2) as usize;
    let editor_width = editor_area.width.saturating_sub(2);
    let gutter_digits = editor.lines().len().max(1).to_string().len() as u16;
    let gutter_width = (gutter_digits + 1)
        .min(editor_width.saturating_sub(3))
        .max(2);
    let editor_inner = Layout::horizontal([
        Constraint::Length(gutter_width),
        Constraint::Length(1),
        Constraint::Min(1),
    ])
    .split(editor_content_rect);
    let gutter_rect = editor_inner[0];
    let code_rect = editor_inner[2];
    let text_width = code_rect.width.saturating_sub(1).max(1) as usize;

    if show_file_tree {
        let tree_height = tree_area.height.saturating_sub(2) as usize;
        let tree_width = tree_content_rect.width.max(1) as usize;
        let tree_lines = file_tree
            .entries()
            .iter()
            .enumerate()
            .skip(file_tree.scroll())
            .take(tree_height.max(1))
            .map(|(idx, entry)| {
                let indent = "  ".repeat(entry.depth());
                let marker = if entry.is_dir() { ">" } else { "-" };
                let line_text = truncate_plain_with_tilde(
                    &format!("{}{} {}", indent, marker, entry.name()),
                    tree_width,
                );
                if idx == file_tree.selected_index() {
                    if focus == FocusPane::FileTree {
                        Line::styled(
                            line_text,
                            Style::default()
                                .fg(palette.warning_text)
                                .bg(palette.warning),
                        )
                    } else {
                        Line::styled(line_text, Style::default().bg(palette.tree_inactive_bg))
                    }
                } else {
                    Line::from(line_text)
                }
            })
            .collect::<Vec<_>>();
        let tree_title = if focus == FocusPane::FileTree {
            " Explorer [FOCUS] "
        } else {
            " Explorer "
        };
        let tree_widget = Paragraph::new(tree_lines)
            .block(Block::default().title(tree_title).borders(Borders::ALL));
        frame.render_widget(tree_widget, tree_area);
    }

    let highlighted_document = highlight_rust_document(editor.lines(), &syntax_palette);
    let mut lines = Vec::new();
    let mut cursor_visual = None;
    let visible_rows = editor_height.max(1);
    let mut rendered_rows = 0usize;
    for row in editor.scroll_y()..editor.lines().len() {
        if rendered_rows >= visible_rows {
            break;
        }
        let source_line = &editor.lines()[row];
        let mut highlighted = highlighted_document
            .get(row)
            .cloned()
            .unwrap_or_else(|| Line::from(source_line.clone()));
        if let Some(keyword) = search_keyword {
            highlighted =
                apply_search_background(highlighted, source_line, keyword, palette.search_bg);
        }
        if let Some((start, end)) = editor.selection_columns_for_row(row) {
            highlighted = apply_selection_background(highlighted, start, end, palette.selection_bg);
        }

        let wrapped = wrap_line_smart(highlighted, text_width.max(1));
        let mut cursor_segment_index = None;
        let mut cursor_x_in_segment = 0usize;
        if row == editor.cursor_y() {
            let (segment_index, segment_x) = cursor_in_wrapped_line(editor.cursor_x(), &wrapped);
            cursor_segment_index = Some(segment_index);
            cursor_x_in_segment = segment_x;
        }

        for (segment_idx, segment) in wrapped.into_iter().enumerate() {
            if rendered_rows >= visible_rows {
                break;
            }
            if Some(segment_idx) == cursor_segment_index {
                cursor_visual = Some((cursor_x_in_segment, rendered_rows));
            }
            lines.push(segment.line);
            rendered_rows += 1;
        }
    }

    if lines.is_empty() {
        lines.push(Line::from(String::new()));
    }

    let (cursor_visual_x, cursor_visual_y) = cursor_visual.unwrap_or_else(|| {
        (
            editor.cursor_x().min(text_width.saturating_sub(1)),
            editor
                .cursor_y()
                .saturating_sub(editor.scroll_y())
                .min(editor_height.saturating_sub(1)),
        )
    });

    let selection_indicator = if editor.has_selection() {
        format!(" [BLOCK {}] ", editor.selection_char_count())
    } else {
        String::new()
    };
    let editor_block = Block::default()
        .title(format!(
            " {}{} ",
            tabs.active_tab_title(),
            selection_indicator
        ))
        .borders(Borders::ALL);
    frame.render_widget(editor_block, editor_area);

    let gutter = build_gutter_lines(
        editor.lines().len(),
        editor.scroll_y(),
        editor_height.max(1),
        editor.cursor_y(),
        theme,
    );
    frame.render_widget(Paragraph::new(gutter), gutter_rect);

    // Vertical separator between gutter and code area.
    let separator = Rect::new(
        gutter_rect.x + gutter_rect.width,
        gutter_rect.y,
        1,
        gutter_rect.height,
    );
    frame.render_widget(
        Paragraph::new("│".repeat(gutter_rect.height as usize)),
        separator,
    );

    let text_editor = Paragraph::new(lines).wrap(Wrap { trim: false });
    frame.render_widget(text_editor, code_rect);

    let mut scrollbar_state = ScrollbarState::new(editor.lines().len())
        .viewport_content_length(editor_height.max(1))
        .position(editor.scroll_y());
    let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight);
    frame.render_stateful_widget(scrollbar, editor_area, &mut scrollbar_state);

    let selection_status = if editor.has_selection() {
        format!(" | block {} karakter", editor.selection_char_count())
    } else {
        String::new()
    };
    let status = format!(
        "{}{} | baris {}, kolom {}",
        status_message,
        selection_status,
        editor.cursor_y() + 1,
        editor.cursor_x() + 1
    );
    let status_bar =
        Paragraph::new(status).style(Style::default().fg(palette.status_fg).bg(palette.status_bg));
    frame.render_widget(status_bar, chunks[2]);

    EditorRenderParts {
        editor_content_rect: code_rect,
        tree_content_rect,
        tab_content_rect,
        tab_hits,
        editor_height,
        text_width,
        cursor_visual_x,
        cursor_visual_y,
    }
}

pub fn render_editor_cursor(frame: &mut Frame, _editor: &Editor, parts: &EditorRenderParts) {
    let visible_cursor_x = parts
        .cursor_visual_x
        .min(parts.text_width.saturating_sub(1)) as u16;
    let visible_cursor_y = parts
        .cursor_visual_y
        .min(parts.editor_height.saturating_sub(1)) as u16;
    let cursor_x = parts.editor_content_rect.x + visible_cursor_x;
    let cursor_y = parts.editor_content_rect.y + visible_cursor_y;
    frame.set_cursor_position((cursor_x, cursor_y));
}

fn visible_tab_window(labels: &[String], active: usize, max_width: u16) -> (usize, usize) {
    if labels.is_empty() || max_width == 0 {
        return (0, 0);
    }

    let mut start = active.min(labels.len() - 1);
    let mut end = start + 1;

    loop {
        let mut progressed = false;

        if end < labels.len() {
            let next_end = end + 1;
            if tab_window_width(labels, start, next_end) <= max_width {
                end = next_end;
                progressed = true;
            }
        }

        if start > 0 {
            let next_start = start - 1;
            if tab_window_width(labels, next_start, end) <= max_width {
                start = next_start;
                progressed = true;
            }
        }

        if !progressed {
            break;
        }
    }

    while tab_window_width(labels, start, end) > max_width && start + 1 < end {
        start += 1;
    }

    (start, end)
}

fn tab_window_width(labels: &[String], start: usize, end: usize) -> u16 {
    if start >= end {
        return 0;
    }
    let mut width = 0u16;
    if start > 0 {
        width = width.saturating_add(2);
    }
    for idx in start..end {
        width = width.saturating_add(labels[idx].chars().count() as u16);
        if idx + 1 < end {
            width = width.saturating_add(1);
        }
    }
    if end < labels.len() {
        width = width.saturating_add(2);
    }
    width
}

fn apply_selection_background(
    line: Line<'static>,
    start: usize,
    end: usize,
    bg_color: Color,
) -> Line<'static> {
    apply_background_color(line, start, end, bg_color)
}

fn apply_search_background(
    mut line: Line<'static>,
    source_line: &str,
    keyword: &str,
    bg_color: Color,
) -> Line<'static> {
    if keyword.is_empty() {
        return line;
    }

    for (start, _) in source_line.match_indices(keyword) {
        let end = start + keyword.len();
        line = apply_background_color(line, start, end, bg_color);
    }
    line
}

fn apply_background_color(
    line: Line<'static>,
    start: usize,
    end: usize,
    bg_color: Color,
) -> Line<'static> {
    if start >= end {
        return line;
    }

    let mut out = Vec::new();
    let mut offset = 0usize;
    for span in line.spans {
        let text = span.content.to_string();
        let len = text.len();
        let span_start = offset;
        let span_end = offset + len;

        if end <= span_start || start >= span_end {
            out.push(Span::styled(text, span.style));
            offset = span_end;
            continue;
        }

        let local_start = start.saturating_sub(span_start).min(len);
        let local_end = end.saturating_sub(span_start).min(len);

        if local_start > 0 {
            if let Some(prefix) = text.get(..local_start) {
                if !prefix.is_empty() {
                    out.push(Span::styled(prefix.to_string(), span.style));
                }
            }
        }

        if local_end > local_start {
            if let Some(selected) = text.get(local_start..local_end) {
                if !selected.is_empty() {
                    out.push(Span::styled(selected.to_string(), span.style.bg(bg_color)));
                }
            }
        }

        if local_end < len {
            if let Some(suffix) = text.get(local_end..) {
                if !suffix.is_empty() {
                    out.push(Span::styled(suffix.to_string(), span.style));
                }
            }
        }

        offset = span_end;
    }

    Line::from(out)
}

fn truncate_plain_with_tilde(input: &str, width: usize) -> String {
    if width == 0 {
        return String::new();
    }
    let char_count = input.chars().count();
    if char_count <= width {
        return input.to_string();
    }
    if width == 1 {
        return String::from("~");
    }
    let keep = width - 1;
    let prefix = input.chars().take(keep).collect::<String>();
    format!("{}~", prefix)
}

#[derive(Debug)]
struct WrappedLineSegment {
    line: Line<'static>,
    start: usize,
    end: usize,
}

fn wrap_line_smart(line: Line<'static>, width: usize) -> Vec<WrappedLineSegment> {
    let plain = line
        .spans
        .iter()
        .map(|span| span.content.as_ref())
        .collect::<String>();
    if plain.is_empty() {
        return vec![WrappedLineSegment {
            line: Line::from(String::new()),
            start: 0,
            end: 0,
        }];
    }

    let ranges = compute_wrap_ranges(&plain, width.max(1));
    ranges
        .into_iter()
        .map(|(start, end)| WrappedLineSegment {
            line: slice_styled_line(&line, start, end),
            start,
            end,
        })
        .collect::<Vec<_>>()
}

fn cursor_in_wrapped_line(cursor_x: usize, wrapped: &[WrappedLineSegment]) -> (usize, usize) {
    if wrapped.is_empty() {
        return (0, 0);
    }

    for (idx, segment) in wrapped.iter().enumerate() {
        if cursor_x >= segment.start && cursor_x <= segment.end {
            return (idx, cursor_x.saturating_sub(segment.start));
        }
        if cursor_x < segment.start {
            return (idx, 0);
        }
    }

    let last_idx = wrapped.len() - 1;
    let last = &wrapped[last_idx];
    (
        last_idx,
        cursor_x
            .saturating_sub(last.start)
            .min(last.end.saturating_sub(last.start)),
    )
}

fn compute_wrap_ranges(text: &str, width: usize) -> Vec<(usize, usize)> {
    let mut ranges = Vec::new();
    if text.is_empty() {
        ranges.push((0, 0));
        return ranges;
    }

    let mut start = 0usize;
    while start < text.len() {
        let candidate = advance_boundary_by_columns(text, start, width.max(1));
        if candidate >= text.len() {
            ranges.push((start, text.len()));
            break;
        }

        let mut break_at = None;
        for (rel_idx, ch) in text[start..candidate].char_indices() {
            if ch.is_whitespace() {
                break_at = Some(start + rel_idx);
            }
        }

        let (end, next_start) = if let Some(ws_idx) = break_at {
            if ws_idx > start {
                let ws_len = text[ws_idx..]
                    .chars()
                    .next()
                    .map(|ch| ch.len_utf8())
                    .unwrap_or(1);
                let mut next = ws_idx + ws_len;
                while next < text.len() {
                    let ch = text[next..].chars().next();
                    if ch.is_some_and(|value| value.is_whitespace()) {
                        next += ch.map(|value| value.len_utf8()).unwrap_or(1);
                    } else {
                        break;
                    }
                }
                (ws_idx, next)
            } else {
                (candidate, candidate)
            }
        } else {
            (candidate, candidate)
        };

        if end <= start {
            let forced = advance_boundary_by_columns(text, start, 1);
            ranges.push((start, forced));
            start = forced;
        } else {
            ranges.push((start, end));
            start = next_start;
        }
    }

    if ranges.is_empty() {
        ranges.push((0, 0));
    }
    ranges
}

fn advance_boundary_by_columns(text: &str, start: usize, width: usize) -> usize {
    if start >= text.len() || width == 0 {
        return start;
    }

    let mut idx = start;
    let mut consumed = 0usize;
    while idx < text.len() && consumed < width {
        if let Some(ch) = text[idx..].chars().next() {
            idx += ch.len_utf8();
            consumed += 1;
        } else {
            break;
        }
    }
    idx
}

fn slice_styled_line(line: &Line<'static>, start: usize, end: usize) -> Line<'static> {
    if start >= end {
        return Line::from(String::new());
    }

    let mut out = Vec::new();
    let mut offset = 0usize;
    for span in &line.spans {
        let text = span.content.to_string();
        let span_start = offset;
        let span_end = span_start + text.len();
        if end <= span_start || start >= span_end {
            offset = span_end;
            continue;
        }

        let local_start = start.saturating_sub(span_start).min(text.len());
        let local_end = end.saturating_sub(span_start).min(text.len());
        if local_end > local_start {
            if let Some(slice) = text.get(local_start..local_end) {
                if !slice.is_empty() {
                    out.push(Span::styled(slice.to_string(), span.style));
                }
            }
        }
        offset = span_end;
    }

    if out.is_empty() {
        Line::from(String::new())
    } else {
        Line::from(out)
    }
}
