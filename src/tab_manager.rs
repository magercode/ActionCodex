use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Result;

use crate::editor::Editor;

#[derive(Debug)]
pub struct Tab {
    title: String,
    path: Option<PathBuf>,
    dirty: bool,
    editor: Editor,
}

#[derive(Debug)]
pub struct TabManager {
    tabs: Vec<Tab>,
    active: usize,
    untitled_count: usize,
}

impl TabManager {
    pub fn new() -> Self {
        let mut manager = Self {
            tabs: Vec::new(),
            active: 0,
            untitled_count: 0,
        };
        manager.new_tab();
        manager
    }

    pub fn tabs(&self) -> &[Tab] {
        &self.tabs
    }

    pub fn active_index(&self) -> usize {
        self.active
    }

    pub fn active_editor(&self) -> &Editor {
        &self.tabs[self.active].editor
    }

    pub fn active_editor_mut(&mut self) -> &mut Editor {
        &mut self.tabs[self.active].editor
    }

    pub fn active_path(&self) -> Option<&Path> {
        self.tabs[self.active].path.as_deref()
    }

    pub fn active_tab_title(&self) -> &str {
        &self.tabs[self.active].title
    }

    pub fn active_suggested_filename(&self) -> String {
        if let Some(path) = self.active_path() {
            return path.to_string_lossy().to_string();
        }
        format!("{}.txt", self.active_tab_title())
    }

    pub fn mark_active_dirty(&mut self) {
        self.tabs[self.active].dirty = true;
    }

    pub fn new_tab(&mut self) {
        self.untitled_count += 1;
        let title = format!("untitled-{}", self.untitled_count);
        self.tabs.push(Tab {
            title,
            path: None,
            dirty: false,
            editor: Editor::new(),
        });
        self.active = self.tabs.len() - 1;
    }

    pub fn clear_tabs(&mut self) {
        self.tabs.clear();
        self.active = 0;
    }

    pub fn next_tab(&mut self) {
        if self.tabs.is_empty() {
            return;
        }
        self.active = (self.active + 1) % self.tabs.len();
    }

    pub fn set_active(&mut self, index: usize) {
        if index < self.tabs.len() {
            self.active = index;
        }
    }

    pub fn prev_tab(&mut self) {
        if self.tabs.is_empty() {
            return;
        }
        if self.active == 0 {
            self.active = self.tabs.len() - 1;
        } else {
            self.active -= 1;
        }
    }

    pub fn open_file(&mut self, path: &Path) -> Result<()> {
        if let Some((idx, _)) = self
            .tabs
            .iter()
            .enumerate()
            .find(|(_, tab)| tab.path.as_deref() == Some(path))
        {
            self.active = idx;
            return Ok(());
        }

        let text = fs::read_to_string(path)?;
        let title = path
            .file_name()
            .map(|name| name.to_string_lossy().to_string())
            .unwrap_or_else(|| path.to_string_lossy().to_string());
        self.tabs.push(Tab {
            title,
            path: Some(path.to_path_buf()),
            dirty: false,
            editor: Editor::from_text(&text),
        });
        self.active = self.tabs.len() - 1;
        Ok(())
    }

    pub fn open_or_create_file(&mut self, path: &Path) -> Result<()> {
        if path.exists() {
            return self.open_file(path);
        }

        if let Some((idx, _)) = self
            .tabs
            .iter()
            .enumerate()
            .find(|(_, tab)| tab.path.as_deref() == Some(path))
        {
            self.active = idx;
            return Ok(());
        }

        let title = path
            .file_name()
            .map(|name| name.to_string_lossy().to_string())
            .unwrap_or_else(|| path.to_string_lossy().to_string());

        self.tabs.push(Tab {
            title,
            path: Some(path.to_path_buf()),
            dirty: false,
            editor: Editor::new(),
        });
        self.active = self.tabs.len() - 1;
        Ok(())
    }

    pub fn save_active_to(&mut self, path: PathBuf) -> Result<()> {
        let content = self.active_editor().text();
        fs::write(&path, content)?;
        let title = path
            .file_name()
            .map(|name| name.to_string_lossy().to_string())
            .unwrap_or_else(|| path.to_string_lossy().to_string());
        let tab = &mut self.tabs[self.active];
        tab.path = Some(path);
        tab.title = title;
        tab.dirty = false;
        Ok(())
    }

    pub fn close_active_tab(&mut self) {
        if self.tabs.is_empty() {
            self.new_tab();
            return;
        }

        self.tabs.remove(self.active);
        if self.tabs.is_empty() {
            self.new_tab();
            return;
        }

        if self.active >= self.tabs.len() {
            self.active = self.tabs.len() - 1;
        }
    }
}

impl Tab {
    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn dirty(&self) -> bool {
        self.dirty
    }
}
