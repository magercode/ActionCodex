use anyhow::Result;
use std::path::PathBuf;
mod app;
mod editor;
mod file_tree;
mod manager;
mod signature;
mod syntax;
mod tab_manager;
mod terminal;
mod ui;

fn main() -> Result<()> {
    let startup_files = std::env::args_os()
        .skip(1)
        .map(PathBuf::from)
        .collect::<Vec<_>>();
    let mut terminal = terminal::setup_terminal()?;
    let editor = app::run(&mut terminal, &startup_files);
    terminal::restore_terminal(terminal)?;
    editor
}
