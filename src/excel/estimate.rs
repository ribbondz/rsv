use crate::args::{Count, Estimate};
use crate::utils::cli_result::CliResult;

impl Estimate {
    pub fn excel_run(&self) -> CliResult {
        Count {
            filename: self.filename.clone(),
            no_header: false,
            sheet: self.sheet,
            return_result: false,
        }
        .excel_run()
    }
}
