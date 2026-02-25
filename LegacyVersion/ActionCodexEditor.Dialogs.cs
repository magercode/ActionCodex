using Terminal.Gui;

namespace ActionCodex
{
    public partial class ActionCodexEditor
    {
        private void ShowAbout()
        {
            MessageBox.Query(
                GetLocalizedString("AboutTitle"),
                $"{GetLocalizedString("AboutVersion")}\n\n" +
                $"{GetLocalizedString("AboutGithub")}\n" +
                $"{GetLocalizedString("AboutCopyright")}",
                GetLocalizedString("MsgOK"));
        }

        private void ShowShortcuts()
        {
            var shortcuts = new Dialog(GetLocalizedString("ShortcutsTitle"))
            {
                Width = 50,
                Height = 20
            };

            var text = new Label
            {
                X = 1,
                Y = 1,
                Text =
                    $"{GetLocalizedString("ShortcutsFileOps")}\n" +
                    $"  {GetLocalizedString("ShortcutsNew")}\n" +
                    $"  {GetLocalizedString("ShortcutsOpen")}\n" +
                    $"  {GetLocalizedString("ShortcutsSave")}\n" +
                    $"  {GetLocalizedString("ShortcutsClose")}\n" +
                    $"  {GetLocalizedString("ShortcutsQuit")}\n\n" +
                    $"{GetLocalizedString("ShortcutsEditing")}\n" +
                    $"  {GetLocalizedString("ShortcutsCopy")}\n" +
                    $"  {GetLocalizedString("ShortcutsCut")}\n" +
                    $"  {GetLocalizedString("ShortcutsPaste")}\n" +
                    $"  {GetLocalizedString("ShortcutsDelete")}\n" +
                    $"  {GetLocalizedString("ShortcutsSelectAll")}\n\n" +
                    $"{GetLocalizedString("ShortcutsNavigation")}\n" +
                    $"  {GetLocalizedString("ShortcutsArrow")}\n" +
                    $"  {GetLocalizedString("ShortcutsHomeEnd")}\n" +
                    $"  {GetLocalizedString("ShortcutsPageUpDown")}\n\n" +
                    $"{GetLocalizedString("ShortcutsOther")}\n" +
                    $"  {GetLocalizedString("ShortcutsF1")}\n" +
                    $"  {GetLocalizedString("ShortcutsTab")}"
            };

            shortcuts.Add(text);

            var okBtn = new Button(GetLocalizedString("MsgOK"))
            {
                X = Pos.Center(),
                Y = Pos.AnchorEnd(2)
            };
            okBtn.Clicked += () => Application.RequestStop();
            shortcuts.Add(okBtn);

            Application.Run(shortcuts);
        }

        private void ShowError(string message)
        {
            MessageBox.ErrorQuery(GetLocalizedString("MsgError"), message, GetLocalizedString("MsgOK"));
        }

        private bool CheckUnsavedChanges(TabContext? ctx = null)
        {
            if (ctx == null) ctx = GetCurrentTabContext();
            if (ctx == null) return true;

            if (ctx.IsModified)
            {
                int result = MessageBox.Query(
                    GetLocalizedString("MsgUnsavedChanges"),
                    $"'{Path.GetFileName(ctx.FilePath)}' {GetLocalizedString("MsgUnsavedChanges")}",
                    GetLocalizedString("MsgSave"),
                    GetLocalizedString("MsgDiscard"),
                    GetLocalizedString("MsgCancel"));

                switch (result)
                {
                    case 0:
                        SaveFile();
                        return !ctx.IsModified;
                    case 1:
                        return true;
                    default:
                        return false;
                }
            }
            return true;
        }
    }
}