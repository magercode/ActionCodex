use std::io::Write;
use std::process::{Command, Stdio};

use anyhow::{Context, Result, anyhow};

pub fn format_rust_source(source: &str) -> Result<String> {
    let mut child = Command::new("rustfmt")
        .args(["--edition", "2024", "--emit", "stdout"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .context("gagal menjalankan rustfmt")?;

    if let Some(stdin) = child.stdin.as_mut() {
        stdin
            .write_all(source.as_bytes())
            .context("gagal menulis source ke rustfmt")?;
    }

    let output = child
        .wait_with_output()
        .context("gagal membaca output rustfmt")?;

    if !output.status.success() {
        return Err(anyhow!(
            "rustfmt gagal: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    String::from_utf8(output.stdout).context("output rustfmt bukan UTF-8 valid")
}
