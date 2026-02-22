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
                var editor = new TextEditorWindow();
                Application.Run(editor);
            }
            finally
            {
                Application.Shutdown();
            }
        }
    }
}