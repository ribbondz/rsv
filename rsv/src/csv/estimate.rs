use crate::args::Estimate;
use rsv_lib::utils::cli_result::CliResult;
use std::fs::File;
use std::io::{BufRead, BufReader};

impl Estimate {
    pub fn csv_run(&self) -> CliResult {
        // read 20000 lines to estimate bytes per line
        let file = File::open(self.path())?;
        let filesize = file.metadata()?.len() as f64;

        let mut total_bytes = 0;
        let mut n = 0;
        for l in BufReader::new(file).lines().skip(1) {
            total_bytes += l.unwrap().len() + 1;
            n += 1;
            if n > 20000 {
                break;
            }
        }

        // estimate line count
        let mut estimate_n = filesize / ((total_bytes as f64) / (n as f64));

        // default to have a header
        if estimate_n > 1.0 {
            estimate_n -= 1.0;
        }

        println!("{}", estimate_n as usize);

        Ok(())
    }
}
