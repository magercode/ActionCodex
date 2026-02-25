using Terminal.Gui;

namespace ActionCodex
{
    public partial class ActionCodexEditor
    {
        private void NewFile()
        {
            CreateNewTab();
            if (statusLabel != null)
                statusLabel.Text = GetLocalizedString("StatusNew");
        }

        private void OpenFile()
        {
            var dialog = new OpenDialog(GetLocalizedString("MenuOpen"), GetLocalizedString("MenuOpen"))
            {
                AllowsMultipleSelection = false,
                CanChooseDirectories = false,
                CanChooseFiles = true
            };

            Application.Run(dialog);

            if (!dialog.Canceled && dialog.FilePaths != null && dialog.FilePaths.Count > 0)
            {
                try
                {
                    string? filePath = dialog.FilePaths[0]?.ToString();
                    if (string.IsNullOrEmpty(filePath)) return;

                    var existingTab = openTabs.Values.FirstOrDefault(t => t.FilePath == filePath);
                    if (existingTab != null)
                    {
                        if (tabView != null) tabView.SelectedTab = existingTab.Tab;
                        return;
                    }

                    string content = File.ReadAllText(filePath);

                    var currentCtx = GetCurrentTabContext();
                    if (currentCtx != null && currentCtx.FilePath.StartsWith("Untitled") && !currentCtx.IsModified)
                    {
                        tabView?.RemoveTab(currentCtx.Tab);
                        openTabs.Remove(currentCtx.Tab);
                    }

                    CreateNewTab(filePath, content);

                    if (statusLabel != null)
                        statusLabel.Text = $"Loaded {Path.GetFileName(filePath)}";
                }
                catch (Exception ex)
                {
                    ShowError($"Error opening file: {ex.Message}");
                }
            }
        }

        private void SaveFile()
        {
            var ctx = GetCurrentTabContext();
            if (ctx == null) return;

            if (ctx.FilePath.StartsWith("Untitled"))
            {
                SaveFileAs();
            }
            else
            {
                try
                {
                    File.WriteAllText(ctx.FilePath, ctx.Editor.Text.ToString());
                    ctx.IsModified = false;
                    UpdateTitle();
                    if (statusLabel != null)
                        statusLabel.Text = $"Saved {Path.GetFileName(ctx.FilePath)}";
                }
                catch (Exception ex)
                {
                    ShowError($"Error saving file: {ex.Message}");
                }
            }
        }

        private void SaveFileAs()
        {
            var ctx = GetCurrentTabContext();
            if (ctx == null) return;

            var dialog = new SaveDialog(GetLocalizedString("MenuSaveAs"), GetLocalizedString("MenuSaveAs"))
            {
                AllowedFileTypes = new string[] { ".txt", ".cs", ".vb" },
                CanCreateDirectories = true
            };

            Application.Run(dialog);

            if (!dialog.Canceled && dialog.FilePath != null && !string.IsNullOrEmpty(dialog.FilePath.ToString()))
            {
                try
                {
                    string filePath = dialog.FilePath.ToString() ?? "";
                    if (string.IsNullOrEmpty(filePath)) return;

                    if (!filePath.Contains("."))
                    {
                        filePath += ".txt";
                    }

                    File.WriteAllText(filePath, ctx.Editor.Text.ToString());
                    ctx.FilePath = filePath;
                    ctx.IsModified = false;
                    UpdateTitle();

                    if (statusLabel != null)
                        statusLabel.Text = $"Saved as {Path.GetFileName(ctx.FilePath)}";
                }
                catch (Exception ex)
                {
                    ShowError($"Error saving file: {ex.Message}");
                }
            }
        }

        private void CloseFile()
        {
            var ctx = GetCurrentTabContext();
            if (ctx == null) return;

            if (CheckUnsavedChanges(ctx))
            {
                tabView?.RemoveTab(ctx.Tab);
                openTabs.Remove(ctx.Tab);

                if (openTabs.Count == 0)
                {
                    CreateNewTab();
                }

                UpdateTitle();
                if (statusLabel != null)
                    statusLabel.Text = GetLocalizedString("MenuCloseTab");
            }
        }

        private void Quit()
        {
            if (CheckAllUnsavedChanges())
            {
                Application.RequestStop();
            }
        }
    }
}