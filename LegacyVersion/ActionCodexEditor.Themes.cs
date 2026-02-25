using Terminal.Gui;

namespace ActionCodex
{
    public partial class ActionCodexEditor
    {
        private void ApplyTheme(string themeName)
        {
            if (Application.Driver == null) return;

            currentTheme = themeName;

            switch (themeName)
            {
                case "Tokyo Mint":
                    ApplyTokyoMintTheme();
                    break;
                case "Dark Mode":
                    ApplyDarkTheme();
                    break;
                case "Light Mode":
                    ApplyLightTheme();
                    break;
                case "Solarized Dark":
                    ApplySolarizedDarkTheme();
                    break;
                case "Solarized Light":
                    ApplySolarizedLightTheme();
                    break;
                case "Monokai":
                    ApplyMonokaiTheme();
                    break;
                case "Dracula":
                    ApplyDraculaTheme();
                    break;
                case "Nord":
                    ApplyNordTheme();
                    break;
                case "Gruvbox Dark":
                    ApplyGruvboxDarkTheme();
                    break;
                case "Gruvbox Light":
                    ApplyGruvboxLightTheme();
                    break;
            }

            SetNeedsDisplay();

            if (statusLabel != null)
                statusLabel.Text = $"Theme changed to {themeName}";
        }

        private void ApplyTokyoMintTheme()
        {
            var bgBase = Color.Black;
            var bgPanel = Color.DarkGray;
            var fgText = Color.BrightCyan;
            var fgMuted = Color.Cyan;
            var accentMint = Color.BrightGreen;
            var fgFocus = Color.Black;

            //base
            Colors.Base.Normal = Application.Driver.MakeAttribute(fgMuted, bgBase);
            Colors.Base.Focus = Application.Driver.MakeAttribute(fgText, bgBase);
            Colors.Base.HotNormal = Application.Driver.MakeAttribute(accentMint, bgBase);
            Colors.Base.HotFocus = Application.Driver.MakeAttribute(accentMint, bgBase);

            //panel
            Colors.Menu.Normal = Application.Driver.MakeAttribute(fgText, bgPanel);
            Colors.Menu.Focus = Application.Driver.MakeAttribute(fgFocus, fgText);
            Colors.Menu.HotNormal = Application.Driver.MakeAttribute(accentMint, bgPanel);
            Colors.Menu.HotFocus = Application.Driver.MakeAttribute(fgFocus, accentMint);

            //dialog
            Colors.Dialog.Normal = Application.Driver.MakeAttribute(fgText, bgPanel);
            Colors.Dialog.Focus = Application.Driver.MakeAttribute(fgFocus, fgText);
            Colors.Dialog.HotNormal = Application.Driver.MakeAttribute(accentMint, bgPanel);
            Colors.Dialog.HotFocus = Application.Driver.MakeAttribute(fgFocus, accentMint);

            //error
            Colors.Error.Normal = Application.Driver.MakeAttribute(Color.BrightRed, bgPanel);
            Colors.Error.Focus = Application.Driver.MakeAttribute(Color.White, Color.BrightRed);
            Colors.Error.HotNormal = Application.Driver.MakeAttribute(accentMint, bgPanel);
            Colors.Error.HotFocus = Application.Driver.MakeAttribute(Color.White, Color.BrightRed);
        }

