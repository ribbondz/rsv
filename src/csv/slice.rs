use crate::utils::cli_result::CliResult;
use crate::utils::filename::new_path;
use crate::utils::writer::Writer;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

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
    let mut wtr = Writer::file_or_stdout(export, &out_path)?;
    let mut rdr = BufReader::new(File::open(path)?);

    // header
    if !no_header {
        let mut buf = vec![];
        match rdr.read_until(b'\n', &mut buf) {
            Ok(_) => wtr.write_bytes_unchecked(&buf),
            Err(_) => return Ok(()),
        };
    }

    // slice
    match index {
        Some(index) => write_by_index(&mut rdr, &mut wtr, index),
        None => {
            let e = end
                .or_else(|| length.map(|l| start + l))
                .unwrap_or(usize::MAX - 10);
            write_by_range(&mut rdr, &mut wtr, start, e);
        }
    }

    if export {
        println!("Saved to file: {}", out_path.display())
    }

    Ok(())
}

fn write_by_index(rdr: &mut BufReader<File>, wtr: &mut Writer, index: usize) {
    let mut buf = vec![];
    let mut n = 0;

    while let Ok(bytes) = rdr.read_until(b'\n', &mut buf) {
        if bytes == 0 {
            break;
        }

        if n == index {
            wtr.write_bytes_unchecked(&buf[..bytes]);
            break;
        }

        buf.clear();
        n += 1;
    }
}

fn write_by_range(rdr: &mut BufReader<File>, wtr: &mut Writer, start: usize, end: usize) {
    let mut buf = vec![];
    let mut n = 0;

    while let Ok(bytes) = rdr.read_until(b'\n', &mut buf) {
        if bytes == 0 || n >= end {
            break;
        }

        if n >= start && n < end {
            wtr.write_bytes_unchecked(&buf[..bytes]);
        }

        buf.clear();
        n += 1;
    }
}
