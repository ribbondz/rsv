use crate::utils::{cli_result::CliResult, util::print_table};
use std::io::{self, BufRead};

pub fn run(sep: &str) -> CliResult {
    let mut rows = vec![];

    for l in io::stdin().lock().lines() {
        let l = l?.split(sep).map(|i| i.to_owned()).collect::<Vec<_>>();
        rows.push(l);
    }

    print_table(rows);

    Ok(())
}
