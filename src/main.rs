use clap::{Args, Parser, Subcommand};
use utils::{
    cmd_desc::{
        CLEAN_DESC, COUNT_DESC, ESTIMATE_DESC, EXCEL2CSV_DESC, FLATTEN_DESC, FREQUENCY_DESC,
        HEADER_DESC, HEAD_DESC, SEARCH_DESC, SELECT_DESC, SLICE_DESC, SPLIT_DESC, STATS_DESC,
        TABLE_DESC,
    },
    file::is_excel,
    filename::full_path,
    util::werr,
};

mod csv;
mod excel;
mod utils;

#[derive(Parser)]
#[command(name = "rsv")]
#[command(author = "ribbondz@163.com")]
#[command(version = "0.2")]
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
        about="Count the number of lines in a file, or number of files in a directory",
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
        override_help = SPLIT_DESC
    )]
    Split(Split),
    #[command(
        about = "Select rows and columns by filter",
        override_help=SELECT_DESC
    )]
    Select(Select),
    #[command(
        about = "Extract a slice of rows from file.",
        override_help = SLICE_DESC
    )]
    Slice(Slice),
    #[command(
        about = "Search with regexes",
        override_help = SEARCH_DESC
    )]
    Search(Search),
    #[command(
        about = "Statistics for column(s), including min, max, mean, unique, null.",
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
}

#[derive(Debug, Args)]
struct Count {
    /// File to open
    filename: String,
    /// Whether the file has a header
    #[arg(long, default_value_t = false)]
    no_header: bool,
    /// Get the nth worksheet of Excel file
    #[arg(short = 'S', long, default_value_t = 0)]
    sheet: usize,
}

#[derive(Debug, Args)]
struct Estimate {
    /// File to open
    filename: String,
    /// Get the nth worksheet for an Excel file
    #[arg(short = 'S', long, default_value_t = 0)]
    sheet: usize,
}

#[derive(Debug, Args)]
struct Headers {
    /// File to open
    filename: String,
    /// Field separator
    #[arg(short, long, default_value_t = String::from(","))]
    sep: String,
    /// Get the nth worksheet of EXCEL file
    #[arg(short = 'S', long, default_value_t = 0)]
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
    /// Get the nth worksheet of EXCEL file
    #[arg(short = 'S', long, default_value_t = 0)]
    sheet: usize,
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
    /// Get the nth worksheet of EXCEL file
    #[arg(short = 'S', long, default_value_t = 0)]
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
    /// Get the nth worksheet of EXCEL file
    #[arg(short = 'S', long, default_value_t = 0)]
    sheet: usize,
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
    #[arg(short = 'E', long, default_value_t = false)]
    export: bool,
    /// Top N to keep in frequency table
    #[arg(short, long, default_value_t = -1)]
    n: i32,
    /// Get the nth worksheet of EXCEL file
    #[arg(short = 'S', long, default_value_t = 0)]
    sheet: usize,
}

#[derive(Debug, Args)]
struct Split {
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
    /// Get the nth worksheet of EXCEL file
    #[arg(short = 'S', long, default_value_t = 0)]
    sheet: usize,
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
    #[arg(short = 'E', long, default_value_t = false)]
    export: bool,
    /// Get the nth worksheet of EXCEL file
    #[arg(short = 'S', long, default_value_t = 0)]
    sheet: usize,
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
    #[arg(short = 'E', long, default_value_t = false)]
    export: bool,
    /// Get the nth worksheet of EXCEL file
    #[arg(short = 'S', long, default_value_t = 0)]
    sheet: usize,
}

#[derive(Debug, Args)]
struct Excel2csv {
    /// File to open
    filename: String,
    /// Get the nth worksheet of EXCEL file
    #[arg(short = 'S', long, default_value_t = 0)]
    sheet: usize,
    /// Separator
    #[arg(short, long, default_value_t = String::from(","))]
    sep: String,
}

#[derive(Debug, Args)]
struct Table {
    /// Separator
    #[arg(short, long, default_value_t = String::from(","))]
    sep: String,
}

