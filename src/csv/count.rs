use crate::args::Count;
use crate::utils::cli_result::CliResult;
use crate::utils::progress::Progress;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
extern crate bytecount;

impl Count {
    pub fn csv_run(&self) -> CliResult {
        // current file
        match self.path().is_dir() {
            true => count_dir_files(&self.path())?,
            false => count_file_lines(&self.path(), self.no_header)?,
        };

        Ok(())
    }
}

fn count_file_lines(path: &Path, no_header: bool) -> CliResult {
    // progress
    let mut prog = Progress::new();

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

    println!("{n}");
    prog.print_elapsed_time();

    Ok(())
}

fn count_dir_files(path: &Path) -> CliResult {
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

    println!(
        "{} files and {} sub-directories in {}",
        file_n,
        dir_n,
        path.display()
    );

    Ok(())
}
