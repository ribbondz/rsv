use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn head(filename: &str, n: usize) -> Result<(), Box<dyn std::error::Error>> {
    // current file
    let mut path = std::env::current_dir()?;
    path.push(Path::new(filename));

    // show head n
    let file = File::open(path)?;
    for l in BufReader::new(file).lines().take(n + 1) {
        println!("{}", l.unwrap())
    }

    Ok(())
}
