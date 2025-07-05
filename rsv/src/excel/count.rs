use crate::args::Count;
use rsv_lib::utils::cli_result::CliResult;
use rsv_lib::utils::progress::Progress;
use rsv_lib::utils::reader::ExcelReader;

impl Count {
    pub fn excel_run(&self) -> CliResult {
        // progress
        let mut prog = Progress::new();

        // open file and count
        let range = ExcelReader::new(&self.path(), self.sheet)?;
        let mut n = range.len();

        // default to have a header
        if !self.no_header && n > 0 {
            n -= 1;
        }

        println!("{n}");
        prog.print_elapsed_time();

        Ok(())
    }
}
