use std::collections::HashSet;
use std::fs;
use std::process::Command;

use anyhow::{Context, Result, anyhow};

#[derive(Debug, Clone)]
pub struct CargoPackage {
    pub name: String,
    pub version: String,
    pub description: String,
}

pub fn search_crate(query: &str) -> Result<String> {
    if query.trim().is_empty() {
        return Err(anyhow!("keyword pencarian tidak boleh kosong"));
    }
    run_cargo(["search", query.trim(), "--limit", "10"])
}

pub fn search_crate_packages(query: &str) -> Result<Vec<CargoPackage>> {
    let output = search_crate(query)?;
    Ok(output
        .lines()
        .filter_map(parse_search_line)
        .collect::<Vec<_>>())
}

pub fn add_crate(crate_name: &str) -> Result<String> {
    if crate_name.trim().is_empty() {
        return Err(anyhow!("nama crate tidak boleh kosong"));
    }
    run_cargo(["add", crate_name.trim()])
}

pub fn remove_crate(crate_name: &str) -> Result<String> {
    if crate_name.trim().is_empty() {
        return Err(anyhow!("nama crate tidak boleh kosong"));
    }
    run_cargo(["remove", crate_name.trim()])
}

pub fn installed_packages() -> Result<HashSet<String>> {
    let content = fs::read_to_string("Cargo.toml").context("gagal membaca Cargo.toml")?;
    let mut in_dep_block = false;
    let mut set = HashSet::new();

    for raw in content.lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if line.starts_with('[') && line.ends_with(']') {
            let section = &line[1..line.len() - 1];
            in_dep_block = matches!(
                section,
                "dependencies"
                    | "dev-dependencies"
                    | "build-dependencies"
                    | "workspace.dependencies"
            );
            continue;
        }

        if !in_dep_block {
            continue;
        }

        let Some(eq_idx) = line.find('=') else {
            continue;
        };
        let name = line[..eq_idx].trim();
        if name.is_empty() || name.contains(' ') || name.starts_with('\"') {
            continue;
        }
        set.insert(name.to_string());
    }

    Ok(set)
}

fn run_cargo<const N: usize>(args: [&str; N]) -> Result<String> {
    let output = Command::new("cargo")
        .args(args)
        .output()
        .context("gagal menjalankan cargo command")?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    if output.status.success() {
        if stdout.trim().is_empty() {
            Ok(format!("Berhasil: {}", args.join(" ")))
        } else {
            Ok(stdout)
        }
    } else {
        let msg = if stderr.trim().is_empty() {
            stdout
        } else {
            stderr
        };
        Err(anyhow!("cargo {} gagal: {}", args.join(" "), msg.trim()))
    }
}

fn parse_search_line(line: &str) -> Option<CargoPackage> {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return None;
    }

    let eq = trimmed.find('=')?;
    let name = trimmed[..eq].trim();
    if name.is_empty() {
        return None;
    }

    let rest = trimmed[eq + 1..].trim();
    let first_quote = rest.find('"')?;
    let rest_after_first = &rest[first_quote + 1..];
    let second_quote = rest_after_first.find('"')?;
    let version = rest_after_first[..second_quote].to_string();

    let desc = if let Some(hash) = trimmed.find('#') {
        trimmed[hash + 1..].trim().to_string()
    } else {
        String::new()
    };

    Some(CargoPackage {
        name: name.to_string(),
        version,
        description: desc,
    })
}
