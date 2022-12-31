use regex::bytes::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

use crate::utils;
use crate::utils::file::estimate_line_count_by_mb;
use crate::utils::progress::Progress;
extern crate bytecount;

pub fn clean(
    filename: &str,
    escape: &str,
    new_filename: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // current file
    let mut path = std::env::current_dir()?;
    path.push(Path::new(filename));
    // new file
    let new_path = if new_filename.is_empty() {
        utils::filename::new_path(&path, "-cleaned").to_owned()
    } else {
        Path::new(new_filename).into()
    };
    // open current file
    let mut rdr = BufReader::new(File::open(&path)?);
    // open new file
    let mut wtr = BufWriter::new(File::create(&new_path)?);
    // progress
    let mut prog = Progress::new();
    let prog_check_every_n = estimate_line_count_by_mb(filename, None);
    // copy
    let re = Regex::new(escape)?;
    let null = &b""[..];
    let mut buf = vec![];
    let mut i = 0;
    while let Ok(bytes_read) = rdr.read_until(b'\n', &mut buf) {
        if bytes_read == 0 {
            break;
        }
        prog.add_bytes(bytes_read);
        let str = &re.replace_all(&buf[..bytes_read], null);
        wtr.write(str)?;
        buf.clear();
        // progress print
        if i % prog_check_every_n == 0 {
            prog.add_chuncks(1);
            prog.print();
        }
        i += 1;
    }
    prog.print();

    println!("\nSaved to new file: {:?}.", new_path);
    Ok(())
}
