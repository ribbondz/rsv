use std::error::Error;
use std::io::BufWriter;
use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
    path::Path,
};

use super::constants::MB_USIZE;
use super::row_split::CsvRowSplitter;

pub fn estimate_row_bytes(path: &Path) -> Result<f64, Box<dyn Error>> {
    // read 20000 lines to estimate bytes per line
    let mut n = 0;
    let mut bytes = 0;
    let file = File::open(path)?;
    for l in BufReader::new(file).lines().skip(1) {
        bytes += l.unwrap().len() + 1;
        n += 1;

        if n > 5000 {
            break;
        }
    }

    // estimate line count
    Ok((bytes as f64) / (n as f64))
}

pub fn column_n(path: &Path, sep: char, quote: char) -> Result<Option<usize>, Box<dyn Error>> {
    // read
    let rdr = BufReader::new(File::open(path)?);
    let n = rdr
        .lines()
        .next()
        .map(|i| i.ok())
        .unwrap_or_default()
        .map(|i| CsvRowSplitter::new(&i, sep, quote).count());

    Ok(n)
}

#[allow(dead_code)]
pub fn estimate_line_count_by_mb(path: &Path, mb: Option<usize>) -> usize {
    match estimate_row_bytes(path) {
        // default chunk-size to 200mb or 10_0000 lines
        Ok(v) => ((mb.unwrap_or(200) * MB_USIZE) as f64 / v) as usize,
        Err(_) => 100_000,
    }
}

pub fn write_frequency_to_csv(path: &Path, names: &[String], freq: Vec<(String, usize)>) {
    let mut wtr = BufWriter::new(File::create(path).unwrap());

    // header
    if !names.is_empty() {
        writeln!(wtr, "{}", names.join(",")).unwrap();
    }

    // content
    for (k, v) in freq {
        writeln!(wtr, "{k},{v}").unwrap();
    }
}

pub fn is_excel(p: &Path) -> bool {
    match p.extension() {
        Some(e) => e == "xlsx" || e == "xls",
        None => false,
    }
}
