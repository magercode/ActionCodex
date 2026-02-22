using Terminal.Gui;

namespace ActionCodex
{
    class Program
    {
        static void Main(string[] args)
        {
            Application.Init();
            
            try
            {
                var edi = new TextEditorWindow();
                Application.Run(edi);
            }
            finally
            {
                Application.Shutdown();
            }
        }
    }
}