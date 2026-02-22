using System.Text.RegularExpressions;
using Terminal.Gui;

namespace ActionCodex
{
    public class TextEditorWindow : Window
    {
        private TabView? tabView;
        private MenuBar? menuBar;
        private StatusBar? statusBar;
        private Label? statusLabel;
        private Label? positionLabel;

        private Dictionary<TabView.Tab, TabContext> openTabs = new Dictionary<TabView.Tab, TabContext>();
        private int tabCounter = 1;

        private static readonly string[] CSharpKeywords = {
            "abstract", "as", "base", "bool", "break", "byte", "case", "catch", "char", "checked", "class", "const", "continue", "decimal", "default", "delegate", "do", "double", "else", "enum", "event", "explicit", "extern", "false", "finally", "fixed", "float", "for", "foreach", "goto", "if", "implicit", "in", "int", "interface", "internal", "is", "lock", "long", "namespace", "new", "null", "object", "operator", "out", "override", "params", "private", "protected", "public", "readonly", "ref", "return", "sbyte", "sealed", "short", "sizeof", "stackalloc", "static", "string", "struct", "switch", "this", "throw", "true", "try", "typeof", "uint", "ulong", "unchecked", "unsafe", "ushort", "using", "virtual", "void", "volatile", "while"
        };

        private static readonly string[] VBKeywords = {
            "AddHandler", "AddressOf", "Alias", "And", "AndAlso", "As", "Boolean", "ByRef", "Byte", "ByVal", "Call", "Case", "Catch", "CBool", "CByte", "CChar", "CDate", "CDec", "CDbl", "Char", "CInt", "Class", "CLng", "CObj", "Const", "Continue", "CSByte", "CShort", "CSng", "CStr", "CType", "CUInt", "CULng", "CUShort", "Date", "Decimal", "Declare", "Default", "Delegate", "Dim", "DirectCast", "Do", "Double", "Each", "Else", "ElseIf", "End", "EndIf", "Enum", "Erase", "Error", "Event", "Exit", "False", "Finally", "For", "Friend", "Function", "Get", "GetType", "GetXMLNamespace", "Global", "GoSub", "GoTo", "Handles", "If", "Implements", "Imports", "In", "Inherits", "Integer", "Interface", "Is", "IsNot", "Let", "Lib", "Like", "Long", "Loop", "Me", "Mod", "Module", "MustInherit", "MustOverride", "MyBase", "MyClass", "Namespace", "Narrowing", "New", "Next", "Not", "Nothing", "NotInheritable", "NotOverridable", "Object", "Of", "On", "Operator", "Option", "Optional", "Or", "OrElse", "Out", "Overloads", "Overridable", "Overrides", "ParamArray", "Partial", "Private", "Property", "Protected", "Public", "RaiseEvent", "ReadOnly", "ReDim", "REM", "RemoveHandler", "Resume", "Return", "SByte", "Select", "Set", "Shadows", "Shared", "Short", "Single", "Static", "Step", "Stop", "String", "Structure", "Sub", "SyncLock", "Then", "Throw", "To", "True", "Try", "TryCast", "TypeOf", "UInteger", "ULong", "Unit", "Until", "UShort", "Using", "Variant", "Wend", "When", "While", "Widening", "With", "WithEvents", "WriteOnly", "Xor"
        };

        private class TabContext
        {
            public TabView.Tab Tab { get; set; } = null!;
            public TextView Editor { get; set; } = null!;
            public TextView LineNumbers { get; set; } = null!;
            public string FilePath { get; set; } = "";
            public bool IsModified { get; set; }
        }

        public TextEditorWindow()
        {
            InitializeComponent();
        }

