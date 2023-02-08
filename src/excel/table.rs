use crate::utils::excel::datatype_vec_to_string_vec;
use crate::utils::{cli_result::CliResult, excel_reader::ExcelReader, table::Table};
use std::path::Path;

pub fn run(path: &Path, sheet: usize) -> CliResult {
    // rdr
    let range = ExcelReader::new(path, sheet)?;

    let rows = range
        .iter()
        .map(datatype_vec_to_string_vec)
        .collect::<Vec<_>>();

    Table::from_records(rows).print_blank()?;

    Ok(())
}
