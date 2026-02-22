using Terminal.Gui;

namespace ActionCodex
{
    public partial class ActionCodexEditor
    {
        private TabContext? GetCurrentTabContext()
        {
            if (tabView?.SelectedTab != null && openTabs.ContainsKey(tabView.SelectedTab))
            {
                return openTabs[tabView.SelectedTab];
            }
            return null;
        }

        private void CreateNewTab(string filePath = "Untitled", string content = "")
        {
            var textView = new TextView { Width = Dim.Fill(), Height = Dim.Fill(), Text = content, AllowsTab = true };
            var lineNumView = new TextView { Width = 7, Height = Dim.Fill(), ReadOnly = true, CanFocus = false };
            var container = new View { Width = Dim.Fill(), Height = Dim.Fill() };
            container.Add(lineNumView, textView);

            var tabTitle = filePath == "Untitled" ? $"Untitled {tabCounter++}" : Path.GetFileName(filePath);
            var tab = new TabView.Tab(tabTitle, container);
            var ctx = new TabContext { Tab = tab, Editor = textView, LineNumbers = lineNumView, FilePath = filePath };

            textView.TextChanged += () => { ctx.IsModified = true; UpdateTitle(); };
            textView.KeyDown += (e) => HandleKeyDown(e, ctx);

            if (tabView != null) { tabView.AddTab(tab, true); openTabs[tab] = ctx; }
            UpdateTitle();
        }

        private void UpdateTitle()
        {
            var ctx = GetCurrentTabContext();
            if (ctx != null)
            {
                Title = $"Action Codex - {Path.GetFileName(ctx.FilePath)}{(ctx.IsModified ? " *" : "")}";
                ctx.Tab.Text = Path.GetFileName(ctx.FilePath) + (ctx.IsModified ? " *" : "");
            }
            else
            {
                Title = GetLocalizedString("WindowTitle");
            }
        }

        private bool UpdatePositionDisplay(MainLoop caller)
        {
            var ctx = GetCurrentTabContext();
            if (ctx != null && positionLabel != null)
            {
                var cursor = ctx.Editor.CursorPosition;
                positionLabel.Text = GetLocalizedString("PositionFormat")
                    .Replace("{0}", (cursor.Y + 1).ToString())
                    .Replace("{1}", (cursor.X + 1).ToString());
            }
            return true;
        }

        private bool CheckAllUnsavedChanges()
        {
            foreach (var ctx in openTabs.Values.ToList())
            {
                if (ctx.IsModified)
                {
                    if (tabView != null) tabView.SelectedTab = ctx.Tab;
                    if (!CheckUnsavedChanges(ctx))
                        return false;
                }
            }
            return true;
        }
    }
}