use crate::args::Headers;
use rsv_lib::utils::cli_result::CliResult;
use std::fs::File;
use std::io::{BufRead, BufReader};

impl Headers {
    pub fn csv_run(&self) -> CliResult {
        // open file and header
        let mut rdr = BufReader::new(File::open(self.path())?).lines();

        if let Some(r) = rdr.next() {
            self.split(&r?)
                .enumerate()
                .for_each(|(i, v)| println!(" {i:<5}{v}"));
        };

        Ok(())
    }
}
