use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};

const RUST_KEYWORDS: &[&str] = &[
    "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum",
    "extern", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut",
    "pub", "ref", "return", "self", "Self", "static", "struct", "super", "trait", "type",
    "unsafe", "use", "where", "while",
];
const RUST_LITERAL_KEYWORDS: &[&str] = &["true", "false"];
const RUST_BUILTIN_TYPES: &[&str] = &[
    "i8", "i16", "i32", "i64", "i128", "isize",
    "u8", "u16", "u32", "u64", "u128", "usize",
    "f32", "f64",
    "bool", "char", "str", "String",
];

#[derive(Debug, Clone, Copy)]
pub struct SyntaxPalette {
    pub plain: Style,
    pub keyword: Style,
    pub string: Style,
    pub number: Style,
    pub comment: Style,
    pub r#type: Style,
    pub function: Style,
    pub r#macro: Style,
    pub lifetime: Style,
    pub operator: Style,
    pub delimiter: Style,
    pub literal: Style,
}

impl SyntaxPalette {
    pub fn dark() -> Self {
        Self {
            plain: Style::default().fg(Color::White),
            keyword: Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            string: Style::default().fg(Color::Green),
            number: Style::default().fg(Color::Magenta),
            comment: Style::default().fg(Color::DarkGray),
            r#type: Style::default().fg(Color::Yellow),
            function: Style::default().fg(Color::LightBlue),
            r#macro: Style::default().fg(Color::Blue),
            lifetime: Style::default().fg(Color::LightMagenta),
            operator: Style::default().fg(Color::Gray),
            delimiter: Style::default().fg(Color::LightCyan),
            literal: Style::default().fg(Color::LightMagenta),
        }
    }

    pub fn light() -> Self {
        Self {
            plain: Style::default().fg(Color::Black),
            keyword: Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD),
            string: Style::default().fg(Color::Green),
            number: Style::default().fg(Color::Magenta),
            comment: Style::default().fg(Color::DarkGray),
            r#type: Style::default().fg(Color::Blue),
            function: Style::default().fg(Color::Cyan),
            r#macro: Style::default().fg(Color::LightBlue),
            lifetime: Style::default().fg(Color::Magenta),
            operator: Style::default().fg(Color::Gray),
            delimiter: Style::default().fg(Color::Blue),
            literal: Style::default().fg(Color::Magenta),
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct ScanState {
    block_comment_depth: usize,
    in_string: Option<StringState>,
}

#[derive(Debug, Clone, Copy)]
enum StringState {
    Standard,
    Raw { hashes: usize },
}

pub fn highlight_rust_document(lines: &[String], palette: &SyntaxPalette) -> Vec<Line<'static>> {
    let mut state = ScanState::default();
    lines
        .iter()
        .map(|line| highlight_rust_line_with_state(line, palette, &mut state))
        .collect()
}

fn highlight_rust_line_with_state(
    line: &str,
    palette: &SyntaxPalette,
    state: &mut ScanState,
) -> Line<'static> {
    if line.is_empty() {
        return Line::from(String::new());
    }

    let bytes = line.as_bytes();
    let mut spans = Vec::new();
    let mut i = 0usize;

