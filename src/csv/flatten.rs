use crate::utils::cli_result::CliResult;
use crate::utils::file;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn run(path: &Path, no_header: bool, sep: &str, delimiter: &str, n: i32) -> CliResult {
    // open file and header
    let mut rdr = BufReader::new(File::open(path)?).lines();
    let columns: Vec<String> = if no_header {
        let column_n = file::column_n(path, sep)?;
        (1..=column_n)
            .map(|i| "col".to_owned() + &i.to_string())
            .collect::<Vec<_>>()
    } else {
        let first_row = rdr.next().unwrap()?;
        first_row
            .split(sep)
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
    };

    // column_width
    let column_width = columns.iter().map(|i| i.len()).max().unwrap();

    // read file
    let n = if n <= 0 { usize::MAX } else { n as usize };
    let mut rdr = rdr.take(n).peekable();
    while let Some(l) = rdr.next() {
        let l = l.unwrap();

        l.split(sep)
            .zip(&columns)
            .for_each(|(v, k)| println!("{k:width$}    {v}", width = column_width));

        if rdr.peek().is_some() {
            println!("{delimiter}");
        } else {
            println!();
        }
    }

    Ok(())
}
