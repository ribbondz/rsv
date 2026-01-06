use crate::args::Count;
use rsv_lib::utils::cli_result::CliResult;
use std::io::{BufRead, stdin};

impl Count {
    pub fn io_run(&self) -> CliResult {
        // progress
        let mut n = stdin().lock().lines().count();

        if !self.no_header && n > 0 {
            n -= 1;
        }

        println!("{n}");

        Ok(())
    }
}