    while i < bytes.len() {
        if state.block_comment_depth > 0 {
            let start = i;
            i = scan_block_comment(line, bytes, i, state);
            push_span(&mut spans, line, start, i, palette.comment);
            continue;
        }

        if let Some(string_state) = state.in_string {
            let start = i;
            i = scan_string(line, bytes, i, string_state, state);
            push_span(&mut spans, line, start, i, palette.string);
            continue;
        }

        if is_line_comment_start(bytes, i) {
            push_span(&mut spans, line, i, bytes.len(), palette.comment);
            break;
        }

        if is_block_comment_start(bytes, i) {
            let start = i;
            state.block_comment_depth = 1;
            i += 2;
            i = scan_block_comment(line, bytes, i, state);
            push_span(&mut spans, line, start, i, palette.comment);
            continue;
        }

        if let Some((start, next_i)) = try_start_raw_string(bytes, i) {
            state.in_string = Some(StringState::Raw {
                hashes: next_i.saturating_sub(start + 2),
            });
            i = scan_string(
                line,
                bytes,
                start,
                StringState::Raw {
                    hashes: next_i.saturating_sub(start + 2),
                },
                state,
            );
            push_span(&mut spans, line, start, i, palette.string);
            continue;
        }

        if bytes[i] == b'"' {
            state.in_string = Some(StringState::Standard);
            let start = i;
            i = scan_string(line, bytes, i, StringState::Standard, state);
            push_span(&mut spans, line, start, i, palette.string);
            continue;
        }

        if bytes[i] == b'\'' {
            if let Some((start, end)) = scan_lifetime_or_char(line, i) {
                let style = if is_char_literal_token(&line[start..end]) {
                    palette.literal
                } else {
                    palette.lifetime
                };
                push_span(&mut spans, line, start, end, style);
                i = end;
                continue;
            }
        }

        if bytes[i].is_ascii_digit() {
            let start = i;
            i = scan_number(bytes, i);
            push_span(&mut spans, line, start, i, palette.number);
            continue;
        }

        if is_ident_start(bytes[i]) {
            let start = i;
            i = scan_identifier(bytes, i);
            let token = &line[start..i];

            let style = if RUST_LITERAL_KEYWORDS.contains(&token) {
                palette.literal
            } else if RUST_KEYWORDS.contains(&token) {
                palette.keyword
            } else if RUST_BUILTIN_TYPES.contains(&token) {
                palette.r#type
            } else if looks_like_macro(bytes, i) {
                palette.r#macro
            } else if looks_like_type(token) {
                palette.r#type
            } else if looks_like_function_call(bytes, i) {
                palette.function
            } else {
                palette.plain
            };

            push_span(&mut spans, line, start, i, style);
            continue;
        }

        if is_operator(bytes[i]) {
            push_span(&mut spans, line, i, i + 1, palette.operator);
            i += 1;
            continue;
        }

        if is_delimiter(bytes[i]) {
            push_span(&mut spans, line, i, i + 1, palette.delimiter);
            i += 1;
            continue;
        }

        push_span(&mut spans, line, i, i + 1, palette.plain);
        i += 1;
    }

    Line::from(spans)
}

fn scan_block_comment(line: &str, bytes: &[u8], mut i: usize, state: &mut ScanState) -> usize {
    while i + 1 < bytes.len() {
        if is_block_comment_start(bytes, i) {
            state.block_comment_depth += 1;
            i += 2;
            continue;
        }
        if is_block_comment_end(bytes, i) {
            state.block_comment_depth = state.block_comment_depth.saturating_sub(1);
            i += 2;
            if state.block_comment_depth == 0 {
                return i;
            }
            continue;
        }
        i += 1;
    }

    if i < line.len() {
        line.len()
    } else {
        i
    }
}

fn scan_string(
    line: &str,
    bytes: &[u8],
    mut i: usize,
    mode: StringState,
    state: &mut ScanState,
) -> usize {
    match mode {
        StringState::Standard => {
            if i < bytes.len() {
                i += 1;
            }
            while i < bytes.len() {
                if bytes[i] == b'"' && !is_escaped(bytes, i) {
                    i += 1;
                    state.in_string = None;
                    return i;
                }
                i += 1;
            }
            line.len()
        }
        StringState::Raw { hashes } => {
            if i < bytes.len() {
                i += 1;
            }
            while i < bytes.len() {
                if bytes[i] == b'"' {
                    let mut ok = true;
                    for h in 0..hashes {
                        if i + 1 + h >= bytes.len() || bytes[i + 1 + h] != b'#' {
                            ok = false;
                            break;
                        }
                    }
                    if ok {
                        i += 1 + hashes;
                        state.in_string = None;
                        return i;
                    }
                }
                i += 1;
            }
            line.len()
        }
    }
}

fn try_start_raw_string(bytes: &[u8], i: usize) -> Option<(usize, usize)> {
    if i >= bytes.len() {
        return None;
    }

    let mut start = i;
    if bytes[i] == b'b' {
        start += 1;
    }
    if start >= bytes.len() || bytes[start] != b'r' {
        return None;
    }

    let mut j = start + 1;
    while j < bytes.len() && bytes[j] == b'#' {
        j += 1;
    }
    if j < bytes.len() && bytes[j] == b'"' {
        return Some((start, j + 1));
    }

    None
}

fn scan_lifetime_or_char(line: &str, i: usize) -> Option<(usize, usize)> {
    let bytes = line.as_bytes();
    if i + 1 >= bytes.len() {
        return None;
    }

    if is_ident_start(bytes[i + 1]) {
        let mut j = i + 2;
        while j < bytes.len() && is_ident_continue(bytes[j]) {
            j += 1;
        }
        return Some((i, j));
    }

    let mut j = i + 1;
    if j < bytes.len() && bytes[j] == b'\\' {
        j += 2;
    } else {
        j += 1;
    }
    if j < bytes.len() && bytes[j] == b'\'' {
        return Some((i, j + 1));
    }
    None
}

