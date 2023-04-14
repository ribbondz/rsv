use crate::utils::cli_result::CliResult;
use crate::utils::filename::new_path;
use crate::utils::writer::Writer;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn run(path: &Path, no_header: bool, n: usize, export: bool) -> CliResult {
    let out = new_path(path, "-head");
    let mut wtr = Writer::file_or_stdout(export, &out)?;

    // show head n
    BufReader::new(File::open(path)?)
        .lines()
        .take(n + 1 - no_header as usize)
        .for_each(|r| {
            if let Ok(r) = r {
                wtr.write_line_unchecked(&r);
            }
        });

    if export {
        println!("Saved to file: {}", out.display())
    }

    Ok(())
}
