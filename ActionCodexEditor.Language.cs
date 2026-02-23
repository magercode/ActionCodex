using Terminal.Gui;

namespace ActionCodex
{
    public partial class ActionCodexEditor
    {
        private void ChangeLanguage(string languageCode)
        {
            currentLanguage = languageCode;

            if (menuBar != null)
            {
                Remove(menuBar);
                InitializeMenu();
            }

            if (statusLabel != null)
            {
                statusLabel.Text = GetLocalizedString("StatusLanguageChanged");
            }

            if (positionLabel != null)
            {
                var ctx = GetCurrentTabContext();
                if (ctx != null)
                {
                    var cursor = ctx.Editor.CursorPosition;
                    positionLabel.Text = GetLocalizedString("PositionFormat")
                        .Replace("{0}", (cursor.Y + 1).ToString())
                        .Replace("{1}", (cursor.X + 1).ToString());
                }
            }

            UpdateTitle();
            SetNeedsDisplay();
        }

        private string GetLocalizedString(string key)
        {
            return currentLanguage switch
            {
                "ID" => GetIndonesianString(key),
                "RU" => GetRussianString(key),
                "DE" => GetGermanString(key),
                _ => GetEnglishString(key),
            };
        }

        private string GetEnglishString(string key)
        {
            return key switch
            {
                "WindowTitle" => "Action Codex - Editor",

                "MenuFile" => "_File",
                "MenuEdit" => "_Edit",
                "MenuView" => "_View",
                "MenuHelp" => "_Help",

                "MenuNew" => "_New",
                "MenuOpen" => "_Open",
                "MenuSave" => "_Save",
                "MenuSaveAs" => "Save _As",
                "MenuCloseTab" => "_Close Tab",
                "MenuQuit" => "_Quit",

                "MenuCopy" => "_Copy",
                "MenuCut" => "C_ut",
                "MenuPaste" => "_Paste",
                "MenuSelectAll" => "_Select All",

                "MenuWordWrap" => "_Word Wrap",
                "MenuColorThemes" => "_Color Themes",
                "MenuLanguage" => "_Language",

                "MenuAbout" => "_About",
                "MenuShortcuts" => "_Shortcuts",

                "ThemeTokyoMint" => "_Tokyo Mint",
                "ThemeDarkMode" => "_Dark Mode",
                "ThemeLightMode" => "_Light Mode",
                "ThemeSolarizedDark" => "_Solarized Dark",
                "ThemeSolarizedLight" => "_Solarized Light",
                "ThemeMonokai" => "_Monokai",
                "ThemeDracula" => "_Dracula",
                "ThemeNord" => "_Nord",
                "ThemeGruvboxDark" => "_Gruvbox Dark",
                "ThemeGruvboxLight" => "_Gruvbox Light",

                "StatusReady" => "Ready",
                "StatusNew" => "~^N~ New",
                "StatusOpen" => "~^O~ Open",
                "StatusSave" => "~^S~ Save",
                "StatusHelp" => "~F1~ Help",
                "StatusQuit" => "~^Q~ Quit",
                "StatusLanguageChanged" => "Language changed",
                "PositionFormat" => "Ln {0}, Col {1}",

                "MsgUnsavedChanges" => "has unsaved changes. Do you want to save before continuing?",
                "MsgSave" => "Save",
                "MsgDiscard" => "Discard",
                "MsgCancel" => "Cancel",
                "MsgError" => "Error",
                "MsgOK" => "OK",

                "AboutTitle" => "About",
                "AboutVersion" => "Action Codex v1.0.1",
                "AboutGithub" => "Github: Github.com/MagerCode",
                "AboutCopyright" => "© 2026 All Rights Reserved",

                "ShortcutsTitle" => "Keyboard Shortcuts",
                "ShortcutsFileOps" => "File Operations:",
                "ShortcutsNew" => "  Ctrl+N  - New tab",
                "ShortcutsOpen" => "  Ctrl+O  - Open file in new tab",
                "ShortcutsSave" => "  Ctrl+S  - Save current tab",
                "ShortcutsClose" => "  Ctrl+W  - Close current tab",
                "ShortcutsQuit" => "  Ctrl+Q  - Quit editor",
                "ShortcutsEditing" => "Editing:",
                "ShortcutsCopy" => "  Ctrl+C  - Copy",
                "ShortcutsCut" => "  Ctrl+X  - Cut",
                "ShortcutsPaste" => "  Ctrl+V  - Paste",
                "ShortcutsDelete" => "  Del     - Delete",
                "ShortcutsSelectAll" => "  Ctrl+A  - Select all",
                "ShortcutsNavigation" => "Navigation:",
                "ShortcutsArrow" => "  Arrow Keys - Move cursor",
                "ShortcutsHomeEnd" => "  Home/End   - Line start/end",
                "ShortcutsPageUpDown" => "  PgUp/PgDn  - Page up/down",
                "ShortcutsOther" => "Other:",
                "ShortcutsF1" => "  F1         - This help",
                "ShortcutsTab" => "  Tab        - Insert tab",

                _ => key,
            };
        }

