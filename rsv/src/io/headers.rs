use crate::args::Headers;
use rsv_lib::utils::cli_result::CliResult;
use std::io::{stdin, BufRead};

impl Headers {
    pub fn io_run(&self) -> CliResult {
        // open file and header
        if let Some(r) = stdin().lock().lines().next() {
            self.split(&r?)
                .enumerate()
                .for_each(|(u, r)| println!(" {u:<5}{r}"));
        }

        Ok(())
    }
}
