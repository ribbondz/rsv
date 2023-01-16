use crate::utils::cli_result::CliResult;
use std::io::{stdin, BufRead};

pub fn run(sep: &str) -> CliResult {
    // open file and header
    match stdin().lock().lines().next() {
        Some(r) => {
            let r = r?;
            r.split(sep)
                .enumerate()
                .for_each(|(u, r)| println!("{:<5}{}", u, r));
        }
        None => {}
    }

    Ok(())
}