fn scan_number(bytes: &[u8], mut i: usize) -> usize {
    if i + 1 < bytes.len() && bytes[i] == b'0' && matches!(bytes[i + 1], b'x' | b'b' | b'o') {
        i += 2;
        while i < bytes.len() && (bytes[i].is_ascii_hexdigit() || bytes[i] == b'_') {
            i += 1;
        }
        return i;
    }

    while i < bytes.len() && (bytes[i].is_ascii_digit() || bytes[i] == b'_') {
        i += 1;
    }

    if i + 1 < bytes.len() && bytes[i] == b'.' && bytes[i + 1].is_ascii_digit() {
        i += 1;
        while i < bytes.len() && (bytes[i].is_ascii_digit() || bytes[i] == b'_') {
            i += 1;
        }
    }

    if i < bytes.len() && matches!(bytes[i], b'e' | b'E') {
        i += 1;
        if i < bytes.len() && matches!(bytes[i], b'+' | b'-') {
            i += 1;
        }
        while i < bytes.len() && (bytes[i].is_ascii_digit() || bytes[i] == b'_') {
            i += 1;
        }
    }

    while i < bytes.len() && is_ident_continue(bytes[i]) {
        i += 1;
    }

    i
}

fn scan_identifier(bytes: &[u8], mut i: usize) -> usize {
    while i < bytes.len() && is_ident_continue(bytes[i]) {
        i += 1;
    }
    i
}

fn looks_like_function_call(bytes: &[u8], mut i: usize) -> bool {
    while i < bytes.len() && bytes[i].is_ascii_whitespace() {
        i += 1;
    }
    i < bytes.len() && bytes[i] == b'('
}

fn looks_like_macro(bytes: &[u8], mut i: usize) -> bool {
    while i < bytes.len() && bytes[i].is_ascii_whitespace() {
        i += 1;
    }
    i < bytes.len() && bytes[i] == b'!'
}

fn looks_like_type(token: &str) -> bool {
    token.chars().next().is_some_and(|ch| ch.is_ascii_uppercase())
}

fn is_line_comment_start(bytes: &[u8], i: usize) -> bool {
    i + 1 < bytes.len() && bytes[i] == b'/' && bytes[i + 1] == b'/'
}

fn is_block_comment_start(bytes: &[u8], i: usize) -> bool {
    i + 1 < bytes.len() && bytes[i] == b'/' && bytes[i + 1] == b'*'
}

fn is_block_comment_end(bytes: &[u8], i: usize) -> bool {
    i + 1 < bytes.len() && bytes[i] == b'*' && bytes[i + 1] == b'/'
}

fn is_escaped(bytes: &[u8], i: usize) -> bool {
    if i == 0 {
        return false;
    }
    let mut backslashes = 0usize;
    let mut idx = i;
    while idx > 0 {
        idx -= 1;
        if bytes[idx] == b'\\' {
            backslashes += 1;
        } else {
            break;
        }
    }
    backslashes % 2 == 1
}

fn is_ident_start(ch: u8) -> bool {
    ch == b'_' || ch.is_ascii_alphabetic()
}

fn is_ident_continue(ch: u8) -> bool {
    ch == b'_' || ch.is_ascii_alphanumeric()
}

fn is_operator(ch: u8) -> bool {
    matches!(
        ch,
        b'=' | b'+' | b'-' | b'*' | b'/' | b'%' | b'!' | b'<' | b'>' | b'&' | b'|' | b'^' | b':'
    )
}

fn is_delimiter(ch: u8) -> bool {
    matches!(ch, b'(' | b')' | b'[' | b']' | b'{' | b'}' | b',' | b';' | b'.')
}

fn is_char_literal_token(token: &str) -> bool {
    token.len() >= 3 && token.starts_with('\'') && token.ends_with('\'')
}

fn push_span(spans: &mut Vec<Span<'static>>, line: &str, start: usize, end: usize, style: Style) {
    if start >= end || end > line.len() {
        return;
    }
    if let Some(slice) = line.get(start..end) {
        spans.push(Span::styled(slice.to_string(), style));
    }
}
