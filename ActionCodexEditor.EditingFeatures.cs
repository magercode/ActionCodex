using Terminal.Gui;

namespace ActionCodex
{
    public partial class ActionCodexEditor
    {
        private void HandleKeyDown(View.KeyEventEventArgs e, TabContext ctx)
        {
            if ((e.KeyEvent.IsCtrl && e.KeyEvent.Key == Key.S))
            {
                SaveFile();
                e.Handled = true;
            }
            else if ((e.KeyEvent.IsCtrl && e.KeyEvent.Key == Key.N))
            {
                NewFile();
                e.Handled = true;
            }
            else if ((e.KeyEvent.IsCtrl && e.KeyEvent.Key == Key.O))
            {
                OpenFile();
                e.Handled = true;
            }
            else if ((e.KeyEvent.IsCtrl && e.KeyEvent.Key == Key.W))
            {
                CloseFile();
                e.Handled = true;
            }
        }

        private void ToggleWordWrap()
        {
            var ctx = GetCurrentTabContext();
            if (ctx != null)
            {
                ctx.Editor.WordWrap = !ctx.Editor.WordWrap;
                if (statusLabel != null)
                    statusLabel.Text = ctx.Editor.WordWrap ? "Word wrap ON" : "Word wrap OFF";
            }
        }
    }
}