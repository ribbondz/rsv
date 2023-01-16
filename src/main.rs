use clap::{Args, Parser, Subcommand};
use utils::{
    cli_result::E,
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
mod io;
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
    filename: Option<String>,
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
    filename: Option<String>,
    /// Get the nth worksheet for an Excel file
    #[arg(short = 'S', long, default_value_t = 0)]
    sheet: usize,
}

#[derive(Debug, Args)]
struct Headers {
    /// File to open
    filename: Option<String>,
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
    filename: Option<String>,
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
    filename: Option<String>,
    /// Separator
    #[arg(short, long, default_value_t = String::from(","))]
    sep: String,
    /// Whether the file has a header
    #[arg(long, default_value_t = false)]
    no_header: bool,
    /// Number of records to show
    #[arg(short, long, default_value_t = 10)]
    n: usize,
    /// Print as an aligned table
    #[arg(short, long, default_value_t = false)]
    tabled: bool,
    /// Get the nth worksheet of EXCEL file
    #[arg(short = 'S', long, default_value_t = 0)]
    sheet: usize,
}

#[derive(Debug, Args)]
struct Flatten {
    /// File to open
    filename: Option<String>,
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
    filename: Option<String>,
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
    filename: Option<String>,
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
    filename: Option<String>,
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
    filename: Option<String>,
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
    /// File to open
    filename: Option<String>,
    /// Get the nth worksheet of EXCEL file
    #[arg(short = 'S', long, default_value_t = 0)]
    sheet: usize,
    /// Separator
    #[arg(short, long, default_value_t = String::from(","))]
    sep: String,
}

