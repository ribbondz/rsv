use crate::csv::head::print_as_table;
use crate::utils::cli_result::CliResult;
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
        print_as_table(r, sep, no_header);
    } else {
        r.iter().for_each(|i| println!("{}", i));
    }

    Ok(())
}
