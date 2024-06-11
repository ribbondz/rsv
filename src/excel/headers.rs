use crate::args::Headers;
use crate::utils::cli_result::CliResult;
use crate::utils::reader::ExcelReader;

impl Headers {
    pub fn excel_run(&self) -> CliResult {
        // open file and header
        let mut range = ExcelReader::new(&self.path(), self.sheet)?;

        if let Some(r) = range.next() {
            r.iter()
                .enumerate()
                .for_each(|(u, r)| println!(" {u:<5}{r}"));
        }

        Ok(())
    }
}
