use std::io::{stdin, BufRead};

use crate::utils::cli_result::CliResult;

pub fn run(no_header: bool) -> CliResult {
    // progress
    let mut n = stdin().lock().lines().count();

    if !no_header && n > 0 {
        n -= 1;
    }

    println!("{n}");

    Ok(())
}
