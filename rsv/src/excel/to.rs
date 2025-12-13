use std::path::Path;

use crate::args::To;
use rsv_lib::utils::cli_result::CliResult;
use rsv_lib::utils::reader::ExcelReader;
use rsv_lib::utils::to::{is_valid_plain_text, out_filename};
use rsv_lib::utils::writer::Writer;

impl To {
    pub fn excel_run(&self) -> CliResult {
        let out = self.out.to_lowercase();
        let outsep = if out.ends_with("tsv") {
            '\t'.to_string()
        } else {
            ','.to_string()
        };

        if !is_valid_plain_text(&out) {
            let msg = format!("output file format of <{out}> is un-recognized.");
            return Err(msg.into());
        }

        self.excel_to_csv(&out, &outsep)?;

        Ok(())
    }

    pub fn excel_to_csv(&self, out: &str, sep: &str) -> CliResult {
        // out path
        let out = out_filename(out);

        // rdr and wtr
        let range = ExcelReader::new(&self.path(), self.sheet)?;
        let mut wtr = Writer::new(Path::new(&out))?;

        // Convert that str to bytes
        let sep_bytes = sep.as_bytes();

        for r in range.iter() {
            wtr.write_excel_line(r, sep_bytes)?;
        }

        println!("Saved to file: {}", out.display());

        Ok(())
    }
}
