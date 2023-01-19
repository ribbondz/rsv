use crate::utils::cli_result::CliResult;
use crate::utils::excel::datatype_vec_to_string_vec;
use crate::utils::excel_reader::ExcelReader;
use crate::utils::table::Table;
use std::path::Path;

pub fn run(path: &Path, sheet: usize, no_header: bool, n: usize, tabled: bool) -> CliResult {
    let range = ExcelReader::new(path, sheet)?;

    // show head n
    let r = range
        .iter()
        .take(n + 1 - no_header as usize)
        .map(datatype_vec_to_string_vec)
        .collect::<Vec<_>>();

    // tabled print
    if tabled {
        Table::from_records(r).print_blank()?;
    } else {
        r.iter().for_each(|i| println!("{}", i.join(",")))
    }

    Ok(())
}
