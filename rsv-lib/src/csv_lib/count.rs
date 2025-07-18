use crate::utils::return_result::{CliResultData, ResultData};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

pub fn csv_count(file: &PathBuf, no_header: bool) -> CliResultData {
    // current file
    let n = match file.is_dir() {
        true => count_dir_files(&file)?,
        false => count_file_lines(&file, no_header)?,
    };

    Ok(Some(ResultData {
        header: vec!["count".to_string()],
        data: vec![vec![n.to_string()]],
    }))
}

fn count_file_lines(path: &Path, no_header: bool) -> Result<usize, Box<dyn std::error::Error>> {
    // open file and count
    let mut n = 0;
    let file = File::open(path)?;
    let mut rdr = BufReader::with_capacity(1024 * 32, file);
    loop {
        let bytes_read = {
            let buf = rdr.fill_buf()?;
            if buf.is_empty() {
                break;
            }
            n += bytecount::count(buf, b'\n');
            buf.len()
        };

        rdr.consume(bytes_read);
    }

    if !no_header && n > 0 {
        n -= 1;
    }

    Ok(n)
}

fn count_dir_files(path: &Path) -> Result<usize, Box<dyn std::error::Error>> {
    let mut file_n = 0;
    let mut dir_n = 0;

    path.read_dir()?.for_each(|i| {
        if let Ok(e) = i {
            if e.file_type().unwrap().is_file() {
                file_n += 1;
            } else {
                dir_n += 1;
            }
        }
    });

    Ok(file_n)
}
