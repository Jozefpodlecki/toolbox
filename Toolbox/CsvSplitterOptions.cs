namespace Toolbox
{
    public class CsvSplitterOptions
    {
        public string InputPath { get; set; } = string.Empty;
        public string OutputPath { get; set; } = string.Empty;
        public int LinesPerFile { get; set; } = 1000;
        public bool IncludeHeader { get; set; } = true;
    }
}
