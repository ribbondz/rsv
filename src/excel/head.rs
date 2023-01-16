use std::path::Path;

use crate::utils::cli_result::CliResult;
use crate::utils::excel::datatype_vec_to_string_vec;
use crate::utils::excel_reader::ExcelReader;
use crate::utils::util::print_table;

pub fn run(path: &Path, sheet: usize, no_header: bool, n: usize) -> CliResult {
    let range = ExcelReader::new(path, sheet)?;

    // show head n
    let r = range
        .iter()
        .take(n + 1 - no_header as usize)
        .map(datatype_vec_to_string_vec)
        .collect::<Vec<_>>();

    // tabled print
    print_table(r);

    Ok(())
}
