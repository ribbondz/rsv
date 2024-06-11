use crate::args::Head;
use crate::utils::cli_result::CliResult;
use crate::utils::constants::COMMA;
use crate::utils::filename::new_path;
use crate::utils::reader::ExcelReader;
use crate::utils::writer::Writer;

impl Head {
    pub fn excel_run(&self) -> CliResult {
        let path = &self.path();
        let out = new_path(path, "-head").with_extension("csv");
        let mut wtr = Writer::file_or_stdout(self.export, &out)?;
        let range = ExcelReader::new(path, self.sheet)?;

        // show head n
        range
            .iter()
            .take(self.n + 1 - (self.no_header as usize))
            .for_each(|r| {
                wtr.write_excel_line_unchecked(r, COMMA);
            });

        if self.export {
            println!("Saved to file: {}", out.display())
        }

        Ok(())
    }
}
