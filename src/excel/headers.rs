use crate::utils::cli_result::CliResult;
use crate::utils::excel_reader::ExcelReader;
use std::path::Path;

pub fn run(path: &Path, sheet: usize) -> CliResult {
    // open file and header
    let mut range = ExcelReader::new(path, sheet)?;

    if let Some(r) = range.next() {
        r.iter()
            .enumerate()
            .for_each(|(u, r)| println!(" {u:<5}{r}"));
    }

    Ok(())
}
