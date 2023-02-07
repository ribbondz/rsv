use crate::utils::{cli_result::CliResult, table::Table};
use std::io::{self, BufRead};

pub fn run(sep: &str) -> CliResult {
    let lines = io::stdin()
        .lock()
        .lines()
        .filter_map(|i| i.ok())
        .collect::<Vec<_>>();
    let lines = lines
        .iter()
        .map(|r| r.split(sep).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    Table::from_records(lines).print_blank()?;

    Ok(())
}
