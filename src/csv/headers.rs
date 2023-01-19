use crate::utils::cli_result::CliResult;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn run(path: &Path, sep: &str) -> CliResult {
    // open file and header
    let mut rdr = BufReader::new(File::open(path)?).lines();

    if let Some(r) = rdr.next() {
        r?.split(sep)
            .enumerate()
            .for_each(|(i, v)| println!(" {i:<5}{v}"));
    };

    Ok(())
}
