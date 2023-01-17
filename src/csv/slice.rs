use crate::utils::cli_result::CliResult;
use crate::utils::file::file_or_stdout_wtr;
use crate::utils::filename::new_path;
use std::fs::File;
use std::io::BufReader;
use std::io::{BufRead, BufWriter, Write};
use std::path::Path;
use std::process;

pub fn run(
    path: &Path,
    no_header: bool,
    start: usize,
    end: Option<usize>,
    length: Option<usize>,
    index: Option<usize>,
    export: bool,
) -> CliResult {
    // current file
    let out_path = new_path(path, "-slice");

    // open file
    let f = file_or_stdout_wtr(export, &out_path)?;
    let mut wtr = BufWriter::new(f);
    let mut rdr = BufReader::new(File::open(path)?);

    // header
    if !no_header {
        let mut buf = vec![];
        if rdr.read_until(b'\n', &mut buf).is_err() {
            return Ok(());
        };
        write_bytes(&mut wtr, &buf);
    }

    // slice
    match index {
        Some(index) => write_by_index(&mut rdr, &mut wtr, index),
        None => {
            let e = end
                .or_else(|| length.map(|l| start + l).or(Some(usize::MAX - 10)))
                .unwrap();
            write_by_range(&mut rdr, &mut wtr, start, e);
        }
    }

    if export {
        println!("Saved to file: {}", out_path.display())
    }

    Ok(())
}

fn write_by_index(rdr: &mut BufReader<File>, wtr: &mut BufWriter<Box<dyn Write>>, index: usize) {
    let mut buf = vec![];
    let mut n = 0;

    while let Ok(bytes) = rdr.read_until(b'\n', &mut buf) {
        if bytes == 0 {
            break;
        }

        if n == index {
            write_bytes(wtr, &buf[..bytes]);
            break;
        }

        buf.clear();
        n += 1;
    }
}

fn write_by_range(
    rdr: &mut BufReader<File>,
    wtr: &mut BufWriter<Box<dyn Write>>,
    start: usize,
    end: usize,
) {
    let mut buf = vec![];
    let mut n = 0;

    while let Ok(bytes) = rdr.read_until(b'\n', &mut buf) {
        if bytes == 0 || n >= end {
            break;
        }

        if n >= start && n < end {
            write_bytes(wtr, &buf[..bytes])
        }

        buf.clear();
        n += 1;
    }
}

/// ignore write error in case pipeline is early closed.
fn write_bytes(wtr: &mut BufWriter<Box<dyn Write>>, data: &[u8]) {
    if wtr.write_all(data).is_err() {
        process::exit(0)
    }
}
