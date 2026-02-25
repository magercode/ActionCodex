use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Result;

#[derive(Debug, Clone)]
pub struct FileTreeEntry {
    path: PathBuf,
    name: String,
    depth: usize,
    is_dir: bool,
}

#[derive(Debug)]
pub struct FileTree {
    root: PathBuf,
    expanded_dirs: HashSet<PathBuf>,
    entries: Vec<FileTreeEntry>,
    selected: usize,
    scroll: usize,
}

impl FileTree {
    pub fn new(root: PathBuf) -> Result<Self> {
        let mut tree = Self {
            expanded_dirs: HashSet::from([root.clone()]),
            root,
            entries: Vec::new(),
            selected: 0,
            scroll: 0,
        };
        tree.refresh()?;
        Ok(tree)
    }

    pub fn refresh(&mut self) -> Result<()> {
        self.expanded_dirs.retain(|path| path.is_dir());
        self.expanded_dirs.insert(self.root.clone());

        let mut entries = Vec::new();
        self.walk_dir(&self.root, 0, &mut entries)?;
        self.entries = entries;
        if self.entries.is_empty() {
            self.selected = 0;
            self.scroll = 0;
        } else {
            self.selected = self.selected.min(self.entries.len() - 1);
            self.scroll = self.scroll.min(self.selected);
        }
        Ok(())
    }

    pub fn entries(&self) -> &[FileTreeEntry] {
        &self.entries
    }

    pub fn entry(&self, index: usize) -> Option<&FileTreeEntry> {
        self.entries.get(index)
    }

    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }

    pub fn root(&self) -> &Path {
        self.root.as_path()
    }

    pub fn selected_index(&self) -> usize {
        self.selected
    }

    pub fn scroll(&self) -> usize {
        self.scroll
    }

    pub fn ensure_visible(&mut self, viewport_height: usize) {
        if self.selected < self.scroll {
            self.scroll = self.selected;
        } else if self.selected >= self.scroll + viewport_height {
            self.scroll = self.selected + 1 - viewport_height;
        }
    }

    pub fn move_up(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    pub fn move_down(&mut self) {
        if self.selected + 1 < self.entries.len() {
            self.selected += 1;
        }
    }

    pub fn set_selected(&mut self, index: usize) {
        if self.entries.is_empty() {
            self.selected = 0;
        } else {
            self.selected = index.min(self.entries.len() - 1);
        }
    }

    pub fn selected_path(&self) -> Option<&Path> {
        self.entries
            .get(self.selected)
            .map(|entry| entry.path.as_path())
    }

    pub fn toggle_selected_dir(&mut self) -> Result<()> {
        let Some(entry) = self.entries.get(self.selected) else {
            return Ok(());
        };
        if !entry.is_dir {
            return Ok(());
        }
        if self.expanded_dirs.contains(&entry.path) {
            self.expanded_dirs.remove(&entry.path);
        } else {
            self.expanded_dirs.insert(entry.path.clone());
        }
        self.refresh()
    }

    fn walk_dir(&self, dir: &Path, depth: usize, output: &mut Vec<FileTreeEntry>) -> Result<()> {
        let read_dir = match fs::read_dir(dir) {
            Ok(entries) => entries,
            Err(_) => return Ok(()),
        };
        let mut children = read_dir
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                let name = entry.file_name();
                let name = name.to_string_lossy();
                !name.starts_with('.')
            })
            .collect::<Vec<_>>();
        children.sort_by_key(|entry| {
            let path = entry.path();
            (
                !path.is_dir(),
                entry.file_name().to_string_lossy().to_ascii_lowercase(),
            )
        });

        for child in children {
            let path = child.path();
            let is_dir = path.is_dir();
            let name = child.file_name().to_string_lossy().to_string();
            output.push(FileTreeEntry {
                path: path.clone(),
                name,
                depth,
                is_dir,
            });

            if is_dir && self.expanded_dirs.contains(&path) {
                self.walk_dir(&path, depth + 1, output)?;
            }
        }
        Ok(())
    }
}

impl FileTreeEntry {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn depth(&self) -> usize {
        self.depth
    }

    pub fn is_dir(&self) -> bool {
        self.is_dir
    }

    pub fn path(&self) -> &Path {
        self.path.as_path()
    }
}