        private string GetIndonesianString(string key)
        {
            return key switch
            {
                "WindowTitle" => "Action Codex - Editor",

                "MenuFile" => "_Berkas",
                "MenuEdit" => "_Sunting",
                "MenuView" => "_Tampilan",
                "MenuHelp" => "_Bantuan",

                "MenuNew" => "_Baru",
                "MenuOpen" => "_Buka",
                "MenuSave" => "_Simpan",
                "MenuSaveAs" => "Simpan _Sebagai",
                "MenuCloseTab" => "_Tutup Tab",
                "MenuQuit" => "_Keluar",

                "MenuCopy" => "_Salin",
                "MenuCut" => "Po_tong",
                "MenuPaste" => "_Tempel",
                "MenuSelectAll" => "Pilih _Semua",

                "MenuWordWrap" => "_Bungkus Kata",
                "MenuColorThemes" => "_Tema Warna",
                "MenuLanguage" => "_Bahasa",

                "MenuAbout" => "_Tentang",
                "MenuShortcuts" => "Pintasan",

                "ThemeTokyoMint" => "_Tokyo Mint",
                "ThemeDarkMode" => "Mode _Gelap",
                "ThemeLightMode" => "Mode _Terang",
                "ThemeSolarizedDark" => "_Solarized Gelap",
                "ThemeSolarizedLight" => "_Solarized Terang",
                "ThemeMonokai" => "_Monokai",
                "ThemeDracula" => "_Dracula",
                "ThemeNord" => "_Nord",
                "ThemeGruvboxDark" => "Gruvbox _Gelap",
                "ThemeGruvboxLight" => "Gruvbox _Terang",

                "StatusReady" => "Siap",
                "StatusNew" => "~^N~ Baru",
                "StatusOpen" => "~^O~ Buka",
                "StatusSave" => "~^S~ Simpan",
                "StatusHelp" => "~F1~ Bantuan",
                "StatusQuit" => "~^Q~ Keluar",
                "StatusLanguageChanged" => "Bahasa diubah",
                "PositionFormat" => "Br {0}, Kl {1}",

                "MsgUnsavedChanges" => "memiliki perubahan yang belum disimpan. Simpan sebelum melanjutkan?",
                "MsgSave" => "Simpan",
                "MsgDiscard" => "Abaikan",
                "MsgCancel" => "Batal",
                "MsgError" => "Kesalahan",
                "MsgOK" => "OK",

                "AboutTitle" => "Tentang",
                "AboutVersion" => "Action Codex v1.0.1",
                "AboutGithub" => "Github: Github.com/MagerCode",
                "AboutCopyright" => "© 2026 Hak Cipta Dilindungi",

                "ShortcutsTitle" => "Pintasan Keyboard",
                "ShortcutsFileOps" => "Operasi Berkas:",
                "ShortcutsNew" => "  Ctrl+N  - Tab baru",
                "ShortcutsOpen" => "  Ctrl+O  - Buka berkas di tab baru",
                "ShortcutsSave" => "  Ctrl+S  - Simpan tab saat ini",
                "ShortcutsClose" => "  Ctrl+W  - Tutup tab saat ini",
                "ShortcutsQuit" => "  Ctrl+Q  - Keluar dari editor",
                "ShortcutsEditing" => "Penyuntingan:",
                "ShortcutsCopy" => "  Ctrl+C  - Salin",
                "ShortcutsCut" => "  Ctrl+X  - Potong",
                "ShortcutsPaste" => "  Ctrl+V  - Tempel",
                "ShortcutsDelete" => "  Del     - Hapus",
                "ShortcutsSelectAll" => "  Ctrl+A  - Pilih semua",
                "ShortcutsNavigation" => "Navigasi:",
                "ShortcutsArrow" => "  Tombol Panah - Gerakkan kursor",
                "ShortcutsHomeEnd" => "  Home/End   - Awal/akhir baris",
                "ShortcutsPageUpDown" => "  PgUp/PgDn  - Halaman atas/bawah",
                "ShortcutsOther" => "Lainnya:",
                "ShortcutsF1" => "  F1         - Bantuan ini",
                "ShortcutsTab" => "  Tab        - Sisipkan tab",

                _ => key,
            };
        }

