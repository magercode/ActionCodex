using Terminal.Gui;

namespace ActionCodex
{
    public class TabContext
    {
        public required TabView.Tab Tab { get; set; }
        public required TextView Editor { get; set; }
        public required TextView LineNumbers { get; set; }
        public required string FilePath { get; set; }
        public bool IsModified { get; set; }
        public bool WaitingForCommand { get; set; }
        public char? PendingCommand { get; set; }
    }
}