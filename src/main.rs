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

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Count(option) => {
            let n = cmds::count::count(&option.filename, option.no_header).unwrap();
            println!("{:?}", n)
        }
        Commands::Estimate(option) => {
            let n = cmds::estimate::estimate(&option.filename).unwrap();
            println!("{:?}", n)
        }
        Commands::Head(option) => {
            cmds::head::head(&option.filename, option.n, option.no_header).unwrap();
        }
        Commands::Clean(option) => {
            cmds::clean::clean(&option.filename, &option.escape, &option.f).unwrap();
        }
        Commands::Frequency(option) => {
            cmds::frequency::frequency(
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
            cmds::partition::partition(&option.filename, option.no_header, &option.sep, option.col)
                .unwrap();
        }
    }
}