        private void ApplyDarkTheme()
        {
            var bgBase = Color.Black;
            var bgPanel = Color.DarkGray;
            var fgText = Color.White;
            var fgMuted = Color.Gray;
            var accent = Color.BrightYellow;
            var fgFocus = Color.Black;

            Colors.Base.Normal = Application.Driver.MakeAttribute(fgMuted, bgBase);
            Colors.Base.Focus = Application.Driver.MakeAttribute(fgText, bgBase);
            Colors.Base.HotNormal = Application.Driver.MakeAttribute(accent, bgBase);
            Colors.Base.HotFocus = Application.Driver.MakeAttribute(accent, bgBase);

            Colors.Menu.Normal = Application.Driver.MakeAttribute(fgText, bgPanel);
            Colors.Menu.Focus = Application.Driver.MakeAttribute(fgFocus, fgText);
            Colors.Menu.HotNormal = Application.Driver.MakeAttribute(accent, bgPanel);
            Colors.Menu.HotFocus = Application.Driver.MakeAttribute(fgFocus, accent);

            Colors.Dialog.Normal = Application.Driver.MakeAttribute(fgText, bgPanel);
            Colors.Dialog.Focus = Application.Driver.MakeAttribute(fgFocus, fgText);
            Colors.Dialog.HotNormal = Application.Driver.MakeAttribute(accent, bgPanel);
            Colors.Dialog.HotFocus = Application.Driver.MakeAttribute(fgFocus, accent);

            Colors.Error.Normal = Application.Driver.MakeAttribute(Color.BrightRed, bgPanel);
            Colors.Error.Focus = Application.Driver.MakeAttribute(Color.White, Color.BrightRed);
            Colors.Error.HotNormal = Application.Driver.MakeAttribute(accent, bgPanel);
            Colors.Error.HotFocus = Application.Driver.MakeAttribute(Color.White, Color.BrightRed);
        }

        private void ApplyLightTheme()
        {
            var bgBase = Color.White;
            var bgPanel = Color.Gray;
            var fgText = Color.Black;
            var fgMuted = Color.DarkGray;
            var accent = Color.BrightBlue;
            var fgFocus = Color.White;

            Colors.Base.Normal = Application.Driver.MakeAttribute(fgMuted, bgBase);
            Colors.Base.Focus = Application.Driver.MakeAttribute(fgText, bgBase);
            Colors.Base.HotNormal = Application.Driver.MakeAttribute(accent, bgBase);
            Colors.Base.HotFocus = Application.Driver.MakeAttribute(accent, bgBase);

            Colors.Menu.Normal = Application.Driver.MakeAttribute(fgText, bgPanel);
            Colors.Menu.Focus = Application.Driver.MakeAttribute(fgFocus, fgText);
            Colors.Menu.HotNormal = Application.Driver.MakeAttribute(accent, bgPanel);
            Colors.Menu.HotFocus = Application.Driver.MakeAttribute(fgFocus, accent);

            Colors.Dialog.Normal = Application.Driver.MakeAttribute(fgText, bgPanel);
            Colors.Dialog.Focus = Application.Driver.MakeAttribute(fgFocus, fgText);
            Colors.Dialog.HotNormal = Application.Driver.MakeAttribute(accent, bgPanel);
            Colors.Dialog.HotFocus = Application.Driver.MakeAttribute(fgFocus, accent);

            Colors.Error.Normal = Application.Driver.MakeAttribute(Color.BrightRed, bgPanel);
            Colors.Error.Focus = Application.Driver.MakeAttribute(Color.White, Color.BrightRed);
            Colors.Error.HotNormal = Application.Driver.MakeAttribute(accent, bgPanel);
            Colors.Error.HotFocus = Application.Driver.MakeAttribute(Color.White, Color.BrightRed);
        }

        private void ApplySolarizedDarkTheme()
        {
            // Solarized Dark colors menggunakan warna yang tersedia di Terminal.Gui
            var bgBase = Color.Black;
            var bgPanel = Color.DarkGray;
            var fgText = Color.BrightCyan;
            var fgMuted = Color.Cyan;
            var accent = Color.BrightYellow;
            var fgFocus = Color.Black;

            Colors.Base.Normal = Application.Driver.MakeAttribute(fgMuted, bgBase);
            Colors.Base.Focus = Application.Driver.MakeAttribute(fgText, bgBase);
            Colors.Base.HotNormal = Application.Driver.MakeAttribute(accent, bgBase);
            Colors.Base.HotFocus = Application.Driver.MakeAttribute(accent, bgBase);

            Colors.Menu.Normal = Application.Driver.MakeAttribute(fgText, bgPanel);
            Colors.Menu.Focus = Application.Driver.MakeAttribute(fgFocus, fgText);
            Colors.Menu.HotNormal = Application.Driver.MakeAttribute(accent, bgPanel);
            Colors.Menu.HotFocus = Application.Driver.MakeAttribute(fgFocus, accent);

            Colors.Dialog.Normal = Application.Driver.MakeAttribute(fgText, bgPanel);
            Colors.Dialog.Focus = Application.Driver.MakeAttribute(fgFocus, fgText);
            Colors.Dialog.HotNormal = Application.Driver.MakeAttribute(accent, bgPanel);
            Colors.Dialog.HotFocus = Application.Driver.MakeAttribute(fgFocus, accent);

            Colors.Error.Normal = Application.Driver.MakeAttribute(Color.BrightRed, bgPanel);
            Colors.Error.Focus = Application.Driver.MakeAttribute(Color.White, Color.BrightRed);
            Colors.Error.HotNormal = Application.Driver.MakeAttribute(accent, bgPanel);
            Colors.Error.HotFocus = Application.Driver.MakeAttribute(Color.White, Color.BrightRed);
        }

