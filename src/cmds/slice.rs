use crate::utils::filename::{full_path_file, new_path};

use std::fs::File;
use std::io::{stdout, BufRead, BufWriter, Write};
use std::{error::Error, io::BufReader};

pub fn run(
    filename: &str,
    no_header: bool,
    start: usize,
    end: Option<usize>,
    length: Option<usize>,
    index: Option<usize>,
    export: bool,
) -> Result<(), Box<dyn Error>> {
    // current file
    let path = full_path_file(filename)?;

    // open file
    let f = match export {
        true => {
            let out_path = new_path(&path, "-selected");
            Box::new(File::create(&out_path)?) as Box<dyn Write>
        }
        false => Box::new(stdout()) as Box<dyn Write>,
    };
    let mut wtr = BufWriter::new(f);
    let mut rdr = BufReader::new(File::open(&path)?);

    // header
    if !no_header {
        let mut buf = vec![];
        rdr.read_until(b'\n', &mut buf)?;
        wtr.write(&buf)?;
    }

    // slice
    match index {
        Some(index) => write_by_index(&mut rdr, &mut wtr, index)?,
        None => {
            let e = end
                .or(length.and_then(|l| Some(start + l)).or(Some(usize::MAX)))
                .unwrap();
            write_by_range(&mut rdr, &mut wtr, start, e)?;
        }
    }

    Ok(())
}

fn write_by_index(
    rdr: &mut BufReader<File>,
    wtr: &mut BufWriter<Box<dyn Write>>,
    index: usize,
) -> std::io::Result<()> {
    let mut buf = vec![];
    let mut n = 0;

    while let Ok(bytes) = rdr.read_until(b'\n', &mut buf) {
        if bytes == 0 {
            break;
        }

        if n == index {
            wtr.write(&buf[..bytes])?;
            break;
        }

        buf.clear();
        n += 1;
    }

    Ok(())
}

fn write_by_range(
    rdr: &mut BufReader<File>,
    wtr: &mut BufWriter<Box<dyn Write>>,
    start: usize,
    end: usize,
) -> std::io::Result<()> {
    let mut buf = vec![];
    let mut n = 0;

    while let Ok(bytes) = rdr.read_until(b'\n', &mut buf) {
        if bytes == 0 || n >= end {
            break;
        }

        if n >= start && n < end {
            wtr.write(&buf[..bytes])?;
        }

        buf.clear();
        n += 1;
    }

    Ok(())
}