        private string GetRussianString(string key)
        {
            return key switch
            {
                "WindowTitle" => "Action Codex — Редактор",

                "MenuFile" => "_Файл",
                "MenuEdit" => "_Правка",
                "MenuView" => "_Вид",
                "MenuHelp" => "_Справка",

                "MenuNew" => "_Новый",
                "MenuOpen" => "_Открыть",
                "MenuSave" => "_Сохранить",
                "MenuSaveAs" => "Сохранить _как",
                "MenuCloseTab" => "_Закрыть вкладку",
                "MenuQuit" => "_Выход",

                "MenuCopy" => "_Копировать",
                "MenuCut" => "_Вырезать",
                "MenuPaste" => "_Вставить",
                "MenuSelectAll" => "Выбрать _всё",

                "MenuWordWrap" => "_Перенос строк",
                "MenuColorThemes" => "_Цветовые темы",
                "MenuLanguage" => "_Язык",

                "MenuAbout" => "_О программе",
                "MenuShortcuts" => "_Горячие клавиши",

                "StatusReady" => "Готово",
                "StatusNew" => "~^N~ Новый",
                "StatusOpen" => "~^O~ Открыть",
                "StatusSave" => "~^S~ Сохранить",
                "StatusHelp" => "~F1~ Справка",
                "StatusQuit" => "~^Q~ Выход",
                "StatusLanguageChanged" => "Язык изменён",
                "PositionFormat" => "Стр {0}, Кол {1}",

                "MsgUnsavedChanges" => "имеет несохранённые изменения. Сохранить перед продолжением?",
                "MsgSave" => "Сохранить",
                "MsgDiscard" => "Отменить",
                "MsgCancel" => "Отмена",
                "MsgError" => "Ошибка",
                "MsgOK" => "OK",

                "AboutTitle" => "О программе",
                "AboutVersion" => "Action Codex v1.0.1",
                "AboutGithub" => "Github: Github.com/MagerCode",
                "AboutCopyright" => "© 2026 Все права защищены",

                "ThemeTokyoMint" => "_Tokyo Mint",
                "ThemeDarkMode" => "_Тёмный режим",
                "ThemeLightMode" => "_Светлый режим",
                "ThemeSolarizedDark" => "_Solarized тёмный",
                "ThemeSolarizedLight" => "_Solarized светлый",
                "ThemeMonokai" => "_Monokai",
                "ThemeDracula" => "_Dracula",
                "ThemeNord" => "_Nord",
                "ThemeGruvboxDark" => "_Gruvbox тёмный",
                "ThemeGruvboxLight" => "_Gruvbox светлый",
                _ => key,
            };
        }

