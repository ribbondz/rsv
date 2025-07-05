use crate::args::To;
use rsv_lib::utils::cli_result::CliResult;
use rsv_lib::utils::to::{csv_or_io_to_csv, csv_to_excel, is_valid_excel, is_valid_plain_text};

impl To {
    pub fn csv_run(&self) -> CliResult {
        let path = &self.path();
        let out = self.out.to_lowercase();

        match out.as_str() {
            v if is_valid_plain_text(v) => csv_or_io_to_csv(Some(path), &out)?,
            v if is_valid_excel(v) => {
                csv_to_excel(path, self.sep, self.quote, &out, self.no_header)?
            }
            _ => return Err(format!("output file format <{out}> is un-recognized.").into()),
        };

        Ok(())
    }
}
