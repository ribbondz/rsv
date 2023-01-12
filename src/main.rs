use clap::{Args, Parser, Subcommand};
use utils::{
    cmd_desc::{
        CLEAN_DESC, COUNT_DESC, ESTIMATE_DESC, FLATTEN_DESC, FREQUENCY_DESC, HEADER_DESC,
        HEAD_DESC, PARTITION_DESC, SELECT_DESC, SLICE_DESC, STATS_DESC,
    },
    file::is_excel,
    filename::full_path,
};

mod csv;
mod excel;
mod utils;

#[derive(Parser)]
#[command(name = "rsv")]
#[command(author = "ribbondz@163.com")]
#[command(version = "0.1")]
#[command(about = "A Rust command line tool to parse small and large (>10G) csv and txt files", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(
        about = "Show head n lines of CSV file.",
        override_help = HEAD_DESC
    )]
    Head(Head),
    #[command(
        about = "Show file headers", 
        override_help = HEADER_DESC
    )]
    Headers(Headers),
    #[command(
        about="Prints flattened records to view records one by one",
        override_help=FLATTEN_DESC
    )]
    Flatten(Flatten),
    #[command(
        about="Count the number of lines of CSV file, or number of files in directory",
        override_help=COUNT_DESC
    )]
    Count(Count),
    #[command(
        about = "Fast estimate the number of lines.",
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
        about = "Split file into separate files according to column value",
        override_help=PARTITION_DESC
    )]
    Partition(Partition),
    #[command(
        about = "Select rows and columns by filter",
        override_help=SELECT_DESC
    )]
    Select(Select),
    #[command(
        about = "Extract a slice of rows from CSV file.",
        override_help = SLICE_DESC
    )]
    Slice(Slice),
    #[command(
        about = "Statistics for column(s), including min, max, mean, unique, null.",
        override_help = STATS_DESC
    )]
    Stats(Stats),
}

#[derive(Debug, Args)]
struct Count {
    /// File to open
    filename: String,
    /// Whether the file has a header
    #[arg(long, default_value_t = false)]
    no_header: bool,
    /// Get the nth worksheet from Excel file
    #[arg(short, long, default_value_t = 0)]
    sheet: usize,
}

#[derive(Debug, Args)]
struct Estimate {
    /// File to open
    filename: String,
    /// Get the nth worksheet for an Excel file
    #[arg(short, long, default_value_t = 0)]
    sheet: usize,
}

#[derive(Debug, Args)]
struct Headers {
    /// File to open
    filename: String,
    /// Field separator
    #[arg(short, long, default_value_t = String::from(","))]
    sep: String,
    /// Get the nth worksheet
    #[arg(short, long, default_value_t = 0)]
    sheet: usize,
}

#[derive(Debug, Args)]
struct Slice {
    /// File to open
    filename: String,
    /// Start index of CSV
    #[arg(short, long, default_value_t = 0)]
    start: usize,
    /// End index of CSV
    #[arg(short, long)]
    end: Option<usize>,
    /// Slice length
    #[arg(short, long)]
    length: Option<usize>,
    /// Index for single record of CSV
    #[arg(short, long)]
    index: Option<usize>,
    /// Whether the file has a header
    #[arg(long, default_value_t = false)]
    no_header: bool,
    /// Export data to a current-file-slice.csv?
    #[arg(short = 'E', long, default_value_t = false)]
    export: bool,
}

#[derive(Debug, Args)]
struct Head {
    /// File to open
    filename: String,
    /// Separator
    #[arg(short, long, default_value_t = String::from(","))]
    sep: String,
    /// Whether the file has a header
    #[arg(long, default_value_t = false)]
    no_header: bool,
    /// Number of records to show
    #[arg(short, long, default_value_t = 20)]
    n: usize,
    /// print as a table
    #[arg(short, long, default_value_t = false)]
    tabled: bool,
    /// Get the nth worksheet
    #[arg(short, long, default_value_t = 0)]
    sheet: usize,
}

#[derive(Debug, Args)]
struct Flatten {
    /// File to open
    filename: String,
    /// Separator
    #[arg(short, long, default_value_t = String::from(","))]
    sep: String,
    /// Whether the file has a header
    #[arg(long, default_value_t = false)]
    no_header: bool,
    /// Line delimiter for printing
    #[arg(short, long, default_value_t = String::from("#"))]
    delimiter: String,
    /// Number of records to show, n=-1 to show all
    #[arg(short, long, default_value_t = 5)]
    n: i32,
}

#[derive(Debug, Args)]
struct Clean {
    /// File to open
    filename: String,
    /// Output file, default to current-file-cleaned.csv
    #[arg(short, long, default_value_t = String::from(""), hide_default_value=true)]
    output: String,
    /// Escape char to clean
    #[arg(short, long, default_value_t = String::from("\""))]
    escape: String,
}

