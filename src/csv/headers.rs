use crate::args::Headers;
use crate::utils::cli_result::CliResult;
use crate::utils::util::valid_sep;
use std::fs::File;
use std::io::{BufRead, BufReader};

impl Headers {
    pub fn csv_run(&self) -> CliResult {
        let sep = valid_sep(&self.sep);

        // open file and header
        let mut rdr = BufReader::new(File::open(self.path())?).lines();

        if let Some(r) = rdr.next() {
            r?.split(&sep)
                .enumerate()
                .for_each(|(i, v)| println!(" {i:<5}{v}"));
        };

        Ok(())
    }
}
