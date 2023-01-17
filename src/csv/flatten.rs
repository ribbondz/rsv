use crate::utils::cli_result::CliResult;
use crate::utils::file;
use crate::utils::util::print_tabled;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn run(path: &Path, no_header: bool, sep: &str, delimiter: &str, n: i32) -> CliResult {
    // open file and header
    let mut rdr = BufReader::new(File::open(path)?).lines();

    // header
    let columns: Vec<String> = if no_header {
        let column_n = file::column_n(path, sep)?;
        (1..=column_n)
            .map(|i| "col".to_owned() + &i.to_string())
            .collect::<Vec<_>>()
    } else {
        match rdr.next() {
            Some(r) => match r {
                Ok(v) => v.split(sep).map(|i| i.to_string()).collect::<Vec<_>>(),
                Err(_) => return Ok(()),
            },
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
            .map(|(v, k)| vec![k.to_owned(), v.to_owned()])
            .collect::<Vec<_>>();
        print_tabled(r);

        if rdr.peek().is_some() {
            println!(" {delimiter}");
        } else {
            println!();
        }
    }

    Ok(())
}
