use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn run(filename: &str, sep: &str) -> Result<(), Box<dyn Error>> {
    // current file
    let mut path = std::env::current_dir()?;
    path.push(Path::new(filename));

    // open file and header
    let mut rdr = BufReader::new(File::open(&path)?).lines();
    let first_row = rdr.next().unwrap()?;
    first_row
        .split(sep)
        .enumerate()
        .for_each(|(i, v)| println!("{i:<5}{v}"));

    Ok(())
}
