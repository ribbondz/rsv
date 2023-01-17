use crate::utils::{cli_result::CliResult, excel_reader::ExcelReader, util::print_tabled};
use std::path::Path;

pub fn run(path: &Path, sheet: usize) -> CliResult {
    // rdr
    let range = ExcelReader::new(path, sheet)?;

    let rows = range
        .iter()
        .map(|r| r.iter().map(|i| i.to_string()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    if !rows.is_empty() {
        print_tabled(rows);
    }

    Ok(())
}
