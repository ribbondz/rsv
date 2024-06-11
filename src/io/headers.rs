use crate::{
    args::Headers,
    utils::{cli_result::CliResult, util::valid_sep},
};
use std::io::{stdin, BufRead};

impl Headers {
    pub fn io_run(&self) -> CliResult {
        let sep = valid_sep(&self.sep);

        // open file and header
        if let Some(r) = stdin().lock().lines().next() {
            r?.split(&sep)
                .enumerate()
                .for_each(|(u, r)| println!(" {u:<5}{r}"));
        }

        Ok(())
    }
}
