use crate::utils::filename::full_path;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run(filename: &str, sep: &str) -> Result<(), Box<dyn Error>> {
    // current file
    let path = full_path(filename)?;

    // open file and header
    let mut rdr = BufReader::new(File::open(path)?).lines();
    let first_row = rdr.next().unwrap()?;
    first_row
        .split(sep)
        .enumerate()
        .for_each(|(i, v)| println!("{i:<5}{v}"));

    Ok(())
}
