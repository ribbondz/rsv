use crate::args::Sort;
use crate::utils::cli_result::CliResult;
use crate::utils::filename::new_path;
use crate::utils::sort::SortColumns;
use crate::utils::util::valid_sep;
use crate::utils::writer::Writer;
use std::fs::File;
use std::io::{BufRead, BufReader};

impl Sort {
    pub fn csv_run(&self) -> CliResult {
        let sep = valid_sep(&self.sep);
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
        let lines = rdr.filter_map(|i| i.ok()).collect::<Vec<_>>();

        // sort
        cols.sort_and_write(&lines, &sep, &mut wtr)?;

        if self.export {
            println!("Saved to file: {}", out.display())
        }

        Ok(())
    }
}
