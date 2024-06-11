use crate::args::Head;
use crate::utils::cli_result::CliResult;
use crate::utils::filename::new_path;
use crate::utils::writer::Writer;
use std::fs::File;
use std::io::{BufRead, BufReader};

impl Head {
    pub fn csv_run(&self) -> CliResult {
        let path = self.path();
        let out = new_path(&path, "-head");
        let mut wtr = Writer::file_or_stdout(self.export, &out)?;

        // show head n
        BufReader::new(File::open(path)?)
            .lines()
            .take(self.n + 1 - self.no_header as usize)
            .for_each(|r| {
                if let Ok(r) = r {
                    wtr.write_str_unchecked(&r);
                }
            });

        if self.export {
            println!("Saved to file: {}", out.display())
        }

        Ok(())
    }
}