#[derive(Debug, Args)]
struct Search {
    /// Regex pattern to search
    pattern: String,
    /// File to open
    filename: String,
    /// Whether the file has a header
    #[arg(long, default_value_t = false)]
    no_header: bool,
    /// Get the nth worksheet of EXCEL file
    #[arg(short = 'S', long, default_value_t = 0)]
    sheet: usize,
    /// Export to a file named current-file-searched.csv?
    #[arg(short = 'E', long, default_value_t = false)]
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
                match excel::count::run(&path, option.sheet, option.no_header) {
                    Ok(()) => {}
                    Err(msg) => werr!("Error: {}", msg),
                };
            } else {
                match csv::count::run(&option.filename, option.no_header) {
                    Ok(()) => {}
                    Err(msg) => werr!("Error: {}", msg),
                };
            }
        }
        Commands::Head(option) => {
            let path = full_path(&option.filename);
            if is_excel(&path) {
                match excel::head::run(&path, option.sheet, option.no_header, option.n) {
                    Ok(()) => {}
                    Err(msg) => werr!("Error: {}", msg),
                };
            } else {
                match csv::head::run(
                    &option.filename,
                    option.no_header,
                    &option.sep,
                    option.n,
                    option.tabled,
                ) {
                    Ok(()) => {}
                    Err(msg) => werr!("Error: {}", msg),
                };
            }
        }
        Commands::Headers(option) => {
            let path = full_path(&option.filename);
            if is_excel(&path) {
                match excel::headers::run(&path, option.sheet) {
                    Ok(()) => {}
                    Err(msg) => werr!("Error: {}", msg),
                };
            } else {
                match csv::headers::run(&option.filename, &option.sep) {
                    Ok(()) => {}
                    Err(msg) => werr!("Error: {}", msg),
                };
            }
        }
        Commands::Estimate(option) => {
            let path = full_path(&option.filename);
            if is_excel(&path) {
                match excel::count::run(&path, option.sheet, true) {
                    Ok(()) => {}
                    Err(msg) => werr!("Error: {}", msg),
                };
            } else {
                match csv::estimate::run(&option.filename) {
                    Ok(()) => {}
                    Err(msg) => werr!("Error: {}", msg),
                };
            }
        }
        Commands::Clean(option) => {
            let path = full_path(&option.filename);
            if is_excel(&path) {
                werr!("rsv clean does not support Excel files.")
            } else {
                match csv::clean::run(&option.filename, &option.escape, &option.output) {
                    Ok(()) => {}
                    Err(msg) => werr!("Error: {}", msg),
                };
            }
        }
        Commands::Frequency(option) => {
            let path = full_path(&option.filename);
            if is_excel(&path) {
                match excel::frequency::run(
                    &option.filename,
                    option.no_header,
                    option.sheet,
                    &option.cols,
                    option.ascending,
                    option.n,
                    option.export,
                ) {
                    Ok(()) => {}
                    Err(msg) => werr!("Error: {}", msg),
                };
            } else {
                match csv::frequency::run(
                    &option.filename,
                    option.no_header,
                    &option.sep,
                    &option.cols,
                    option.ascending,
                    option.n,
                    option.export,
                ) {
                    Ok(()) => {}
                    Err(msg) => werr!("Error: {}", msg),
                };
            }
        }
        Commands::Split(option) => {
            let path = full_path(&option.filename);
            if is_excel(&path) {
                match excel::split::run(&path, option.sheet, option.no_header, option.col) {
                    Ok(()) => {}
                    Err(msg) => werr!("Error: {}", msg),
                };
            } else {
                match csv::split::run(&option.filename, option.no_header, &option.sep, option.col) {
                    Ok(()) => {}
                    Err(msg) => werr!("Error: {}", msg),
                };
            }
        }
        Commands::Select(option) => {
            let path = full_path(&option.filename);
            if is_excel(&path) {
                match excel::select::run(
                    &path,
                    option.no_header,
                    option.sheet,
                    &option.cols,
                    &option.filter,
                    option.export,
                ) {
                    Ok(()) => {}
                    Err(msg) => werr!("Error: {}", msg),
                };
            } else {
                match csv::select::run(
                    &option.filename,
                    option.no_header,
                    &option.sep,
                    &option.cols,
                    &option.filter,
                    option.export,
                ) {
                    Ok(()) => {}
                    Err(msg) => werr!("Error: {}", msg),
                };
            }
        }
        Commands::Flatten(option) => {
            let path = full_path(&option.filename);
            if is_excel(&path) {
                match excel::flatten::run(
                    &path,
                    option.no_header,
                    option.sheet,
                    &option.delimiter,
                    option.n,
                ) {
                    Ok(()) => {}
                    Err(msg) => werr!("Error: {}", msg),
                };
            } else {
                match csv::flatten::run(
                    &option.filename,
                    option.no_header,
                    &option.sep,
                    &option.delimiter,
                    option.n,
                ) {
                    Ok(()) => {}
                    Err(msg) => werr!("Error: {}", msg),
                };
            }
        }
        Commands::Slice(option) => {
            let path = full_path(&option.filename);
            if is_excel(&path) {
                match excel::slice::run(
                    &path,
                    option.sheet,
                    option.no_header,
                    option.start,
                    option.end,
                    option.length,
                    option.index,
                    option.export,
                ) {
                    Ok(()) => {}
                    Err(msg) => werr!("Error: {}", msg),
                };
            } else {
                match csv::slice::run(
                    &option.filename,
                    option.no_header,
                    option.start,
                    option.end,
                    option.length,
                    option.index,
                    option.export,
                ) {
                    Ok(()) => {}
                    Err(msg) => werr!("Error: {}", msg),
                };
            }
        }
        Commands::Stats(option) => {
            let path = full_path(&option.filename);
            if is_excel(&path) {
                match excel::stats::run(
                    &path,
                    option.sheet,
                    option.no_header,
                    &option.cols,
                    option.export,
                ) {
                    Ok(()) => {}
                    Err(msg) => werr!("Error: {}", msg),
                };
            } else {
                match csv::stats::run(
                    &option.filename,
                    &option.sep,
                    option.no_header,
                    &option.cols,
                    option.export,
                ) {
                    Ok(()) => {}
                    Err(msg) => werr!("Error: {}", msg),
                };
            }
        }
        Commands::Excel2csv(option) => {
            let path = full_path(&option.filename);
            if is_excel(&path) {
                match excel::excel2csv::run(&path, option.sheet, &option.sep) {
                    Ok(()) => {}
                    Err(msg) => werr!("Error: {}", msg),
                }
            } else {
                werr!("Error: File <{}> is not an excel file.", path.display())
            }
        }
        Commands::Table(option) => match csv::table::run(&option.sep) {
            Ok(()) => {}
            Err(msg) => werr!("Error: {}", msg),
        },
        Commands::Search(option) => {
            let path = full_path(&option.filename);
            if is_excel(&path) {
                match excel::search::run(
                    &path,
                    option.sheet,
                    &option.pattern,
                    option.no_header,
                    option.export,
                ) {
                    Ok(()) => {}
                    Err(msg) => werr!("Error: {}", msg),
                }
            } else {
                match csv::search::run(
                    &option.filename,
                    &path,
                    &option.pattern,
                    option.no_header,
                    option.export,
                ) {
                    Ok(()) => {}
                    Err(msg) => werr!("Error: {}", msg),
                }
            }
        }
    }
}
