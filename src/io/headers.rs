use crate::{
    args::Headers,
    utils::{cli_result::CliResult, row_split::CsvRow},
};
use std::io::{stdin, BufRead};

impl Headers {
    pub fn io_run(&self) -> CliResult {
        // open file and header
        if let Some(r) = stdin().lock().lines().next() {
            CsvRow::new(&r?)
                .split(self.sep, self.quote)
                .enumerate()
                .for_each(|(u, r)| println!(" {u:<5}{r}"));
        }

        Ok(())
    }
}
