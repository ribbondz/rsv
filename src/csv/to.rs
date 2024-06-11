use crate::args::To;
use crate::utils::cli_result::CliResult;
use crate::utils::to::{csv_or_io_to_csv, csv_to_excel, is_valid_excel, is_valid_plain_text};
use crate::utils::util::valid_sep;

impl To {
    pub fn csv_run(&self) -> CliResult {
        let sep = valid_sep(&self.sep);
        let path = &self.path();
        let out = self.out.to_lowercase();
        let outsep = if out.ends_with("tsv") {
            '\t'.to_string()
        } else {
            self.outsep.to_owned()
        };

        match out.as_str() {
            v if is_valid_plain_text(v) => csv_or_io_to_csv(Some(path), &sep, &outsep, &out)?,
            v if is_valid_excel(v) => csv_to_excel(path, &sep, &out, self.no_header)?,
            _ => return Err(format!("output file format <{out}> is un-recognized.").into()),
        };

        Ok(())
    }
}
