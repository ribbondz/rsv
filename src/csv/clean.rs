use crate::utils;
use crate::utils::cli_result::CliResult;
use crate::utils::file::estimate_line_count_by_mb;
use crate::utils::progress::Progress;
use regex::bytes::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

pub fn run(path: &Path, escape: &str, new_filename: &str) -> CliResult {
    // new file
    let new_path = match new_filename.is_empty() {
        true => utils::filename::new_path(path, "-cleaned"),
        false => Path::new(new_filename).into(),
    };

    // open files
    let mut rdr = BufReader::new(File::open(path)?);
    let mut wtr = BufWriter::new(File::create(&new_path)?);

    // progress
    let mut prog = Progress::new();
    let prog_check_every_n = estimate_line_count_by_mb(path, None);

    // copy
    let re = Regex::new(escape)?;
    let empty_bytes = &b""[..];

    let mut buf = vec![];
    let mut i = 0;
    while let Ok(bytes_read) = rdr.read_until(b'\n', &mut buf) {
        if bytes_read == 0 {
            break;
        }

        let str = re.replace_all(&buf[..bytes_read], empty_bytes);
        wtr.write_all(&str)?;
        buf.clear();

        // progress print
        prog.add_bytes(bytes_read);
        if i % prog_check_every_n == 0 {
            prog.add_chunks(1);
            prog.print();
        }

        i += 1;
    }

    prog.print();

    println!("\nSaved to file: {}", new_path.display());

    Ok(())
}
