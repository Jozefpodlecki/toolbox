using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Hosting;
using Microsoft.Extensions.Logging;
using static Toolbox.CsvSplitter;

namespace Toolbox;

class Program
{
    static async Task Main(string[] args)
    {
        using IHost host = Host.CreateDefaultBuilder(args)
        .ConfigureServices(services =>
        {
            services.AddLogging();
            services.AddSingleton<CsvSplitter>((sp) =>
            {
                var options = new CsvSplitterOptions
                {
                    InputPath = args.Length > 0 ? args[0] : throw new ArgumentException("Input path is required."),
                    OutputPath = args.Length > 1 ? args[1] : throw new ArgumentException("Output path is required."),
                };

                var logger = sp.GetRequiredService<ILogger<CsvSplitter>>();
                var csvSplitter = new CsvSplitter(logger, options);
                return new CsvSplitter(logger, options);
            });
        })
        .Build();


        var csvSplitter = host.Services.GetRequiredService<CsvSplitter>();
        var result = await csvSplitter.RunAsync();

        switch (result)
        {
            case CsvSplitResult.Success:
                Console.WriteLine("CSV split successfully completed.");
                break;

            case CsvSplitResult.Failure failure:
                Console.WriteLine($"CSV split failed: {failure.ErrorMessage}");
                break;
        }
    }
}
