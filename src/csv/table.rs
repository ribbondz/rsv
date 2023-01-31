use crate::utils::{cli_result::CliResult, table::Table};
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
        .map(|r| r.split(sep).map(String::from).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    Table::from_records(rows).print_blank()?;

    Ok(())
}
