use crate::args::Excel2csv;
use rsv_lib::utils::{cli_result::CliResult, util::werr_exit};

impl Excel2csv {
    pub fn csv_run(&self) -> CliResult {
        werr_exit!(
            "Error: File <{}> is not an excel file.",
            self.path().display()
        )
    }
}