        private void ApplySolarizedLightTheme()
        {
            // Solarized Light colors
            var bgBase = Color.White;
            var bgPanel = Color.Gray;
            var fgText = Color.Black;
            var fgMuted = Color.DarkGray;
            var accent = Color.BrightBlue;
            var fgFocus = Color.White;

            Colors.Base.Normal = Application.Driver.MakeAttribute(fgMuted, bgBase);
            Colors.Base.Focus = Application.Driver.MakeAttribute(fgText, bgBase);
            Colors.Base.HotNormal = Application.Driver.MakeAttribute(accent, bgBase);
            Colors.Base.HotFocus = Application.Driver.MakeAttribute(accent, bgBase);

            Colors.Menu.Normal = Application.Driver.MakeAttribute(fgText, bgPanel);
            Colors.Menu.Focus = Application.Driver.MakeAttribute(fgFocus, fgText);
            Colors.Menu.HotNormal = Application.Driver.MakeAttribute(accent, bgPanel);
            Colors.Menu.HotFocus = Application.Driver.MakeAttribute(fgFocus, accent);

            Colors.Dialog.Normal = Application.Driver.MakeAttribute(fgText, bgPanel);
            Colors.Dialog.Focus = Application.Driver.MakeAttribute(fgFocus, fgText);
            Colors.Dialog.HotNormal = Application.Driver.MakeAttribute(accent, bgPanel);
            Colors.Dialog.HotFocus = Application.Driver.MakeAttribute(fgFocus, accent);

            Colors.Error.Normal = Application.Driver.MakeAttribute(Color.BrightRed, bgPanel);
            Colors.Error.Focus = Application.Driver.MakeAttribute(Color.White, Color.BrightRed);
            Colors.Error.HotNormal = Application.Driver.MakeAttribute(accent, bgPanel);
            Colors.Error.HotFocus = Application.Driver.MakeAttribute(Color.White, Color.BrightRed);
        }

        private void ApplyMonokaiTheme()
        {
            // Monokai colors
            var bgBase = Color.Black;
            var bgPanel = Color.DarkGray;
            var fgText = Color.White;
            var fgMuted = Color.Gray;
            var accent = Color.BrightMagenta;
            var fgFocus = Color.Black;

            Colors.Base.Normal = Application.Driver.MakeAttribute(fgMuted, bgBase);
            Colors.Base.Focus = Application.Driver.MakeAttribute(fgText, bgBase);
            Colors.Base.HotNormal = Application.Driver.MakeAttribute(accent, bgBase);
            Colors.Base.HotFocus = Application.Driver.MakeAttribute(accent, bgBase);

            Colors.Menu.Normal = Application.Driver.MakeAttribute(fgText, bgPanel);
            Colors.Menu.Focus = Application.Driver.MakeAttribute(fgFocus, fgText);
            Colors.Menu.HotNormal = Application.Driver.MakeAttribute(accent, bgPanel);
            Colors.Menu.HotFocus = Application.Driver.MakeAttribute(fgFocus, accent);

            Colors.Dialog.Normal = Application.Driver.MakeAttribute(fgText, bgPanel);
            Colors.Dialog.Focus = Application.Driver.MakeAttribute(fgFocus, fgText);
            Colors.Dialog.HotNormal = Application.Driver.MakeAttribute(accent, bgPanel);
            Colors.Dialog.HotFocus = Application.Driver.MakeAttribute(fgFocus, accent);

            Colors.Error.Normal = Application.Driver.MakeAttribute(Color.BrightRed, bgPanel);
            Colors.Error.Focus = Application.Driver.MakeAttribute(Color.White, Color.BrightRed);
            Colors.Error.HotNormal = Application.Driver.MakeAttribute(accent, bgPanel);
            Colors.Error.HotFocus = Application.Driver.MakeAttribute(Color.White, Color.BrightRed);
        }