#[derive(Debug, Args)]
struct Frequency {
    /// File to open
    filename: String,
    /// Separator
    #[arg(short, long, default_value_t = String::from(","))]
    sep: String,
    /// Whether the file has a header
    #[arg(long, default_value_t = false)]
    no_header: bool,
    /// Columns to generate frequency table
    #[arg(short, long, default_value_t = String::from("0"))]
    cols: String,
    /// Ascending order or not
    #[arg(short, long, default_value_t = false)]
    ascending: bool,
    /// Export result to a frequency.csv?
    #[arg(short, long, default_value_t = false, short_alias = 'E')]
    export: bool,
    /// Top N to keep in frequency table
    #[arg(short, long, default_value_t = -1)]
    n: i32,
    /// Get the nth worksheet
    #[arg(short, long, default_value_t = 0)]
    sheet: usize,
}

#[derive(Debug, Args)]
struct Partition {
    /// File to open
    filename: String,
    /// Separator
    #[arg(short, long, default_value_t = String::from(","))]
    sep: String,
    /// Whether the file has a header
    #[arg(long, default_value_t = false)]
    no_header: bool,
    /// Columns to generate frequency table
    #[arg(short, long, default_value_t = 0)]
    col: usize,
}

#[derive(Debug, Args)]
struct Select {
    /// File to open
    filename: String,
    /// Separator
    #[arg(short, long, default_value_t = String::from(","))]
    sep: String,
    /// Whether the file has a header
    #[arg(long, default_value_t = false)]
    no_header: bool,
    /// Columns to select, support syntax 0,1,3 or 0-4, including 4; Default to select all columns
    #[arg(short, long, default_value_t = String::from(""))]
    cols: String,
    /// Row filter, support syntax 0=a,b,c or 0=a,b&1=c,d; Default to None
    #[arg(short, long, default_value_t = String::from(""))]
    filter: String,
    /// Export results to a file named current-file-selected.csv?
    #[arg(short, long, default_value_t = false, short_alias = 'E')]
    export: bool,
}

#[derive(Debug, Args)]
struct Stats {
    /// File to open
    filename: String,
    /// Separator
    #[arg(short, long, default_value_t = String::from(","))]
    sep: String,
    /// Whether the file has a header
    #[arg(long, default_value_t = false)]
    no_header: bool,
    /// Columns to generate statistics, support syntax 0,1,3 or 0-4, including 4; Default to select all columns
    #[arg(short, long, default_value_t = String::from(""))]
    cols: String,
    /// Export results to a file named current-file-selected.csv?
    #[arg(short, long, default_value_t = false, short_alias = 'E')]
    export: bool,
}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Count(option) => {
            let path = full_path(&option.filename);
            if is_excel(&path) {
                excel::count::run(&path, option.sheet, option.no_header).unwrap();
            } else {
                csv::count::run(&option.filename, option.no_header).unwrap();
            }
        }
        Commands::Head(option) => {
            let path = full_path(&option.filename);
            if is_excel(&path) {
                excel::head::run(&path, option.sheet, option.no_header, option.n).unwrap();
            } else {
                csv::head::run(
                    &option.filename,
                    option.no_header,
                    &option.sep,
                    option.n,
                    option.tabled,
                )
                .unwrap();
            }
        }
        Commands::Headers(option) => {
            let path = full_path(&option.filename);
            if is_excel(&path) {
                excel::headers::run(&path, option.sheet).unwrap();
            } else {
                csv::headers::run(&option.filename, &option.sep).unwrap();
            }
        }
        Commands::Estimate(option) => {
            let path = full_path(&option.filename);
            if is_excel(&path) {
                excel::count::run(&path, option.sheet, true).unwrap();
            } else {
                csv::estimate::run(&option.filename).unwrap();
            }
        }
        Commands::Clean(option) => {
            let path = full_path(&option.filename);
            if is_excel(&path) {
                panic!("rsv clean is not workable for Excel file.")
            } else {
                csv::clean::run(&option.filename, &option.escape, &option.output).unwrap();
            }
        }
        Commands::Frequency(option) => {
            let path = full_path(&option.filename);
            if is_excel(&path) {
                excel::frequency::run(
                    &option.filename,
                    option.no_header,
                    option.sheet,
                    &option.cols,
                    option.ascending,
                    option.n,
                    option.export,
                )
                .unwrap();
            } else {
                csv::frequency::run(
                    &option.filename,
                    option.no_header,
                    &option.sep,
                    &option.cols,
                    option.ascending,
                    option.n,
                    option.export,
                )
                .unwrap();
            }
        }
        Commands::Partition(option) => {
            let path = full_path(&option.filename);
            if is_excel(&path) {
                panic!("rsv partition is not workable for Excel file.")
            } else {
                csv::partition::run(&option.filename, option.no_header, &option.sep, option.col)
                    .unwrap();
            }
        }
        Commands::Select(option) => {
            csv::select::run(
                &option.filename,
                option.no_header,
                &option.sep,
                &option.cols,
                &option.filter,
                option.export,
            )
            .unwrap();
        }
        Commands::Flatten(option) => {
            csv::flatten::run(
                &option.filename,
                option.no_header,
                &option.sep,
                &option.delimiter,
                option.n,
            )
            .unwrap();
        }
        Commands::Slice(option) => {
            csv::slice::run(
                &option.filename,
                option.no_header,
                option.start,
                option.end,
                option.length,
                option.index,
                option.export,
            )
            .unwrap();
        }
        Commands::Stats(option) => {
            csv::stats::run(
                &option.filename,
                &option.sep,
                option.no_header,
                &option.cols,
                option.export,
            )
            .unwrap();
        }
    }
}