#[derive(Debug, Args)]
struct Search {
    /// Regex pattern to search
    pattern: String,
    /// File to open
    filename: Option<String>,
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
        Commands::Count(option) => match &option.filename {
            Some(f) => {
                let path = full_path(f);
                match is_excel(&path) {
                    true => excel::count::run(&path, option.sheet, option.no_header).handle_err(),
                    false => csv::count::run(&path, option.no_header).handle_err(),
                }
            }
            None => io::count::run(option.no_header).handle_err(),
        },
        Commands::Head(option) => match &option.filename {
            Some(f) => {
                let path = full_path(f);
                match is_excel(&path) {
                    true => excel::head::run(&path, option.sheet, option.no_header, option.n)
                        .handle_err(),
                    false => csv::head::run(
                        &path,
                        option.no_header,
                        &option.sep,
                        option.n,
                        option.tabled,
                    )
                    .handle_err(),
                }
            }
            None => {
                io::head::run(option.no_header, &option.sep, option.n, option.tabled).handle_err()
            }
        },
        Commands::Headers(option) => match &option.filename {
            Some(f) => {
                let path = full_path(f);
                match is_excel(&path) {
                    true => excel::headers::run(&path, option.sheet).handle_err(),
                    false => csv::headers::run(&path, &option.sep).handle_err(),
                }
            }
            None => io::headers::run(&option.sep).handle_err(),
        },
        Commands::Estimate(option) => match &option.filename {
            Some(f) => {
                let path = full_path(f);
                match is_excel(&path) {
                    true => excel::count::run(&path, option.sheet, true).handle_err(),
                    false => csv::estimate::run(&path).handle_err(),
                }
            }
            None => io::count::run(false).handle_err(),
        },
        Commands::Clean(option) => {
            let path = full_path(&option.filename);
            if is_excel(&path) {
                werr!("rsv clean does not support Excel files.")
            } else {
                csv::clean::run(&path, &option.escape, &option.output).handle_err()
            }
        }
        Commands::Frequency(option) => match &option.filename {
            Some(f) => {
                let path = full_path(f);
                match is_excel(&path) {
                    true => excel::frequency::run(
                        &path,
                        option.no_header,
                        option.sheet,
                        &option.cols,
                        option.ascending,
                        option.n,
                        option.export,
                    )
                    .handle_err(),
                    false => csv::frequency::run(
                        &path,
                        option.no_header,
                        &option.sep,
                        &option.cols,
                        option.ascending,
                        option.n,
                        option.export,
                    )
                    .handle_err(),
                }
            }
            None => io::frequency::run(
                option.no_header,
                &option.sep,
                &option.cols,
                option.ascending,
                option.n,
                option.export,
            )
            .handle_err(),
        },
        Commands::Split(option) => match &option.filename {
            Some(f) => {
                let path = full_path(f);
                match is_excel(&path) {
                    true => excel::split::run(&path, option.sheet, option.no_header, option.col)
                        .handle_err(),
                    false => csv::split::run(&path, option.no_header, &option.sep, option.col)
                        .handle_err(),
                }
            }
            None => io::split::run(option.no_header, &option.sep, option.col).handle_err(),
        },
        Commands::Select(option) => match &option.filename {
            Some(f) => {
                let path = full_path(f);
                match is_excel(&path) {
                    true => excel::select::run(
                        &path,
                        option.no_header,
                        option.sheet,
                        &option.cols,
                        &option.filter,
                        option.export,
                    )
                    .handle_err(),
                    false => csv::select::run(
                        &path,
                        option.no_header,
                        &option.sep,
                        &option.cols,
                        &option.filter,
                        option.export,
                    )
                    .handle_err(),
                }
            }
            None => io::select::run(
                option.no_header,
                &option.sep,
                &option.cols,
                &option.filter,
                option.export,
            )
            .handle_err(),
        },
        Commands::Flatten(option) => match &option.filename {
            Some(f) => {
                let path = full_path(f);
                match is_excel(&path) {
                    true => excel::flatten::run(
                        &path,
                        option.no_header,
                        option.sheet,
                        &option.delimiter,
                        option.n,
                    )
                    .handle_err(),
                    false => csv::flatten::run(
                        &path,
                        option.no_header,
                        &option.sep,
                        &option.delimiter,
                        option.n,
                    )
                    .handle_err(),
                }
            }
            None => io::flatten::run(option.no_header, &option.sep, &option.delimiter, option.n)
                .handle_err(),
        },
        Commands::Slice(option) => match &option.filename {
            Some(f) => {
                let path = full_path(f);
                match is_excel(&path) {
                    true => excel::slice::run(
                        &path,
                        option.sheet,
                        option.no_header,
                        option.start,
                        option.end,
                        option.length,
                        option.index,
                        option.export,
                    )
                    .handle_err(),
                    false => csv::slice::run(
                        &path,
                        option.no_header,
                        option.start,
                        option.end,
                        option.length,
                        option.index,
                        option.export,
                    )
                    .handle_err(),
                }
            }
            None => io::slice::run(
                option.no_header,
                option.start,
                option.end,
                option.length,
                option.index,
                option.export,
            )
            .handle_err(),
        },
        Commands::Stats(option) => match &option.filename {
            Some(f) => {
                let path = full_path(f);
                match is_excel(&path) {
                    true => excel::stats::run(
                        &path,
                        option.sheet,
                        option.no_header,
                        &option.cols,
                        option.export,
                    )
                    .handle_err(),
                    false => csv::stats::run(
                        &path,
                        &option.sep,
                        option.no_header,
                        &option.cols,
                        option.export,
                    )
                    .handle_err(),
                }
            }
            None => io::stats::run(&option.sep, option.no_header, &option.cols, option.export)
                .handle_err(),
        },
        Commands::Excel2csv(option) => {
            let path = full_path(&option.filename);
            match is_excel(&path) {
                true => excel::excel2csv::run(&path, option.sheet, &option.sep).handle_err(),
                false => werr!("Error: File <{}> is not an excel file.", path.display()),
            }
        }
        Commands::Table(option) => match &option.filename {
            Some(f) => {
                let p = full_path(f);
                match is_excel(&p) {
                    true => excel::table::run(&p, option.sheet).handle_err(),
                    false => csv::table::run(&p, &option.sep).handle_err(),
                }
            }
            None => io::table::run(&option.sep).handle_err(),
        },
        Commands::Search(option) => match &option.filename {
            Some(f) => {
                let path = full_path(f);
                match is_excel(&path) {
                    true => excel::search::run(
                        &path,
                        option.sheet,
                        &option.pattern,
                        option.no_header,
                        option.export,
                    )
                    .handle_err(),
                    false => {
                        csv::search::run(&path, &option.pattern, option.no_header, option.export)
                            .handle_err()
                    }
                }
            }
            None => io::search::run(&option.pattern, option.no_header, option.export).handle_err(),
        },
    }
}
