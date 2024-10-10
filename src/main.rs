mod args;
mod csv;
mod excel;
mod io;
mod utils;

use args::{
    Clean, Count, Estimate, Excel2csv, Flatten, Frequency, Head, Headers, Sample, Search, Select,
    Slice, Sort, Split, Stats, Table, Tail, To, Unique,
};
use clap::{Parser, Subcommand};
use utils::cmd_desc::{
    CLEAN_DESC, COUNT_DESC, ESTIMATE_DESC, EXCEL2CSV_DESC, FLATTEN_DESC, FREQUENCY_DESC,
    HEADER_DESC, HEAD_DESC, SAMPLE_DESC, SEARCH_DESC, SELECT_DESC, SLICE_DESC, SORT_DESC,
    SPLIT_DESC, STATS_DESC, TABLE_DESC, TAIL_DESC, TO_DESC, UNIQUE_DESC,
};

#[derive(Parser)]
#[command(name = "rsv")]
#[command(author = "ribbondz@163.com")]
#[command(version = "0.4.14")]
#[command(
    about = "A Rust command line tool to parse small and large (>10G) CSV, TXT, and EXCEL files."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(
        about = "Show head n lines",
        override_help = HEAD_DESC
    )]
    Head(Head),
    #[command(
        about = "Show tail n lines",
        override_help = TAIL_DESC
    )]
    Tail(Tail),
    #[command(
        about = "Show file headers", 
        override_help = HEADER_DESC
    )]
    Headers(Headers),
    #[command(
        about="Print flattened records to view them one by one",
        override_help=FLATTEN_DESC
    )]
    Flatten(Flatten),
    #[command(
        about="Count the number of lines in a file, or number of files in a directory",
        override_help=COUNT_DESC
    )]
    Count(Count),
    #[command(
        about = "Fast estimate the number of lines in a file",
        override_help = ESTIMATE_DESC
    )]
    Estimate(Estimate),
    #[command(
        about = "Clean file with escape chars, default to \"", 
        override_help = CLEAN_DESC
    )]
    Clean(Clean),
    #[command(
        about = "Frequency table for column(s)",
        override_help=FREQUENCY_DESC
    )]
    Frequency(Frequency),
    #[command(
        about = "Split file into separate files sequentially or according to column value",
        override_help = SPLIT_DESC
    )]
    Split(Split),
    #[command(
        about = "Select rows and columns by filter",
        override_help=SELECT_DESC
    )]
    Select(Select),
    #[command(
        about = "Extract a slice of rows from file",
        override_help = SLICE_DESC
    )]
    Slice(Slice),
    #[command(
        about = "Search with regexes",
        override_help = SEARCH_DESC
    )]
    Search(Search),
    #[command(
        about = "Sort data by column(s)",
        override_help = SORT_DESC
    )]
    Sort(Sort),
    #[command(
        about = "Statistics for column(s), including min, max, mean, unique, null",
        override_help = STATS_DESC
    )]
    Stats(Stats),
    #[command(
        about = "Convert EXCEL to CSV", 
        override_help = EXCEL2CSV_DESC
    )]
    Excel2csv(Excel2csv),
    #[command(
        about = "Show data in an aligned table",
        override_help = TABLE_DESC
    )]
    Table(Table),
    #[command(
        about = "Save data to disk, can be one of TXT, CSV, TSV, XLSX or XLS",
        override_help = TO_DESC
    )]
    To(To),
    #[command(
        about = "Data sampling",
        override_help = SAMPLE_DESC
    )]
    Sample(Sample),
    #[command(
        about = "Drop duplicates of data",
        override_help = UNIQUE_DESC
    )]
    Unique(Unique),
}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Count(count) => count.run(),
        Commands::Estimate(estimate) => estimate.run(),
        Commands::Head(head) => head.run(),
        Commands::Tail(tail) => tail.run(),
        Commands::Headers(headers) => headers.run(),
        Commands::Clean(clean) => clean.run(),
        Commands::Frequency(frequency) => frequency.run(),
        Commands::Split(split) => split.run(),
        Commands::Select(select) => select.run(),
        Commands::Flatten(flatten) => flatten.run(),
        Commands::Slice(slice) => slice.run(),
        Commands::Stats(stats) => stats.run(),
        Commands::Excel2csv(excel2csv) => excel2csv.run(),
        Commands::Table(table) => table.run(),
        Commands::Search(search) => search.run(),
        Commands::Sort(sort) => sort.run(),
        Commands::To(to) => to.run(),
        Commands::Sample(sample) => sample.run(),
        Commands::Unique(unique) => unique.run(),
    }
}
