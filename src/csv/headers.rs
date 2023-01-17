use crate::utils::cli_result::CliResult;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn run(path: &Path, sep: &str) -> CliResult {
    // open file and header
    let mut rdr = BufReader::new(File::open(path)?).lines();

    let first_row = match rdr.next() {
        Some(r) => match r {
            Ok(r) => r,
            Err(_) => return Ok(()),
        },
        None => return Ok(()),
    };
    
    first_row
        .split(sep)
        .enumerate()
        .for_each(|(i, v)| println!("{i:<5}{v}"));

    Ok(())
}
