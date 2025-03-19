using Microsoft.Extensions.Logging;

namespace Toolbox;

public sealed class CsvSplitter
{
    private readonly ILogger _logger;
    private readonly CsvSplitterOptions _options;

    public CsvSplitter(ILogger logger, CsvSplitterOptions options)
    {
        _logger = logger;
        _options = options;  
    }

    public async Task<CsvSplitResult> RunAsync() {

        if (!File.Exists(_options.InputPath))
        {
            return new CsvSplitResult.Failure($"Input file '{_options.InputPath}' not found.");
        }

        if (!Directory.Exists(_options.OutputPath))
        {
            Directory.CreateDirectory(_options.OutputPath);
        }

        try
        {
            using var reader = new StreamReader(_options.InputPath);
            string? header = null;
            int fileIndex = 1;
            int lineCount = 0;
            var buffer = new List<string>(_options.LinesPerFile);

            if (_options.IncludeHeader && !reader.EndOfStream)
            {
                header = await reader.ReadLineAsync();
            }

            while (!reader.EndOfStream)
            {
                string? line = await reader.ReadLineAsync();
                if (line == null)
                {
                    continue;
                }

                buffer.Add(line);
                lineCount++;

                if (lineCount >= _options.LinesPerFile)
                {
                    await WriteChunkToFileAsync(buffer, fileIndex++, header);
                    buffer.Clear();
                    lineCount = 0;
                }
            }

            if (buffer.Any())
            {
                await WriteChunkToFileAsync(buffer, fileIndex++, header);
            }

            _logger.LogInformation("CSV splitting completed successfully.");
            return new CsvSplitResult.Success();
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error processing CSV file.");
            return new CsvSplitResult.Failure($"An error occurred: {ex.Message}");
        }
    }

    private async Task WriteChunkToFileAsync(IEnumerable<string> lines, int fileIndex, string? header)
    {
        var outputFilePath = Path.Combine(_options.OutputPath, $"split_{fileIndex}.csv");

        await using var writer = new StreamWriter(outputFilePath);

        if (_options.IncludeHeader && header != null)
        {
            await writer.WriteLineAsync(header);
        }

        foreach (var line in lines)
        {
            await writer.WriteLineAsync(line);
        }

        _logger.LogInformation($"Written {lines.Count()} lines to {outputFilePath}");
    }
}
