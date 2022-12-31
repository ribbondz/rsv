use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

extern crate bytecount;

pub fn count(filename: &str, no_header: bool) -> Result<i32, Box<dyn std::error::Error>> {
    // current file
    let mut path = std::env::current_dir()?;
    path.push(Path::new(filename));

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

    Ok(n as i32)
}