        private void InitializeComponent()
        {
            if (Application.Driver != null)
            {
                ApplyTokyoMintTheme();
            }

            Title = "Action Codex - Editor";
            X = 0;
            Y = 1;
            Width = Dim.Fill();
            Height = Dim.Fill() - 1;
            InitializeMenu();
            InitializeTabView();
            InitializeStatusBar();
            UpdateTitle();
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

        private void InitializeMenu()
        {
            menuBar = new MenuBar
            {
                Menus = new MenuBarItem[]
                {
                    new MenuBarItem("_File", new MenuItem[]
                    {
                        new MenuItem("_New", "", NewFile, null, null, Key.CtrlMask | Key.N),
                        new MenuItem("_Open", "", OpenFile, null, null, Key.CtrlMask | Key.O),
                        new MenuItem("_Save", "", SaveFile, null, null, Key.CtrlMask | Key.S),
                        new MenuItem("Save _As", "", SaveFileAs, null, null, Key.CtrlMask | Key.ShiftMask | Key.S),
                        new MenuItem("_Close Tab", "", CloseFile, null, null, Key.CtrlMask | Key.W),
                        new MenuItem("_Quit", "", Quit, null, null, Key.CtrlMask | Key.Q),
                    }),
                    new MenuBarItem("_Edit", new MenuItem[]
                    {
                        new MenuItem("_Copy", "", () => GetCurrentTabContext()?.Editor.Copy(), null, null, Key.CtrlMask | Key.C),
                        new MenuItem("C_ut", "", () => GetCurrentTabContext()?.Editor.Cut(), null, null, Key.CtrlMask | Key.X),
                        new MenuItem("_Paste", "", () => GetCurrentTabContext()?.Editor.Paste(), null, null, Key.CtrlMask | Key.V),
                        new MenuItem("_Select All", "", () => GetCurrentTabContext()?.Editor.SelectAll(), null, null, Key.CtrlMask | Key.A),
                    }),
                    new MenuBarItem("_View", new MenuItem[]
                    {
                        new MenuItem("_Word Wrap", "", ToggleWordWrap),
                    }),
                    new MenuBarItem("_Help", new MenuItem[]
                    {
                        new MenuItem("_About", "", ShowAbout),
                        new MenuItem("_Shortcuts", "", ShowShortcuts, null, null, Key.F1),
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

            tabView.SelectedTabChanged += (s, e) =>
            {
                UpdateTitle();
            };

            if (tabView != null)
                Add(tabView);

            CreateNewTab();
        }

        private void InitializeStatusBar()
        {
            statusLabel = new Label("Ready")
            {
                X = 0,
                Y = Pos.AnchorEnd(2),
                Width = 40,
                Height = 1
            };

            positionLabel = new Label("Ln 1, Col 1")
            {
                X = Pos.AnchorEnd(15),
                Y = Pos.AnchorEnd(2),
                Width = 15,
                Height = 1
            };

            statusBar = new StatusBar
            {
                Visible = true,
                Items = new StatusItem[]
                {
                    new StatusItem(Key.CtrlMask | Key.N, "~^N~ New", NewFile),
                    new StatusItem(Key.CtrlMask | Key.O, "~^O~ Open", OpenFile),
                    new StatusItem(Key.CtrlMask | Key.S, "~^S~ Save", SaveFile),
                    new StatusItem(Key.F1, "~F1~ Help", ShowShortcuts),
                    new StatusItem(Key.CtrlMask | Key.Q, "~^Q~ Quit", Quit),
                }
            };

            Add(statusLabel);
            Add(positionLabel);
            Add(statusBar);

            Application.MainLoop?.AddTimeout(TimeSpan.FromMilliseconds(100), UpdatePositionDisplay);
        }

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

        private void HandleKeyDown(View.KeyEventEventArgs e, TabContext ctx)
        {
            if (e.KeyEvent.Key == Key.Enter)
            {
                e.Handled = true;
                HandleSmartIndent(new TabContext
                {
                    Editor = ctx.Editor,
                    FilePath = ctx.FilePath,
                    IsModified = ctx.IsModified,
                    LineNumbers = ctx.LineNumbers,
                    Tab = ctx.Tab
                });
            }
            else if ((e.KeyEvent.IsCtrl && e.KeyEvent.Key == Key.Space))
            {
                e.Handled = true;
                TriggerAutocomplete();
            }
            else if ((e.KeyEvent.IsCtrl && e.KeyEvent.Key == Key.S))
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

        private void HandleSmartIndent(TabContext ctx)
        {
            var editor = ctx.Editor;
            var cursor = editor.CursorPosition;
            var textStr = editor.Text.ToString() ?? "";
            var lines = textStr.Split('\n');

            if (cursor.Y < lines.Length)
            {
                string currentLine = lines[cursor.Y];
                string indentation = Regex.Match(currentLine, @"^[ \t]*").Value;
                bool increaseIndent = currentLine.TrimEnd().EndsWith("{") ||
                                     (currentLine.Trim().StartsWith("if", StringComparison.OrdinalIgnoreCase) &&
                                      currentLine.TrimEnd().EndsWith("then", StringComparison.OrdinalIgnoreCase));

                bool decreaseIndent = currentLine.Trim() == "}" || currentLine.Trim().Equals("End If", StringComparison.OrdinalIgnoreCase);
                string newLineIndent = indentation;
                if (increaseIndent)
                    newLineIndent += "    ";
                else if (decreaseIndent && indentation.Length >= 4)
                    newLineIndent = indentation.Substring(0, indentation.Length - 4);

                editor.InsertText(newLineIndent);
            }
        }

        private void TriggerAutocomplete()
        {
            var ctx = GetCurrentTabContext();
            if (ctx == null) return;
            string text = ctx.Editor.Text.ToString() ?? "";
            string currentWord = GetWordAtCursor(text, ctx.Editor.CursorPosition);
            if (string.IsNullOrEmpty(currentWord) && !text.EndsWith(".")) return;
            string ext = Path.GetExtension(ctx.FilePath).ToLower();
            string[] keywords = (ext == ".vb") ? VBKeywords : CSharpKeywords;
            var suggestions = keywords.Where(k => k.StartsWith(currentWord, StringComparison.OrdinalIgnoreCase)).Take(10).ToList();
            if (suggestions.Any()) ShowAutocompleteMenu(suggestions, currentWord, ctx);
        }

        private string GetWordAtCursor(string text, Point cursor)
        {
            var lines = text.Split('\n');
            if (cursor.Y >= lines.Length) return "";
            string line = lines[cursor.Y].Substring(0, Math.Min(cursor.X, lines[cursor.Y].Length));
            var match = Regex.Match(line, @"(\w+)$");
            return match.Success ? match.Value : "";
        }

        private void ShowAutocompleteMenu(List<string> suggestions, string partial, TabContext ctx)
        {
            if (ctx == null || suggestions.Count == 0) return;

            var popup = new Window("Suggestions")
            {
                X = Math.Min(ctx.Editor.CursorPosition.X, ctx.Editor.Bounds.Width - 20),
                Y = ctx.Editor.CursorPosition.Y + 1,
                Width = 20,
                Height = Math.Min(suggestions.Count + 2, 10),
                CanFocus = true,
                ColorScheme = Colors.Dialog
            };

            var listView = new ListView(suggestions)
            {
                Width = Dim.Fill(),
                Height = Dim.Fill(),
                AllowsMarking = false
            };

            popup.Add(listView);

            listView.OpenSelectedItem += (args) =>
            {
                InsertAutocomplete(ctx, partial, suggestions[listView.SelectedItem]);
                Application.RequestStop();
            };

            popup.KeyDown += (e) =>
            {
                if (e.KeyEvent.Key == Key.Esc)
                    Application.RequestStop();
            };

            Application.Run(popup);
        }

        private void InsertAutocomplete(TabContext ctx, string partial, string completion)
        {
            for (int i = 0; i < partial.Length; i++)
                ctx.Editor.ProcessKey(new KeyEvent(Key.Backspace, new KeyModifiers()));

            ctx.Editor.InsertText(completion);
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
                Title = "Action Codex";
            }
        }

        private bool UpdatePositionDisplay(MainLoop caller)
        {
            var ctx = GetCurrentTabContext();
            if (ctx != null && positionLabel != null)
            {
                var cursor = ctx.Editor.CursorPosition;
                positionLabel.Text = $"Ln {cursor.Y + 1}, Col {cursor.X + 1}";
            }
            return true;
        }

        private void NewFile()
        {
            CreateNewTab();
            if (statusLabel != null)
                statusLabel.Text = "New file created";
        }

        private void OpenFile()
        {
            var dialog = new OpenDialog("Open File", "Open a text file")
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

            var dialog = new SaveDialog("Save File As", "Save the text file")
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
                    statusLabel.Text = "Tab closed";
            }
        }

        private bool CheckUnsavedChanges(TabContext? ctx = null)
        {
            if (ctx == null) ctx = GetCurrentTabContext();
            if (ctx == null) return true;

            if (ctx.IsModified)
            {
                int result = MessageBox.Query("Unsaved Changes",
                    $"'{Path.GetFileName(ctx.FilePath)}' has unsaved changes. Do you want to save before continuing?",
                    "Save", "Discard", "Cancel");

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

        private void Quit()
        {
            if (CheckAllUnsavedChanges())
            {
                Application.RequestStop();
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

        private void ShowAbout()
        {
            MessageBox.Query("About",
                "Action Codex v1.0\n\n" +
                "Github: Github.com/MagerCode\n" +
                "© 2026 All Rights Reserved",
                "OK");
        }

        private void ShowShortcuts()
        {
            var shortcuts = new Dialog("Keyboard Shortcuts")
            {
                Width = 50,
                Height = 20
            };

            var text = new Label
            {
                X = 1,
                Y = 1,
                Text =
                    "File Operations:\n" +
                    "  Ctrl+N  - New tab\n" +
                    "  Ctrl+O  - Open file in new tab\n" +
                    "  Ctrl+S  - Save current tab\n" +
                    "  Ctrl+W  - Close current tab\n" +
                    "  Ctrl+Q  - Quit editor\n\n" +
                    "Editing:\n" +
                    "  Ctrl+C  - Copy\n" +
                    "  Ctrl+X  - Cut\n" +
                    "  Ctrl+V  - Paste\n" +
                    "  Del     - Delete\n" +
                    "  Ctrl+A  - Select all\n\n" +
                    "Navigation:\n" +
                    "  Arrow Keys - Move cursor\n" +
                    "  Home/End   - Line start/end\n" +
                    "  PgUp/PgDn  - Page up/down\n\n" +
                    "Other:\n" +
                    "  F1         - This help\n" +
                    "  Tab        - Insert tab"
            };

            shortcuts.Add(text);

            var okBtn = new Button("OK")
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
            MessageBox.ErrorQuery("Error", message, "OK");
        }
    }
}