        private string GetGermanString(string key)
        {
            return key switch
            {
                "WindowTitle" => "Action Codex – Editor",

                "MenuFile" => "_Datei",
                "MenuEdit" => "_Bearbeiten",
                "MenuView" => "_Ansicht",
                "MenuHelp" => "_Hilfe",

                "MenuNew" => "_Neu",
                "MenuOpen" => "_Öffnen",
                "MenuSave" => "_Speichern",
                "MenuSaveAs" => "Speichern _unter",
                "MenuCloseTab" => "_Tab schließen",
                "MenuQuit" => "_Beenden",

                "MenuCopy" => "_Kopieren",
                "MenuCut" => "_Ausschneiden",
                "MenuPaste" => "_Einfügen",
                "MenuSelectAll" => "_Alles auswählen",

                "MenuWordWrap" => "_Zeilenumbruch",
                "MenuColorThemes" => "_Farbschemata",
                "MenuLanguage" => "_Sprache",

                "MenuAbout" => "_Über",
                "MenuShortcuts" => "_Tastenkürzel",

                "ThemeTokyoMint" => "_Tokyo Mint",
                "ThemeDarkMode" => "_Dunkelmodus",
                "ThemeLightMode" => "_Hellmodus",
                "ThemeSolarizedDark" => "_Solarized Dunkel",
                "ThemeSolarizedLight" => "_Solarized Hell",
                "ThemeMonokai" => "_Monokai",
                "ThemeDracula" => "_Dracula",
                "ThemeNord" => "_Nord",
                "ThemeGruvboxDark" => "_Gruvbox Dunkel",
                "ThemeGruvboxLight" => "_Gruvbox Hell",

                "StatusReady" => "Bereit",
                "StatusNew" => "~^N~ Neu",
                "StatusOpen" => "~^O~ Öffnen",
                "StatusSave" => "~^S~ Speichern",
                "StatusHelp" => "~F1~ Hilfe",
                "StatusQuit" => "~^Q~ Beenden",
                "StatusLanguageChanged" => "Sprache geändert",
                "PositionFormat" => "Z {0}, Sp {1}",

                "MsgUnsavedChanges" => "hat ungespeicherte Änderungen. Vor dem Fortfahren speichern?",
                "MsgSave" => "Speichern",
                "MsgDiscard" => "Verwerfen",
                "MsgCancel" => "Abbrechen",
                "MsgError" => "Fehler",
                "MsgOK" => "OK",

                "AboutTitle" => "Über",
                "AboutVersion" => "Action Codex v1.0.1",
                "AboutGithub" => "Github: Github.com/MagerCode",
                "AboutCopyright" => "© 2026 Alle Rechte vorbehalten",

                "ShortcutsTitle" => "Tastenkombinationen",
                "ShortcutsFileOps" => "Datei-Operationen:",
                "ShortcutsNew" => "  Ctrl+N  - Neuer Tab",
                "ShortcutsOpen" => "  Ctrl+O  - Datei in neuem Tab öffnen",
                "ShortcutsSave" => "  Ctrl+S  - Aktuellen Tab speichern",
                "ShortcutsClose" => "  Ctrl+W  - Aktuellen Tab schließen",
                "ShortcutsQuit" => "  Ctrl+Q  - Editor beenden",
                "ShortcutsEditing" => "Bearbeitung:",
                "ShortcutsCopy" => "  Ctrl+C  - Kopieren",
                "ShortcutsCut" => "  Ctrl+X  - Ausschneiden",
                "ShortcutsPaste" => "  Ctrl+V  - Einfügen",
                "ShortcutsDelete" => "  Entf    - Löschen",
                "ShortcutsSelectAll" => "  Ctrl+A  - Alles auswählen",
                "ShortcutsNavigation" => "Navigation:",
                "ShortcutsArrow" => "  Pfeiltasten - Cursor bewegen",
                "ShortcutsHomeEnd" => "  Pos1/Ende  - Zeilenanfang/-ende",
                "ShortcutsPageUpDown" => "  Bild Auf/Ab - Seite hoch/runter",
                "ShortcutsOther" => "Sonstiges:",
                "ShortcutsF1" => "  F1          - Diese Hilfe",
                "ShortcutsTab" => "  Tab         - Tabulator einfügen",

                _ => key,
            };
        }
    }
}