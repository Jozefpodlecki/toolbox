using Microsoft.Extensions.Logging;
using Microsoft.Extensions.Logging.Abstractions;
using Moq;
using Toolbox;

namespace ToolboxTests
{
    [TestClass]
    public class CsvSplitterTests
    {
        [TestMethod]
        public async Task ShouldFailOnMissingInput()
        {
            var options = new CsvSplitterOptions
            {
                InputPath = "missing.csv",
                OutputPath = "output.csv",
                IncludeHeader = true,
                LinesPerFile = 100
            };
            var splitter = new CsvSplitter(NullLogger.Instance, options);

            var result = await splitter.RunAsync();
            var failure = result as CsvSplitResult.Failure;
            Assert.IsNotNull(failure, "Result should be a failure type.");
            Assert.IsTrue(failure.ErrorMessage.Contains("Input file 'missing.csv' not found"));
        }

        [TestMethod]
        public async Task ShouldSplitCsv()
        {
            var options = new CsvSplitterOptions
            {
                InputPath = "CsvScenarios/without_header.csv",
                OutputPath = "output.csv",
                IncludeHeader = false,
                LinesPerFile = 100
            };

            
            var splitter = new CsvSplitter(NullLogger.Instance, options);

            var result = await splitter.RunAsync();
            var success = result as CsvSplitResult.Success;
            Assert.IsNotNull(success, "Result should be a success type.");
        }
    }
}