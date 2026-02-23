using Terminal.Gui;

namespace ActionCodex
{
    public partial class ActionCodexEditor
    {
        private void HandleKeyDown(View.KeyEventEventArgs e, TabContext ctx)
        {
            //fix: check if the Ctrl modifier is set AND the key is S
            if ((e.KeyEvent.Key & Key.CtrlMask) != 0 && (e.KeyEvent.Key & Key.S) == Key.S)
            {
                SaveFile();
                e.Handled = true;
            }
            else if ((e.KeyEvent.Key & Key.CtrlMask) != 0 && (e.KeyEvent.Key & Key.N) == Key.N)
            {
                NewFile();
                e.Handled = true;
            }
            else if ((e.KeyEvent.Key & Key.CtrlMask) != 0 && (e.KeyEvent.Key & Key.O) == Key.O)
            {
                OpenFile();
                e.Handled = true;
            }
            else if ((e.KeyEvent.Key & Key.CtrlMask) != 0 && (e.KeyEvent.Key & Key.W) == Key.W)
            {
                CloseFile();
                e.Handled = true;
            }
            else if ((e.KeyEvent.Key & Key.CtrlMask) != 0 && (e.KeyEvent.Key & Key.Q) == Key.Q)
            {
                Quit();
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