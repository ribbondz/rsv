use crate::{
    args::{Count, Estimate},
    utils::cli_result::CliResult,
};

impl Estimate {
    pub fn io_run(&self) -> CliResult {
        Count {
            filename: self.filename.clone(),
            no_header: false,
            sheet: self.sheet,
        }
        .io_run()
    }
}
