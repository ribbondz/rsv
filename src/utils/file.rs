use std::error::Error;
use std::io::stdout;
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

pub fn column_n(path: &Path, sep: &str) -> Result<usize, Box<dyn Error>> {
    // read
    let rdr = BufReader::new(File::open(path)?);
    let r = rdr.lines().next().unwrap()?;

    Ok(r.split(sep).count())
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

pub fn file_or_stdout_wtr(export: bool, path: &Path) -> Result<Box<dyn Write>, Box<dyn Error>> {
    match export {
        true => Ok(Box::new(File::create(path)?) as Box<dyn Write>),
        false => Ok(Box::new(stdout()) as Box<dyn Write>),
    }
}

pub fn first_row(path: &Path) -> Result<Option<String>, Box<dyn Error>> {
    let mut rdr = BufReader::new(File::open(path)?).lines();

    let r = rdr.next().and_then(|i| i.ok());
    Ok(r)
}

pub fn is_excel(p: &Path) -> bool {
    let e = p.extension().unwrap();
    e == "xlsx" || e == "xls"
}