        private void ApplyDraculaTheme()
        {
            // Dracula colors
            var bgBase = Color.Black;
            var bgPanel = Color.DarkGray;
            var fgText = Color.White;
            var fgMuted = Color.Gray;
            var accent = Color.Magenta;
            var fgFocus = Color.Black;

            Colors.Base.Normal = Application.Driver.MakeAttribute(fgMuted, bgBase);
            Colors.Base.Focus = Application.Driver.MakeAttribute(fgText, bgBase);
            Colors.Base.HotNormal = Application.Driver.MakeAttribute(accent, bgBase);
            Colors.Base.HotFocus = Application.Driver.MakeAttribute(accent, bgBase);

            Colors.Menu.Normal = Application.Driver.MakeAttribute(fgText, bgPanel);
            Colors.Menu.Focus = Application.Driver.MakeAttribute(fgFocus, fgText);
            Colors.Menu.HotNormal = Application.Driver.MakeAttribute(accent, bgPanel);
            Colors.Menu.HotFocus = Application.Driver.MakeAttribute(fgFocus, accent);

            Colors.Dialog.Normal = Application.Driver.MakeAttribute(fgText, bgPanel);
            Colors.Dialog.Focus = Application.Driver.MakeAttribute(fgFocus, fgText);
            Colors.Dialog.HotNormal = Application.Driver.MakeAttribute(accent, bgPanel);
            Colors.Dialog.HotFocus = Application.Driver.MakeAttribute(fgFocus, accent);

            Colors.Error.Normal = Application.Driver.MakeAttribute(Color.BrightRed, bgPanel);
            Colors.Error.Focus = Application.Driver.MakeAttribute(Color.White, Color.BrightRed);
            Colors.Error.HotNormal = Application.Driver.MakeAttribute(accent, bgPanel);
            Colors.Error.HotFocus = Application.Driver.MakeAttribute(Color.White, Color.BrightRed);
        }

        private void ApplyNordTheme()
        {
            var bgBase = Color.Black;
            var bgPanel = Color.DarkGray;
            var fgText = Color.Cyan;
            var fgMuted = Color.Blue;
            var accent = Color.BrightCyan;
            var fgFocus = Color.Black;

            Colors.Base.Normal = Application.Driver.MakeAttribute(fgMuted, bgBase);
            Colors.Base.Focus = Application.Driver.MakeAttribute(fgText, bgBase);
            Colors.Base.HotNormal = Application.Driver.MakeAttribute(accent, bgBase);
            Colors.Base.HotFocus = Application.Driver.MakeAttribute(accent, bgBase);

            Colors.Menu.Normal = Application.Driver.MakeAttribute(fgText, bgPanel);
            Colors.Menu.Focus = Application.Driver.MakeAttribute(fgFocus, fgText);
            Colors.Menu.HotNormal = Application.Driver.MakeAttribute(accent, bgPanel);
            Colors.Menu.HotFocus = Application.Driver.MakeAttribute(fgFocus, accent);

            Colors.Dialog.Normal = Application.Driver.MakeAttribute(fgText, bgPanel);
            Colors.Dialog.Focus = Application.Driver.MakeAttribute(fgFocus, fgText);
            Colors.Dialog.HotNormal = Application.Driver.MakeAttribute(accent, bgPanel);
            Colors.Dialog.HotFocus = Application.Driver.MakeAttribute(fgFocus, accent);

            Colors.Error.Normal = Application.Driver.MakeAttribute(Color.BrightRed, bgPanel);
            Colors.Error.Focus = Application.Driver.MakeAttribute(Color.White, Color.BrightRed);
            Colors.Error.HotNormal = Application.Driver.MakeAttribute(accent, bgPanel);
            Colors.Error.HotFocus = Application.Driver.MakeAttribute(Color.White, Color.BrightRed);
        }

