use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn run(filename: &str, n: usize, no_header: bool) -> Result<(), Box<dyn std::error::Error>> {
    // current file
    let mut path = std::env::current_dir()?;
    path.push(Path::new(filename));

    // show head n
    for l in BufReader::new(File::open(path)?)
        .lines()
        .take(n + 1 - no_header as usize)
    {
        println!("{}", l.unwrap())
    }

    Ok(())
}
