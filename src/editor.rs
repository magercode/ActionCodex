use crate::syntax::indent::next_indent_for_rust;

const HISTORY_LIMIT: usize = 200;

type Cursor = (usize, usize);

#[derive(Debug, Clone)]
struct EditorSnapshot {
    lines: Vec<String>,
    cursor_x: usize,
    cursor_y: usize,
    scroll_y: usize,
    selection_anchor: Option<Cursor>,
}

#[derive(Debug, Default)]
pub struct Editor {
    lines: Vec<String>,
    cursor_x: usize,
    cursor_y: usize,
    scroll_y: usize,
    selection_anchor: Option<Cursor>,
    undo_stack: Vec<EditorSnapshot>,
    redo_stack: Vec<EditorSnapshot>,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            lines: vec![String::new()],
            ..Self::default()
        }
    }

    pub fn from_text(text: &str) -> Self {
        let mut lines = text.split('\n').map(str::to_string).collect::<Vec<_>>();
        if lines.is_empty() {
            lines.push(String::new());
        }
        Self {
            lines,
            ..Self::default()
        }
    }

    pub fn type_char_smart(&mut self, ch: char) {
        if self.has_selection() {
            self.begin_edit();
            self.delete_selection_if_any();
            self.insert_char_raw(ch);
            return;
        }

        if let Some(next) = self.char_at_cursor() {
            if is_closing_pair(ch) && next == ch {
                self.move_right_raw();
                return;
            }
        }

        self.begin_edit();

        if ch == '}' && self.only_whitespace_before_cursor() {
            self.try_outdent_before_closing();
        }

        if let Some(close) = pair_for_open(ch) {
            if self.should_autopair(ch) {
                self.insert_pair(ch, close);
                return;
            }
        }

        self.insert_char_raw(ch);
    }

    pub fn insert_text(&mut self, text: &str) {
        if text.is_empty() {
            return;
        }

        self.begin_edit();
        self.delete_selection_if_any();
        for ch in text.chars() {
            if ch == '\n' {
                self.insert_newline_raw();
            } else {
                self.insert_char_raw(ch);
            }
        }
    }

    pub fn insert_newline_smart(&mut self) {
        self.begin_edit();
        self.delete_selection_if_any();

        let current_line = self.current_line_text();
        let before = self.current_line_before_cursor();
        let after = current_line
            .chars()
            .skip(before.chars().count())
            .collect::<String>();

        let before_trimmed = before.trim_end();
        let after_trimmed = after.trim_start();
        let between_braces = (before_trimmed.ends_with('{') && after_trimmed.starts_with('}'))
            || (before_trimmed.ends_with('[') && after_trimmed.starts_with(']'))
            || (before_trimmed.ends_with('(') && after_trimmed.starts_with(')'));

        if between_braces {
            let base_indent = leading_ws(&before).to_string();
            let inner_indent = format!("{}    ", base_indent);
            if let Some(line) = self.lines.get_mut(self.cursor_y) {
                *line = before;
            }
            self.cursor_y += 1;
            self.lines.insert(self.cursor_y, inner_indent.clone());
            self.lines
                .insert(self.cursor_y + 1, format!("{}{}", base_indent, after_trimmed));
            self.cursor_x = inner_indent.len();
            return;
        }

        let next_indent = next_indent_for_rust(&before, &after);
        self.insert_newline_with_indent_raw(&next_indent);
    }

    pub fn backspace(&mut self) {
        if self.has_selection() {
            self.begin_edit();
            self.delete_selection_if_any();
            return;
        }

        if self.cursor_x == 0 && self.cursor_y == 0 {
            return;
        }

        self.begin_edit();
        if self.cursor_x > 0 {
            if let Some(line) = self.lines.get_mut(self.cursor_y) {
                self.cursor_x -= 1;
                line.remove(self.cursor_x);
            }
            return;
        }

        let current = self.lines.remove(self.cursor_y);
        self.cursor_y -= 1;
        self.cursor_x = self.lines[self.cursor_y].len();
        self.lines[self.cursor_y].push_str(&current);
    }

    pub fn move_left_selecting(&mut self, selecting: bool) {
        self.prepare_selection(selecting);
        self.move_left_raw();
    }

    pub fn move_right_selecting(&mut self, selecting: bool) {
        self.prepare_selection(selecting);
        self.move_right_raw();
    }

    pub fn move_up_selecting(&mut self, selecting: bool) {
        self.prepare_selection(selecting);
        self.move_up_raw();
    }

    pub fn move_down_selecting(&mut self, selecting: bool) {
        self.prepare_selection(selecting);
        self.move_down_raw();
    }

    pub fn ensure_cursor_visible(&mut self, viewport_height: usize) {
        if self.cursor_y < self.scroll_y {
            self.scroll_y = self.cursor_y;
        } else if self.cursor_y >= self.scroll_y + viewport_height {
            self.scroll_y = self.cursor_y + 1 - viewport_height;
        }
    }

    pub fn set_cursor_from_screen_selecting(
        &mut self,
        screen_x: usize,
        screen_y: usize,
        selecting: bool,
    ) {
        self.prepare_selection(selecting);
        let max_row = self.lines.len().saturating_sub(1);
        let row = (self.scroll_y + screen_y).min(max_row);
        let col = screen_x.min(self.lines[row].len());
        self.cursor_y = row;
        self.cursor_x = col;
    }

    pub fn set_cursor(&mut self, row: usize, col: usize) {
        self.set_cursor_selecting(row, col, false);
    }

    pub fn set_cursor_selecting(&mut self, row: usize, col: usize, selecting: bool) {
        self.prepare_selection(selecting);
        let max_row = self.lines.len().saturating_sub(1);
        let target_row = row.min(max_row);
        let target_col = col.min(self.lines[target_row].len());
        self.cursor_y = target_row;
        self.cursor_x = target_col;
    }

    pub fn lines(&self) -> &[String] {
        &self.lines
    }

    pub fn text(&self) -> String {
        self.lines.join("\n")
    }

    pub fn replace_text(&mut self, text: &str) {
        self.begin_edit();
        let mut lines = text.split('\n').map(str::to_string).collect::<Vec<_>>();
        if lines.is_empty() {
            lines.push(String::new());
        }
        self.lines = lines;
        self.cursor_y = self.cursor_y.min(self.lines.len().saturating_sub(1));
        self.cursor_x = self.cursor_x.min(self.lines[self.cursor_y].len());
        self.scroll_y = self.scroll_y.min(self.lines.len().saturating_sub(1));
        self.selection_anchor = None;
    }

    pub fn current_line_text(&self) -> String {
        self.lines.get(self.cursor_y).cloned().unwrap_or_default()
    }

    pub fn current_line_before_cursor(&self) -> String {
        let line = self.lines.get(self.cursor_y).cloned().unwrap_or_default();
        line.chars().take(self.cursor_x).collect()
    }

    pub fn selected_text(&self) -> Option<String> {
        let (start, end) = self.selection_range()?;

        if start.1 == end.1 {
            return self
                .lines
                .get(start.1)
                .and_then(|line| line.get(start.0..end.0))
                .map(ToOwned::to_owned);
        }

        let mut out = String::new();
        for row in start.1..=end.1 {
            let line = &self.lines[row];
            if row == start.1 {
                if let Some(slice) = line.get(start.0..) {
                    out.push_str(slice);
                }
            } else if row == end.1 {
                if let Some(slice) = line.get(..end.0) {
                    out.push_str(slice);
                }
            } else {
                out.push_str(line);
            }

            if row < end.1 {
                out.push('\n');
            }
        }

        Some(out)
    }

    pub fn selection_char_count(&self) -> usize {
        self.selected_text().map_or(0, |text| text.chars().count())
    }

    pub fn selection_columns_for_row(&self, row: usize) -> Option<(usize, usize)> {
        let (start, end) = self.selection_range()?;
        if row < start.1 || row > end.1 {
            return None;
        }

        if start.1 == end.1 {
            return Some((start.0, end.0));
        }

        if row == start.1 {
            return Some((start.0, self.lines[row].len()));
        }
        if row == end.1 {
            return Some((0, end.0));
        }
        Some((0, self.lines[row].len()))
    }

    pub fn select_all(&mut self) {
        let last_row = self.lines.len().saturating_sub(1);
        let last_col = self.lines[last_row].len();
        self.selection_anchor = Some((0, 0));
        self.cursor_y = last_row;
        self.cursor_x = last_col;
    }

    pub fn has_selection(&self) -> bool {
        self.selection_range().is_some()
    }

    pub fn selection_range(&self) -> Option<(Cursor, Cursor)> {
        if self.lines.is_empty() {
            return None;
        }

        let anchor = self.normalize_cursor(self.selection_anchor?)?;
        let cursor = self.normalize_cursor(self.cursor())?;
        if anchor == cursor {
            return None;
        }

        if (anchor.1, anchor.0) <= (cursor.1, cursor.0) {
            Some((anchor, cursor))
        } else {
            Some((cursor, anchor))
        }
    }

    pub fn cut_current_line(&mut self) -> String {
        if self.has_selection() {
            self.begin_edit();
            return self.delete_selection_if_any().unwrap_or_default();
        }

        if self.lines.is_empty() {
            return String::new();
        }

        self.begin_edit();
        let removed = self.lines.remove(self.cursor_y);
        if self.lines.is_empty() {
            self.lines.push(String::new());
            self.cursor_y = 0;
            self.cursor_x = 0;
            self.scroll_y = 0;
            self.selection_anchor = None;
            return removed;
        }

        if self.cursor_y >= self.lines.len() {
            self.cursor_y = self.lines.len() - 1;
        }
        self.cursor_x = self.cursor_x.min(self.lines[self.cursor_y].len());
        self.selection_anchor = None;
        removed
    }

    pub fn move_selected_lines_up(&mut self) -> bool {
        if self.lines.len() <= 1 {
            return false;
        }

        let (start_row, end_row) = self.movable_line_range();
        if start_row == 0 {
            return false;
        }

        self.begin_edit();
        let block = self
            .lines
            .drain(start_row..=end_row)
            .collect::<Vec<_>>();
        let insert_at = start_row - 1;
        for (idx, line) in block.into_iter().enumerate() {
            self.lines.insert(insert_at + idx, line);
        }

        self.shift_cursor_row(-1);
        self.shift_selection_anchor_row(-1);
        true
    }

    pub fn move_selected_lines_down(&mut self) -> bool {
        if self.lines.len() <= 1 {
            return false;
        }

        let (start_row, end_row) = self.movable_line_range();
        if end_row + 1 >= self.lines.len() {
            return false;
        }

        self.begin_edit();
        let block = self
            .lines
            .drain(start_row..=end_row)
            .collect::<Vec<_>>();
        let insert_at = start_row + 1;
        for (idx, line) in block.into_iter().enumerate() {
            self.lines.insert(insert_at + idx, line);
        }

        self.shift_cursor_row(1);
        self.shift_selection_anchor_row(1);
        true
    }

    pub fn undo(&mut self) -> bool {
        let Some(snapshot) = self.undo_stack.pop() else {
            return false;
        };
        self.redo_stack.push(self.snapshot());
        self.apply_snapshot(snapshot);
        true
    }

    pub fn redo(&mut self) -> bool {
        let Some(snapshot) = self.redo_stack.pop() else {
            return false;
        };
        self.undo_stack.push(self.snapshot());
        self.apply_snapshot(snapshot);
        true
    }

    pub fn cursor_x(&self) -> usize {
        self.cursor_x
    }

    pub fn cursor_y(&self) -> usize {
        self.cursor_y
    }

    pub fn scroll_y(&self) -> usize {
        self.scroll_y
    }

    fn cursor(&self) -> Cursor {
        (self.cursor_x, self.cursor_y)
    }

    fn normalize_cursor(&self, cursor: Cursor) -> Option<Cursor> {
        if self.lines.is_empty() {
            return None;
        }
        let row = cursor.1.min(self.lines.len().saturating_sub(1));
        let col = cursor.0.min(self.lines[row].len());
        Some((col, row))
    }

    fn movable_line_range(&self) -> (usize, usize) {
        if let Some((start, end)) = self.selection_range() {
            let mut end_row = end.1;
            if end.0 == 0 && end.1 > start.1 {
                end_row = end_row.saturating_sub(1);
            }
            (start.1, end_row.max(start.1))
        } else {
            (self.cursor_y, self.cursor_y)
        }
    }

    fn shift_cursor_row(&mut self, delta: isize) {
        self.cursor_y = shift_row(self.cursor_y, delta, self.lines.len());
        self.cursor_x = self.cursor_x.min(self.lines[self.cursor_y].len());
    }

    fn shift_selection_anchor_row(&mut self, delta: isize) {
        if let Some((x, y)) = self.selection_anchor {
            let row = shift_row(y, delta, self.lines.len());
            let col = x.min(self.lines[row].len());
            self.selection_anchor = Some((col, row));
        }
    }

    fn insert_char_raw(&mut self, ch: char) {
        if let Some(line) = self.lines.get_mut(self.cursor_y) {
            line.insert(self.cursor_x, ch);
            self.cursor_x += 1;
        }
    }

    fn insert_newline_raw(&mut self) {
        if let Some(line) = self.lines.get_mut(self.cursor_y) {
            let tail = line.split_off(self.cursor_x);
            self.cursor_y += 1;
            self.cursor_x = 0;
            self.lines.insert(self.cursor_y, tail);
        }
    }

    fn insert_newline_with_indent_raw(&mut self, indent: &str) {
        if let Some(line) = self.lines.get_mut(self.cursor_y) {
            let tail = line.split_off(self.cursor_x);
            self.cursor_y += 1;
            self.lines.insert(self.cursor_y, format!("{}{}", indent, tail));
            self.cursor_x = indent.len();
        }
    }

    fn move_left_raw(&mut self) {
        if self.cursor_x > 0 {
            self.cursor_x -= 1;
        } else if self.cursor_y > 0 {
            self.cursor_y -= 1;
            self.cursor_x = self.lines[self.cursor_y].len();
        }
    }

    fn move_right_raw(&mut self) {
        let current_len = self.lines[self.cursor_y].len();
        if self.cursor_x < current_len {
            self.cursor_x += 1;
        } else if self.cursor_y + 1 < self.lines.len() {
            self.cursor_y += 1;
            self.cursor_x = 0;
        }
    }

    fn move_up_raw(&mut self) {
        if self.cursor_y > 0 {
            self.cursor_y -= 1;
            self.cursor_x = self.cursor_x.min(self.lines[self.cursor_y].len());
        }
    }

    fn move_down_raw(&mut self) {
        if self.cursor_y + 1 < self.lines.len() {
            self.cursor_y += 1;
            self.cursor_x = self.cursor_x.min(self.lines[self.cursor_y].len());
        }
    }

    fn snapshot(&self) -> EditorSnapshot {
        EditorSnapshot {
            lines: self.lines.clone(),
            cursor_x: self.cursor_x,
            cursor_y: self.cursor_y,
            scroll_y: self.scroll_y,
            selection_anchor: self.selection_anchor,
        }
    }

    fn apply_snapshot(&mut self, snapshot: EditorSnapshot) {
        self.lines = snapshot.lines;
        if self.lines.is_empty() {
            self.lines.push(String::new());
        }

        self.cursor_y = snapshot.cursor_y.min(self.lines.len().saturating_sub(1));
        self.cursor_x = snapshot.cursor_x.min(self.lines[self.cursor_y].len());
        self.scroll_y = snapshot.scroll_y.min(self.lines.len().saturating_sub(1));
        self.selection_anchor = snapshot.selection_anchor.map(|(x, y)| {
            let row = y.min(self.lines.len().saturating_sub(1));
            let col = x.min(self.lines[row].len());
            (col, row)
        });
    }

    fn begin_edit(&mut self) {
        self.undo_stack.push(self.snapshot());
        if self.undo_stack.len() > HISTORY_LIMIT {
            self.undo_stack.remove(0);
        }
        self.redo_stack.clear();
    }

    fn prepare_selection(&mut self, selecting: bool) {
        if selecting {
            if self.selection_anchor.is_none() {
                self.selection_anchor = Some(self.cursor());
            }
        } else {
            self.selection_anchor = None;
        }
    }

    fn delete_selection_if_any(&mut self) -> Option<String> {
        let (start, end) = self.selection_range()?;

        let removed = if start.1 == end.1 {
            let line = &mut self.lines[start.1];
            line.drain(start.0..end.0).collect::<String>()
        } else {
            let mut out = String::new();

            let first_line_tail = self.lines[start.1]
                .get(start.0..)
                .unwrap_or_default()
                .to_string();
            out.push_str(&first_line_tail);
            out.push('\n');

            for row in (start.1 + 1)..end.1 {
                out.push_str(&self.lines[row]);
                out.push('\n');
            }

            let last_line_head = self.lines[end.1].get(..end.0).unwrap_or_default().to_string();
            out.push_str(&last_line_head);

            let prefix = self.lines[start.1]
                .get(..start.0)
                .unwrap_or_default()
                .to_string();
            let suffix = self.lines[end.1].get(end.0..).unwrap_or_default().to_string();
            self.lines[start.1] = format!("{}{}", prefix, suffix);
            self.lines.drain((start.1 + 1)..=end.1);

            out
        };

        self.cursor_x = start.0;
        self.cursor_y = start.1;
        self.selection_anchor = None;

        Some(removed)
    }

    fn char_at_cursor(&self) -> Option<char> {
        self.lines
            .get(self.cursor_y)
            .and_then(|line| line.get(self.cursor_x..))
            .and_then(|s| s.chars().next())
    }

    fn char_before_cursor(&self) -> Option<char> {
        self.lines
            .get(self.cursor_y)
            .and_then(|line| line.get(..self.cursor_x))
            .and_then(|s| s.chars().last())
    }

    fn insert_pair(&mut self, open: char, close: char) {
        if let Some(line) = self.lines.get_mut(self.cursor_y) {
            line.insert(self.cursor_x, open);
            line.insert(self.cursor_x + 1, close);
            self.cursor_x += 1;
        }
    }

    fn should_autopair(&self, ch: char) -> bool {
        let next = self.char_at_cursor();
        let prev = self.char_before_cursor();
        let next_ok = next.is_none_or(|c| c.is_whitespace() || ")]}>:;,.\"'".contains(c));

        match ch {
            '"' | '\'' => {
                let prev_blocked = prev.is_some_and(|c| c.is_ascii_alphanumeric() || c == '\\');
                !prev_blocked && next_ok
            }
            '(' | '[' | '{' => next_ok,
            _ => false,
        }
    }

    fn only_whitespace_before_cursor(&self) -> bool {
        self.lines
            .get(self.cursor_y)
            .and_then(|line| line.get(..self.cursor_x))
            .is_some_and(|prefix| prefix.chars().all(char::is_whitespace))
    }

    fn try_outdent_before_closing(&mut self) {
        let remove = self.cursor_x.min(4);
        if remove == 0 {
            return;
        }
        if let Some(line) = self.lines.get_mut(self.cursor_y) {
            let start = self.cursor_x - remove;
            if let Some(slice) = line.get(start..self.cursor_x) {
                if slice.chars().all(|c| c == ' ') {
                    line.drain(start..self.cursor_x);
                    self.cursor_x = start;
                }
            }
        }
    }
}

fn pair_for_open(ch: char) -> Option<char> {
    match ch {
        '(' => Some(')'),
        '[' => Some(']'),
        '{' => Some('}'),
        '"' => Some('"'),
        '\'' => Some('\''),
        _ => None,
    }
}

fn is_closing_pair(ch: char) -> bool {
    matches!(ch, ')' | ']' | '}' | '"' | '\'')
}

fn leading_ws(s: &str) -> &str {
    let idx = s
        .char_indices()
        .find_map(|(i, c)| if c.is_whitespace() { None } else { Some(i) })
        .unwrap_or(s.len());
    &s[..idx]
}

fn shift_row(row: usize, delta: isize, total_lines: usize) -> usize {
    if total_lines == 0 {
        return 0;
    }
    if delta.is_negative() {
        row.saturating_sub(delta.unsigned_abs())
    } else {
        row.saturating_add(delta as usize)
            .min(total_lines.saturating_sub(1))
    }
}
