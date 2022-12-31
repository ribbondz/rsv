use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

extern crate bytecount;

pub fn count(filename: &str, header: bool) -> Result<i32, Box<dyn std::error::Error>> {
    // current file
    let mut path = std::env::current_dir()?;
    path.push(Path::new(filename));

    // open file and count
    let mut n = 0 - header as usize;
    let mut reader = BufReader::with_capacity(1024 * 32, File::open(path)?);
    loop {
        let len = {
            let buf = reader.fill_buf()?;
            if buf.is_empty() {
                break;
            }
            n += bytecount::count(&buf, b'\n');
            buf.len()
        };
        reader.consume(len);
    }

    Ok(n as i32)
}
