use crate::utils::{cli_result::CliResult, util::print_tabled};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub fn run(path: &Path, sep: &str) -> CliResult {
    // rdr
    let rdr = BufReader::new(File::open(path)?);

    let rows = rdr
        .lines()
        .into_iter()
        .filter_map(|r| r.ok())
        .map(|r| r.split(sep).map(|i| i.to_owned()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    if !rows.is_empty() {
        print_tabled(rows);
    }

    Ok(())
}
