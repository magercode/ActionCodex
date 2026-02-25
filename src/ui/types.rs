use ratatui::layout::Rect;
use std::collections::HashSet;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum DialogButton {
    #[default]
    Save,
    Cancel,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum FocusPane {
    #[default]
    Editor,
    FileTree,
    Tabs,
}

#[derive(Debug, Default)]
pub struct SaveDialog {
    pub filename: String,
    pub selected: DialogButton,
}

impl SaveDialog {
    pub fn toggle_button(&mut self) {
        self.selected = match self.selected {
            DialogButton::Save => DialogButton::Cancel,
            DialogButton::Cancel => DialogButton::Save,
        };
    }

    pub fn push_char(&mut self, ch: char) {
        self.filename.push(ch);
    }

    pub fn pop_char(&mut self) {
        self.filename.pop();
    }
}

#[derive(Debug, Default)]
pub struct SearchDialog {
    pub query: String,
}

impl SearchDialog {
    pub fn push_char(&mut self, ch: char) {
        self.query.push(ch);
    }

    pub fn pop_char(&mut self) {
        self.query.pop();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManagerAction {
    FormatRust,
    CargoSearch,
    CargoAdd,
    CargoRemove,
    WorkspaceAddMember,
    WorkspaceRemoveMember,
    Close,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManagerMode {
    Menu,
    Input(ManagerAction),
    SearchResults,
    Output,
}

#[derive(Debug, Clone)]
pub struct CargoSearchItem {
    pub name: String,
    pub version: String,
    pub description: String,
    pub installed: bool,
}

#[derive(Debug)]
pub struct ManagerDialog {
    pub selected: usize,
    pub mode: ManagerMode,
    pub input: String,
    pub output: String,
    pub output_scroll_y: usize,
    pub output_scroll_x: usize,
    pub search_results: Vec<CargoSearchItem>,
    pub search_selected: usize,
    pub search_scroll_y: usize,
    pub search_scroll_x: usize,
    pub installed_packages: HashSet<String>,
}

impl Default for ManagerDialog {
    fn default() -> Self {
        Self {
            selected: 0,
            mode: ManagerMode::Menu,
            input: String::new(),
            output: String::new(),
            output_scroll_y: 0,
            output_scroll_x: 0,
            search_results: Vec::new(),
            search_selected: 0,
            search_scroll_y: 0,
            search_scroll_x: 0,
            installed_packages: HashSet::new(),
        }
    }
}

impl ManagerDialog {
    pub fn actions() -> [ManagerAction; 7] {
        [
            ManagerAction::FormatRust,
            ManagerAction::CargoSearch,
            ManagerAction::CargoAdd,
            ManagerAction::CargoRemove,
            ManagerAction::WorkspaceAddMember,
            ManagerAction::WorkspaceRemoveMember,
            ManagerAction::Close,
        ]
    }

    pub fn current_action(&self) -> ManagerAction {
        Self::actions()[self.selected]
    }

    pub fn move_up(&mut self) {
        if self.selected == 0 {
            self.selected = Self::actions().len() - 1;
        } else {
            self.selected -= 1;
        }
    }

    pub fn move_down(&mut self) {
        self.selected = (self.selected + 1) % Self::actions().len();
    }

    pub fn set_output(&mut self, text: String) {
        self.output = text;
        self.output_scroll_y = 0;
        self.output_scroll_x = 0;
        self.mode = ManagerMode::Output;
    }

    pub fn set_search_results(
        &mut self,
        mut results: Vec<CargoSearchItem>,
        installed: HashSet<String>,
    ) {
        for item in &mut results {
            item.installed = installed.contains(&item.name);
        }
        self.search_results = results;
        self.search_selected = 0;
        self.search_scroll_y = 0;
        self.search_scroll_x = 0;
        self.installed_packages = installed;
        self.mode = ManagerMode::SearchResults;
    }

    pub fn refresh_installed_markers(&mut self) {
        for item in &mut self.search_results {
            item.installed = self.installed_packages.contains(&item.name);
        }
    }

    pub fn move_search_up(&mut self) {
        if self.search_selected > 0 {
            self.search_selected -= 1;
            if self.search_selected < self.search_scroll_y {
                self.search_scroll_y = self.search_selected;
            }
        }
    }

    pub fn move_search_down(&mut self, viewport_height: usize) {
        if self.search_selected + 1 < self.search_results.len() {
            self.search_selected += 1;
            if viewport_height > 0
                && self.search_selected >= self.search_scroll_y.saturating_add(viewport_height)
            {
                self.search_scroll_y = self.search_selected + 1 - viewport_height;
            }
        }
    }

    pub fn selected_search_item(&self) -> Option<&CargoSearchItem> {
        self.search_results.get(self.search_selected)
    }

    pub fn selected_search_name(&self) -> Option<String> {
        self.selected_search_item().map(|item| item.name.clone())
    }

    pub fn mark_installed(&mut self, package: &str, installed: bool) {
        if installed {
            self.installed_packages.insert(package.to_string());
        } else {
            self.installed_packages.remove(package);
        }
        self.refresh_installed_markers();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StartMenuAction {
    NewTab,
    FocusFileTree,
    OpenManager,
    ShowHelp,
    ContinueEditor,
}

#[derive(Debug)]
pub struct StartMenuDialog {
    pub selected: usize,
}

impl Default for StartMenuDialog {
    fn default() -> Self {
        Self { selected: 0 }
    }
}

impl StartMenuDialog {
    pub fn actions() -> [StartMenuAction; 5] {
        [
            StartMenuAction::NewTab,
            StartMenuAction::FocusFileTree,
            StartMenuAction::OpenManager,
            StartMenuAction::ShowHelp,
            StartMenuAction::ContinueEditor,
        ]
    }

    pub fn move_up(&mut self) {
        if self.selected == 0 {
            self.selected = Self::actions().len() - 1;
        } else {
            self.selected -= 1;
        }
    }

    pub fn move_down(&mut self) {
        self.selected = (self.selected + 1) % Self::actions().len();
    }

    pub fn current_action(&self) -> StartMenuAction {
        Self::actions()[self.selected]
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TabHit {
    pub index: usize,
    pub rect: Rect,
}

#[derive(Debug, Clone, Copy)]
pub struct SaveDialogHit {
    pub input_rect: Rect,
    pub save_button_rect: Rect,
    pub cancel_button_rect: Rect,
}

#[derive(Debug, Clone, Copy)]
pub struct ContextMenuDialog {
    pub x: u16,
    pub y: u16,
}

#[derive(Debug, Clone, Copy)]
pub struct ContextMenuHit {
    pub select_all_rect: Rect,
    pub copy_rect: Rect,
    pub paste_rect: Rect,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExplorerMenuAction {
    AddFolder,
    AddFile,
    RenameEntry,
    DeleteEntry,
}

#[derive(Debug, Clone)]
pub struct ExplorerContextMenuDialog {
    pub x: u16,
    pub y: u16,
    pub base_dir: PathBuf,
    pub target_path: Option<PathBuf>,
    pub target_is_dir: bool,
    pub actions: Vec<ExplorerMenuAction>,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ExplorerContextMenuHit {
    pub add_folder_rect: Option<Rect>,
    pub add_file_rect: Option<Rect>,
    pub rename_entry_rect: Option<Rect>,
    pub delete_entry_rect: Option<Rect>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExplorerInputMode {
    AddFolder,
    AddFile,
    RenameEntry,
}

#[derive(Debug, Clone)]
pub struct ExplorerInputDialog {
    pub mode: ExplorerInputMode,
    pub base_dir: PathBuf,
    pub target_path: Option<PathBuf>,
    pub input: String,
    pub selected: DialogButton,
}

impl ExplorerInputDialog {
    pub fn new(mode: ExplorerInputMode, base_dir: PathBuf, target_path: Option<PathBuf>) -> Self {
        let input = match mode {
            ExplorerInputMode::AddFolder => String::from("folder-baru"),
            ExplorerInputMode::AddFile => String::from("file-baru.txt"),
            ExplorerInputMode::RenameEntry => target_path
                .as_ref()
                .and_then(|path| {
                    path.file_name()
                        .map(|name| name.to_string_lossy().to_string())
                })
                .unwrap_or_else(|| String::from("nama-baru")),
        };
        Self {
            mode,
            base_dir,
            target_path,
            input,
            selected: DialogButton::Save,
        }
    }

    pub fn toggle_button(&mut self) {
        self.selected = match self.selected {
            DialogButton::Save => DialogButton::Cancel,
            DialogButton::Cancel => DialogButton::Save,
        };
    }

    pub fn push_char(&mut self, ch: char) {
        self.input.push(ch);
    }

    pub fn pop_char(&mut self) {
        self.input.pop();
    }

    pub fn title(&self) -> &'static str {
        match self.mode {
            ExplorerInputMode::AddFolder => " Tambah Folder ",
            ExplorerInputMode::AddFile => " Tambah File ",
            ExplorerInputMode::RenameEntry => " Ubah Nama ",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ExplorerInputDialogHit {
    pub input_rect: Rect,
    pub save_button_rect: Rect,
    pub cancel_button_rect: Rect,
}

#[derive(Debug, Default, Clone)]
pub struct UiState {
    pub tab_hits: Vec<TabHit>,
    pub tab_content_rect: Rect,
    pub tree_content_rect: Rect,
    pub editor_content_rect: Rect,
    pub save_dialog_hit: Option<SaveDialogHit>,
    pub context_menu_hit: Option<ContextMenuHit>,
    pub explorer_context_menu_hit: Option<ExplorerContextMenuHit>,
    pub explorer_input_dialog_hit: Option<ExplorerInputDialogHit>,
}
