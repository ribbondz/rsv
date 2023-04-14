use crate::utils::cli_result::CliResult;
use crate::utils::file;
use crate::utils::table::Table;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn run(path: &Path, no_header: bool, sep: &str, delimiter: &str, n: i32) -> CliResult {
    // open file and header
    let mut rdr = BufReader::new(File::open(path)?).lines();

    // header
    let columns: Vec<String> = if no_header {
        match file::column_n(path, sep)? {
            Some(n) => (1..=n).map(|i| format!("col{i}")).collect::<Vec<_>>(),
            None => return Ok(()),
        }
    } else {
        match rdr.next() {
            Some(r) => r?.split(sep).map(|i| i.to_string()).collect::<Vec<_>>(),
            None => return Ok(()),
        }
    };

    // read file
    let n = if n <= 0 { usize::MAX } else { n as usize };
    let mut rdr = rdr.take(n).peekable();
    while let Some(l) = rdr.next() {
        let l = l?;
        let r = l
            .split(sep)
            .zip(&columns)
            .map(|(v, k)| [k.as_str(), v])
            .collect::<Vec<_>>();
        Table::from_records(r).print_blank()?;

        if rdr.peek().is_some() {
            println!(" {delimiter}");
        } else {
            println!();
        }
    }

    Ok(())
}
