use crate::utils::cli_result::CliResult;
use crate::utils::filename::new_path;
use crate::utils::table::Table;
use crate::utils::writer::Writer;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn run(
    path: &Path,
    no_header: bool,
    sep: &str,
    n: usize,
    tabled: bool,
    export: bool,
) -> CliResult {
    let out = new_path(path, "-head");
    let mut wtr = Writer::file_or_stdout(export, &out)?;

    // show head n
    let r = BufReader::new(File::open(path)?)
        .lines()
        .take(n + 1 - no_header as usize)
        .filter_map(|i| i.ok())
        .collect::<Vec<_>>();

    // tabled or not
    if export || !tabled {
        wtr.write_lines_unchecked(&r);
    } else {
        Table::from_rows(&r, sep).print_blank()?;
    }

    if export {
        println!("Saved to file: {}", out.display())
    }

    Ok(())
}
