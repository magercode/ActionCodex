use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};

pub fn add_workspace_member(member: &str) -> Result<String> {
    edit_workspace_members(member, true)
}

pub fn remove_workspace_member(member: &str) -> Result<String> {
    edit_workspace_members(member, false)
}

fn edit_workspace_members(member: &str, add: bool) -> Result<String> {
    let member = member.trim();
    if member.is_empty() {
        anyhow::bail!("path member workspace tidak boleh kosong");
    }

    let manifest = PathBuf::from("Cargo.toml");
    let content = fs::read_to_string(&manifest)
        .with_context(|| format!("gagal membaca {}", manifest.display()))?;

    let mut lines = content.lines().map(str::to_string).collect::<Vec<_>>();
    let ws_start = find_section_start(&lines, "workspace");

    let changed = if let Some(start) = ws_start {
        let end = find_section_end(&lines, start);
        upsert_member_in_workspace_block(&mut lines, start, end, member, add)
    } else {
        append_workspace_block(&mut lines, member, add)
    };

    if !changed {
        return Ok(if add {
            format!("Member '{}' sudah ada", member)
        } else {
            format!("Member '{}' tidak ditemukan", member)
        });
    }

    let new_content = lines.join("\n") + "\n";
    fs::write(&manifest, new_content)
        .with_context(|| format!("gagal menulis {}", manifest.display()))?;

    Ok(if add {
        format!("Member workspace ditambahkan: {}", member)
    } else {
        format!("Member workspace dihapus: {}", member)
    })
}

fn append_workspace_block(lines: &mut Vec<String>, member: &str, add: bool) -> bool {
    if !add {
        return false;
    }
    if !lines.is_empty() && !lines.last().is_some_and(|l| l.is_empty()) {
        lines.push(String::new());
    }
    lines.push("[workspace]".to_string());
    lines.push("members = [".to_string());
    lines.push(format!("    \"{}\",", member));
    lines.push("]".to_string());
    true
}

fn upsert_member_in_workspace_block(
    lines: &mut Vec<String>,
    start: usize,
    end: usize,
    member: &str,
    add: bool,
) -> bool {
    let members_line = (start + 1..end)
        .find(|&idx| lines[idx].trim_start().starts_with("members") && lines[idx].contains('='));

    let Some(members_start) = members_line else {
        if !add {
            return false;
        }
        lines.insert(end, "members = [".to_string());
        lines.insert(end + 1, format!("    \"{}\",", member));
        lines.insert(end + 2, "]".to_string());
        return true;
    };

    let open = members_start;
    let mut close = members_start;

    if !lines[members_start].contains('[') {
        return false;
    }

    if lines[members_start].contains(']') {
        let parsed = parse_member_list_inline(&lines[members_start]);
        return write_inline_members(lines, members_start, parsed, member, add);
    }

    for idx in members_start + 1..end {
        if lines[idx].contains(']') {
            close = idx;
            break;
        }
    }

    if close == open {
        return false;
    }

    let mut existing = Vec::new();
    for line in lines.iter().take(close).skip(open + 1) {
        if let Some(value) = parse_quoted_value(line) {
            existing.push(value);
        }
    }

    let changed = if add {
        if existing.iter().any(|v| v == member) {
            false
        } else {
            existing.push(member.to_string());
            true
        }
    } else {
        let prev = existing.len();
        existing.retain(|v| v != member);
        existing.len() != prev
    };

    if !changed {
        return false;
    }

    existing.sort();
    let mut rebuilt = Vec::new();
    rebuilt.push(lines[open].clone());
    rebuilt.extend(existing.into_iter().map(|m| format!("    \"{}\",", m)));
    rebuilt.push("]".to_string());

    lines.splice(open..=close, rebuilt);
    true
}

fn write_inline_members(
    lines: &mut [String],
    at: usize,
    mut existing: Vec<String>,
    member: &str,
    add: bool,
) -> bool {
    let changed = if add {
        if existing.iter().any(|v| v == member) {
            false
        } else {
            existing.push(member.to_string());
            true
        }
    } else {
        let prev = existing.len();
        existing.retain(|v| v != member);
        existing.len() != prev
    };

    if !changed {
        return false;
    }

    existing.sort();
    let body = existing
        .into_iter()
        .map(|m| format!("\"{}\"", m))
        .collect::<Vec<_>>()
        .join(", ");
    lines[at] = format!("members = [{}]", body);
    true
}

fn parse_member_list_inline(line: &str) -> Vec<String> {
    let Some(open) = line.find('[') else {
        return Vec::new();
    };
    let Some(close) = line.rfind(']') else {
        return Vec::new();
    };
    line[open + 1..close]
        .split(',')
        .filter_map(|item| {
            let v = item.trim().trim_matches('"');
            if v.is_empty() {
                None
            } else {
                Some(v.to_string())
            }
        })
        .collect::<Vec<_>>()
}

fn parse_quoted_value(line: &str) -> Option<String> {
    let start = line.find('"')?;
    let end = line[start + 1..].find('"')? + start + 1;
    Some(line[start + 1..end].to_string())
}

fn find_section_start(lines: &[String], name: &str) -> Option<usize> {
    let needle = format!("[{}]", name);
    lines.iter().position(|line| line.trim() == needle)
}

fn find_section_end(lines: &[String], start: usize) -> usize {
    for (idx, line) in lines.iter().enumerate().skip(start + 1) {
        let t = line.trim();
        if t.starts_with('[') && t.ends_with(']') {
            return idx;
        }
    }
    lines.len()
}
