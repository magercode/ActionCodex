const INDENT: &str = "    ";

pub fn next_indent_for_rust(line_before_cursor: &str, line_after_cursor: &str) -> String {
    let before = line_before_cursor;
    let after = line_after_cursor;

    let base_indent = leading_ws(before).to_string();
    let before_trimmed = before.trim_end();
    let before_code = before_trimmed.trim_start();
    let after_code = after.trim_start();

    if let Some(comment_indent) = continue_line_comment(&base_indent, before_code) {
        return comment_indent;
    }
    if let Some(comment_indent) = continue_block_comment(&base_indent, before_code) {
        return comment_indent;
    }

    let mut indent_level = base_indent.len() / INDENT.len();

    if starts_with_closing(after_code) && indent_level > 0 {
        indent_level -= 1;
    }

    if opens_block(before_trimmed) || continues_chain(before_trimmed) || has_unclosed_delimiter(before_trimmed)
    {
        indent_level += 1;
    }

    INDENT.repeat(indent_level)
}

fn continue_line_comment(base_indent: &str, before_code: &str) -> Option<String> {
    if before_code.starts_with("///") {
        return Some(format!("{}/// ", base_indent));
    }
    if before_code.starts_with("//!") {
        return Some(format!("{}//! ", base_indent));
    }
    if before_code.starts_with("//") {
        return Some(format!("{}// ", base_indent));
    }
    None
}

fn continue_block_comment(base_indent: &str, before_code: &str) -> Option<String> {
    if before_code.starts_with("/**") {
        return Some(format!("{} * ", base_indent));
    }
    if before_code.starts_with("/*") {
        return Some(format!("{} * ", base_indent));
    }
    if before_code.starts_with('*') {
        return Some(format!("{}* ", base_indent));
    }
    None
}

fn starts_with_closing(text: &str) -> bool {
    text.starts_with('}') || text.starts_with(')') || text.starts_with(']')
}

fn opens_block(before_trimmed: &str) -> bool {
    let code = strip_trailing_comment(before_trimmed).trim_end();
    code.ends_with('{')
        || code.ends_with('(')
        || code.ends_with('[')
        || code.ends_with("=>")
        || code.ends_with("->")
}

fn continues_chain(before_trimmed: &str) -> bool {
    let code = strip_trailing_comment(before_trimmed).trim_end();
    code.ends_with('.') || code.ends_with("::") || code.ends_with(',')
}

fn has_unclosed_delimiter(before_trimmed: &str) -> bool {
    let code = strip_trailing_comment(before_trimmed);
    let mut paren = 0isize;
    let mut bracket = 0isize;
    let mut brace = 0isize;

    for ch in code.chars() {
        match ch {
            '(' => paren += 1,
            ')' => paren -= 1,
            '[' => bracket += 1,
            ']' => bracket -= 1,
            '{' => brace += 1,
            '}' => brace -= 1,
            _ => {}
        }
    }

    paren > 0 || bracket > 0 || brace > 0
}

fn strip_trailing_comment(s: &str) -> &str {
    if let Some(idx) = s.find("//") {
        &s[..idx]
    } else {
        s
    }
}

fn leading_ws(s: &str) -> &str {
    let idx = s
        .char_indices()
        .find_map(|(i, c)| if c.is_whitespace() { None } else { Some(i) })
        .unwrap_or(s.len());
    &s[..idx]
}
