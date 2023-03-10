use crate::utils::cli_result::CliResult;
use std::io::{stdin, BufRead};

pub fn run(sep: &str) -> CliResult {
    // open file and header
    if let Some(r) = stdin().lock().lines().next() {
        r?.split(sep)
            .enumerate()
            .for_each(|(u, r)| println!(" {u:<5}{r}"));
    }

    Ok(())
}
