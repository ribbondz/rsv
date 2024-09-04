use crate::args::To;
use crate::utils::cli_result::CliResult;
use crate::utils::to::{csv_or_io_to_csv, io_to_excel, is_valid_excel, is_valid_plain_text};

impl To {
    pub fn io_run(&self) -> CliResult {
        let out = self.out.to_lowercase();

        match out.as_str() {
            v if is_valid_plain_text(v) => csv_or_io_to_csv(None, &out)?,
            v if is_valid_excel(v) => io_to_excel(self, &out)?,
            _ => return Err(format!("output file format <{out}> is un-recognized.").into()),
        };

        Ok(())
    }
}
