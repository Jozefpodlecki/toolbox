namespace Toolbox
{
    public abstract class CsvSplitResult
    {
        public sealed class Success : CsvSplitResult { }

        public sealed class Failure : CsvSplitResult
        {
            public string ErrorMessage { get; }

            public Failure(string errorMessage)
            {
                ErrorMessage = errorMessage;
            }
        }
    }
}
