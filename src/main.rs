use clap::{Args, Parser, Subcommand};
mod cmds;
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
    /// Count the number of lines
    Count(Count),
    /// Fast estimate the number of lines
    Estimate(Filename),
    /// Show head n lines
    Head(Head),
    /// Clean file with escape chars (e.g. "")
    Clean(Clean),
    /// Frequency table of some columns
    Frequency(Frequency),
    /// Partition file into separate files according to column value
    Partition(Partition),
    /// Select rows and columns by a filter
    Select(Select),
}

#[derive(Debug, Args)]
struct Count {
    /// File to open
    filename: String,
    /// Whether the file has a header
    #[arg(long, default_value_t = false)]
    no_header: bool,
}

#[derive(Debug, Args)]
struct Filename {
    /// File to open
    filename: String,
}

#[derive(Debug, Args)]
struct Head {
    /// File to open
    filename: String,
    /// Whether the file has a header
    #[arg(long, default_value_t = false)]
    no_header: bool,
    /// Number of records to show
    #[arg(short, long, default_value_t = 20)]
    n: usize,
}

#[derive(Debug, Args)]
struct Clean {
    /// File to open
    filename: String,
    /// New file to save data, default to current-file-cleaned.csv
    #[arg(short, long, default_value_t = String::from(""), hide_default_value=true)]
    f: String,
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
    #[arg(short, long, default_value_t = false)]
    export: bool,
    /// Top N to keep in frequency table
    #[arg(short, long, default_value_t = -1)]
    n: i32,
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
    #[arg(short, long, default_value_t = false)]
    export: bool,
}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Count(option) => {
            cmds::count::run(&option.filename, option.no_header).unwrap();
        }
        Commands::Estimate(option) => {
            cmds::estimate::run(&option.filename).unwrap();
        }
        Commands::Head(option) => {
            cmds::head::run(&option.filename, option.n, option.no_header).unwrap();
        }
        Commands::Clean(option) => {
            cmds::clean::run(&option.filename, &option.escape, &option.f).unwrap();
        }
        Commands::Frequency(option) => {
            cmds::frequency::run(
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
        Commands::Partition(option) => {
            cmds::partition::run(&option.filename, option.no_header, &option.sep, option.col)
                .unwrap();
        }
        Commands::Select(option) => cmds::select::run(
            &option.filename,
            option.no_header,
            &option.sep,
            &option.cols,
            &option.filter,
            option.export,
        )
        .unwrap(),
    }
}
