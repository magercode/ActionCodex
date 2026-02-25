using Terminal.Gui;

namespace ActionCodex
{
    public partial class ActionCodexEditor
    {
        private void InitializeComponent()
        {
            if (Application.Driver != null)
            {
                ApplyTheme(currentTheme);
            }

            Title = GetLocalizedString("WindowTitle");
            X = 0;
            Y = 1;
            Width = Dim.Fill();
            Height = Dim.Fill() - 1;
            InitializeMenu();
            InitializeTabView();
            InitializeStatusBar();
            UpdateTitle();
        }

        private readonly (string Key, string ThemeId)[] themes = {
      ("ThemeTokyoMint", "Tokyo Mint"),
      ("ThemeDarkMode", "Dark Mode"),
      ("ThemeLightMode", "Light Mode"),
      ("ThemeSolarizedDark", "Solarized Dark"),
      ("ThemeSolarizedLight", "Solarized Light"),
      ("ThemeMonokai", "Monokai"),
      ("ThemeDracula", "Dracula"),
      ("ThemeNord", "Nord"),
      ("ThemeGruvboxDark", "Gruvbox Dark"),
      ("ThemeGruvboxLight", "Gruvbox Light"),
    };

        private void InitializeMenu()
        {
            var themeMenuItems =
                themes
                    .Select(t => new MenuItem(GetLocalizedString(t.Key), "",
                                              () => ApplyTheme(t.ThemeId), null, null,
                                              Key.Null))
                    .ToArray();

            var languageMenuItems = new MenuItem[] {
        new("English", "", () => ChangeLanguage("EN"), null, null, Key.Null),
        new("Bahasa Indonesia", "", () => ChangeLanguage("ID"), null, null,
            Key.Null),
        new("Русский", "", () => ChangeLanguage("RU"), null, null, Key.Null),
        new("Deutsch", "", () => ChangeLanguage("DE"), null, null, Key.Null),
      };

            menuBar = new MenuBar
            {
                Menus = new MenuBarItem[] {
        new MenuBarItem(GetLocalizedString("MenuFile"),
                        new MenuItem[] {
                          new MenuItem(GetLocalizedString("MenuNew"), "",
                                       NewFile, null, null,
                                       Key.CtrlMask | Key.N),
                          new MenuItem(GetLocalizedString("MenuOpen"), "",
                                       OpenFile, null, null,
                                       Key.CtrlMask | Key.O),
                          new MenuItem(GetLocalizedString("MenuSave"), "",
                                       SaveFile, null, null,
                                       Key.CtrlMask | Key.S),
                          new MenuItem(GetLocalizedString("MenuSaveAs"), "",
                                       SaveFileAs, null, null,
                                       Key.CtrlMask | Key.ShiftMask | Key.S),
                          new MenuItem(
                              GetLocalizedString("MenuCloseTab"), "",
                              CloseFile, null, null, Key.CtrlMask | Key.W),
                          new MenuItem(GetLocalizedString("MenuQuit"), "",
                                       Quit, null, null, Key.CtrlMask | Key.Q),
                        }),
        new MenuBarItem(GetLocalizedString("MenuEdit"),
                        new MenuItem[] {
                          new MenuItem(
                              GetLocalizedString("MenuCopy"), "",
                              () => GetCurrentTabContext()?.Editor.Copy(), null,
                              null, Key.CtrlMask | Key.C),
                          new MenuItem(
                              GetLocalizedString("MenuCut"), "",
                              () => GetCurrentTabContext()?.Editor.Cut(), null,
                              null, Key.CtrlMask | Key.X),
                          new MenuItem(
                              GetLocalizedString("MenuPaste"), "",
                              () => GetCurrentTabContext()?.Editor.Paste(),
                              null, null, Key.CtrlMask | Key.V),
                          new MenuItem(
                              GetLocalizedString("MenuSelectAll"), "",
                              () => GetCurrentTabContext()?.Editor.SelectAll(),
                              null, null, Key.CtrlMask | Key.A),
                        }),
        new MenuBarItem(GetLocalizedString("MenuView"),
                        new MenuItem[] {
                          new MenuItem(GetLocalizedString("MenuWordWrap"), "",
                                       ToggleWordWrap),
                          new MenuBarItem(GetLocalizedString("MenuColorThemes"),
                                          themeMenuItems),
                          new MenuBarItem(GetLocalizedString("MenuLanguage"),
                                          languageMenuItems),
                        }),
        new MenuBarItem(GetLocalizedString("MenuHelp"),
                        new MenuItem[] {
                          new MenuItem(GetLocalizedString("MenuAbout"), "",
                                       ShowAbout),
                          new MenuItem(GetLocalizedString("MenuShortcuts"), "",
                                       ShowShortcuts, null, null, Key.F1),
                        })
      }
            };

            if (menuBar != null)
                Add(menuBar);
        }

        private void InitializeTabView()
        {
            tabView = new TabView
            {
                X = 0,
                Y = 1,
                Width = Dim.Fill(),
                Height = Dim.Fill() - 1,
            };

            tabView.SelectedTabChanged += (s, e) => { UpdateTitle(); };

            if (tabView != null)
                Add(tabView);

            CreateNewTab();
        }

        private void InitializeStatusBar()
        {
            statusLabel = new Label(GetLocalizedString("StatusReady"))
            {
                X = 0,
                Y = Pos.AnchorEnd(2),
                Width = 40,
                Height = 1
            };

            positionLabel = new Label(GetLocalizedString("PositionFormat")
                                          .Replace("{0}", "1")
                                          .Replace("{1}", "1"))
            {
                X = Pos.AnchorEnd(15),
                Y = Pos.AnchorEnd(2),
                Width = 15,
                Height = 1
            };

            statusBar = new StatusBar
            {
                Visible = true,
                Items =
                  new StatusItem[] {
                    new(Key.CtrlMask | Key.N,
                             GetLocalizedString("StatusNew"), NewFile),
                    new(Key.CtrlMask | Key.O,
                             GetLocalizedString("StatusOpen"), OpenFile),
                    new(Key.CtrlMask | Key.S,
                             GetLocalizedString("StatusSave"), SaveFile),
                    new(Key.F1, GetLocalizedString("StatusHelp"),
                             ShowShortcuts),
                    new(Key.CtrlMask | Key.Q,
                             GetLocalizedString("StatusQuit"), Quit),
                  }
            };

            Add(statusLabel);
            Add(positionLabel);
            Add(statusBar);

            Application.MainLoop?.AddTimeout(TimeSpan.FromMilliseconds(100),
                                             UpdatePositionDisplay);
        }
    }
}