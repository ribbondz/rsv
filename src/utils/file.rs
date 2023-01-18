use std::error::Error;
use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
    path::{Path, PathBuf},
};

use super::constants::MB_USIZE;

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

pub fn column_n(path: &Path, sep: &str) -> Result<Option<usize>, Box<dyn Error>> {
    // read
    let rdr = BufReader::new(File::open(path)?);
    let n = rdr
        .lines()
        .next()
        .map(|i| i.ok())
        .unwrap_or_default()
        .map(|i| i.split(sep).count());

    Ok(n)
}

pub fn estimate_line_count_by_mb(path: &Path, mb: Option<usize>) -> usize {
    match estimate_row_bytes(path) {
        // default chunk-size to 200mb or 10_0000 lines
        Ok(v) => ((mb.unwrap_or(200) * MB_USIZE) as f64 / v) as usize,
        Err(_) => 10_0000,
    }
}

pub fn write_frequency_to_csv(path: &PathBuf, names: &Vec<String>, freq: Vec<(String, usize)>) {
    let mut f = BufWriter::new(File::create(path).unwrap());

    // header
    if !names.is_empty() {
        writeln!(f, "{}", names.join(",")).unwrap();
    }

    // content
    for (k, v) in freq {
        writeln!(f, "{},{}", k, v).unwrap();
    }
}

pub fn first_row(path: &Path) -> Result<Option<String>, Box<dyn Error>> {
    let mut rdr = BufReader::new(File::open(path)?).lines();

    let r = rdr.next().and_then(|i| i.ok());
    Ok(r)
}

pub fn is_excel(p: &Path) -> bool {
    match p.extension() {
        Some(e) => e == "xlsx" || e == "xls",
        None => false,
    }
}