        private void ApplyGruvboxDarkTheme()
        {
            var bgBase = Color.Black;
            var bgPanel = Color.DarkGray;
            var fgText = Color.BrightYellow;
            var fgMuted = Color.Magenta;
            var accent = Color.BrightGreen;
            var fgFocus = Color.Black;

            Colors.Base.Normal = Application.Driver.MakeAttribute(fgMuted, bgBase);
            Colors.Base.Focus = Application.Driver.MakeAttribute(fgText, bgBase);
            Colors.Base.HotNormal = Application.Driver.MakeAttribute(accent, bgBase);
            Colors.Base.HotFocus = Application.Driver.MakeAttribute(accent, bgBase);

            Colors.Menu.Normal = Application.Driver.MakeAttribute(fgText, bgPanel);
            Colors.Menu.Focus = Application.Driver.MakeAttribute(fgFocus, fgText);
            Colors.Menu.HotNormal = Application.Driver.MakeAttribute(accent, bgPanel);
            Colors.Menu.HotFocus = Application.Driver.MakeAttribute(fgFocus, accent);

            Colors.Dialog.Normal = Application.Driver.MakeAttribute(fgText, bgPanel);
            Colors.Dialog.Focus = Application.Driver.MakeAttribute(fgFocus, fgText);
            Colors.Dialog.HotNormal = Application.Driver.MakeAttribute(accent, bgPanel);
            Colors.Dialog.HotFocus = Application.Driver.MakeAttribute(fgFocus, accent);

            Colors.Error.Normal = Application.Driver.MakeAttribute(Color.BrightRed, bgPanel);
            Colors.Error.Focus = Application.Driver.MakeAttribute(Color.White, Color.BrightRed);
            Colors.Error.HotNormal = Application.Driver.MakeAttribute(accent, bgPanel);
            Colors.Error.HotFocus = Application.Driver.MakeAttribute(Color.White, Color.BrightRed);
        }

        private void ApplyGruvboxLightTheme()
        {
            var bgBase = Color.White;
            var bgPanel = Color.Gray;
            var fgText = Color.Black;
            var fgMuted = Color.DarkGray;
            var accent = Color.BrightGreen;
            var fgFocus = Color.White;

            Colors.Base.Normal = Application.Driver.MakeAttribute(fgMuted, bgBase);
            Colors.Base.Focus = Application.Driver.MakeAttribute(fgText, bgBase);
            Colors.Base.HotNormal = Application.Driver.MakeAttribute(accent, bgBase);
            Colors.Base.HotFocus = Application.Driver.MakeAttribute(accent, bgBase);

            Colors.Menu.Normal = Application.Driver.MakeAttribute(fgText, bgPanel);
            Colors.Menu.Focus = Application.Driver.MakeAttribute(fgFocus, fgText);
            Colors.Menu.HotNormal = Application.Driver.MakeAttribute(accent, bgPanel);
            Colors.Menu.HotFocus = Application.Driver.MakeAttribute(fgFocus, accent);

            Colors.Dialog.Normal = Application.Driver.MakeAttribute(fgText, bgPanel);
            Colors.Dialog.Focus = Application.Driver.MakeAttribute(fgFocus, fgText);
            Colors.Dialog.HotNormal = Application.Driver.MakeAttribute(accent, bgPanel);
            Colors.Dialog.HotFocus = Application.Driver.MakeAttribute(fgFocus, accent);

            Colors.Error.Normal = Application.Driver.MakeAttribute(Color.BrightRed, bgPanel);
            Colors.Error.Focus = Application.Driver.MakeAttribute(Color.White, Color.BrightRed);
            Colors.Error.HotFocus = Application.Driver.MakeAttribute(Color.White, Color.BrightRed);
        }
    }
}