use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
    path::{Path, PathBuf},
};

pub fn estimate_row_bytes(filename: &str) -> Result<f64, Box<dyn std::error::Error>> {
    // current file
    let mut path = std::env::current_dir()?;
    path.push(Path::new(filename));
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
    let row_bytes = (bytes as f64) / (n as f64);
    Ok(row_bytes)
}

pub fn estimate_line_count_by_mb(filename: &str, mb: Option<usize>) -> usize {
    match estimate_row_bytes(&filename) {
        // default chunksize to 200mb or 10_0000 lines
        Ok(v) => ((mb.unwrap_or(200) as f64) * 1024.0 * 1024.0 / v) as usize,
        Err(_) => 10_0000,
    }
}

pub fn write_to_csv(path: &PathBuf, names: &Vec<String>, freq: Vec<(String, i32)>) {
    let handler = File::create(path).unwrap();
    let mut f = BufWriter::new(handler);
    // header
    if names.len() > 0 {
        write!(f, "{}\n", names.join(",")).unwrap();
    }
    // content
    for (k, v) in freq {
        write!(f, "{},{}\n", k, v).unwrap();
    }
}
