use crate::utils::{cli_result::CliResult, table::Table};
use std::io::{self, BufRead};

pub fn run(sep: &str) -> CliResult {
    let mut rows = vec![];

    for l in io::stdin().lock().lines() {
        let l = l?.split(sep).map(String::from).collect::<Vec<_>>();
        rows.push(l);
    }

    Table::from_records(rows).print_blank()?;

    Ok(())
}
