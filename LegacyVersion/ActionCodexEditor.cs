using Terminal.Gui;

namespace ActionCodex
{
    public partial class ActionCodexEditor : Window
    {
        private TabView? tabView;
        private MenuBar? menuBar;
        private StatusBar? statusBar;
        private Label? statusLabel;
        private Label? positionLabel;

        private Dictionary<TabView.Tab, TabContext> openTabs = new Dictionary<TabView.Tab, TabContext>();
        private int tabCounter = 1;

        private string currentTheme = "Tokyo Mint";
        private string currentLanguage = "EN"; 

        public ActionCodexEditor()
        {
            InitializeComponent();
        }
    }
}