use crate::utils::{cli_result::CliResult, table::Table};
use std::io::{stdin, BufRead};

pub fn run(no_header: bool, sep: &str, n: usize, tabled: bool) -> CliResult {
    // show head n
    let r = stdin()
        .lock()
        .lines()
        .take(n + 1 - no_header as usize)
        .filter_map(|i| i.ok())
        .collect::<Vec<_>>();

    // tabled or not
    if tabled {
        Table::from_rows(&r, sep).print_blank()?;
    } else {
        r.iter().for_each(|i| println!("{}", i));
    }

    Ok(())
}
