use crate::args::Sort;
use rsv_lib::utils::cli_result::CliResult;
use rsv_lib::utils::filename::new_path;
use rsv_lib::utils::sort::SortColumns;
use rsv_lib::utils::writer::Writer;
use std::fs::File;
use std::io::{BufRead, BufReader};

impl Sort {
    pub fn csv_run(&self) -> CliResult {
        let path = &self.path();

        // rdr and wtr
        let mut rdr = BufReader::new(File::open(path)?).lines();
        let out = new_path(path, "-sorted");
        let mut wtr = Writer::file_or_stdout(self.export, &out)?;

        // cols
        let cols = SortColumns::from(&self.cols)?;

        // header
        if !self.no_header {
            let Some(r) = rdr.next() else { return Ok(()) };
            wtr.write_str_unchecked(r?);
        }

        // lines
        let lines = rdr.map_while(Result::ok).collect::<Vec<_>>();

        // sort
        cols.sort_and_write(&lines, self.sep, self.quote, &mut wtr)?;

        if self.export {
            println!("Saved to file: {}", out.display())
        }

        Ok(())
    }
}
