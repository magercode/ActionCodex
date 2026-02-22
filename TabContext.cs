using Terminal.Gui;

namespace ActionCodex
{
    public class TabContext
    {
        public TabView.Tab Tab { get; set; } = null!;
        public TextView Editor { get; set; } = null!;
        public TextView LineNumbers { get; set; } = null!;
        public string FilePath { get; set; } = "";
        public bool IsModified { get; set; }
    }
}