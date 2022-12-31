use crate::utils::progress::Progress;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

extern crate bytecount;

pub fn run(filename: &str, no_header: bool) -> Result<(), Box<dyn Error>> {
    // current file
    let mut path = std::env::current_dir()?;
    path.push(Path::new(filename));

    // progress
    let mut prog = Progress::new();

    // open file and count
    let mut n = 0 - no_header as usize;
    let file = File::open(path)?;
    let mut rdr = BufReader::with_capacity(1024 * 32, file);
    loop {
        let bytes_read = {
            let buf = rdr.fill_buf()?;

            if buf.is_empty() {
                break;
            }

            n += bytecount::count(&buf, b'\n');

            buf.len()
        };

        rdr.consume(bytes_read);
    }
    println!("{}", n);
    prog.print_elapsed_time();

    Ok(())
}
