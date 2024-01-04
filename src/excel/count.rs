use crate::utils::cli_result::CliResult;
use crate::utils::progress::Progress;
use crate::utils::reader::ExcelReader;
use std::path::Path;

pub fn run(path: &Path, sheet: usize, no_header: bool) -> CliResult {
    // progress
    let mut prog = Progress::new();

    // open file and count
    let range = ExcelReader::new(path, sheet)?;
    let mut n = range.len();

    // default to have a header
    if !no_header && n > 0 {
        n -= 1;
    }

    println!("{n}");
    prog.print_elapsed_time();

    Ok(())
}
