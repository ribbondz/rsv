use crate::utils::cli_result::CliResult;
use crate::utils::excel::datatype_vec_to_string_vec;
use crate::utils::filename::new_path;
use crate::utils::reader::ExcelReader;
use crate::utils::writer::Writer;
use std::path::Path;

pub fn run(path: &Path, sheet: usize, no_header: bool, n: usize, export: bool) -> CliResult {
    let out = new_path(path, "-head").with_extension("csv");
    let mut wtr = Writer::file_or_stdout(export, &out)?;
    let range = ExcelReader::new(path, sheet)?;

    // show head n
    let r = range
        .iter()
        .take(n + 1 - (no_header as usize))
        .map(datatype_vec_to_string_vec)
        .collect::<Vec<_>>();

    // tabled print
    wtr.write_lines_by_field_unchecked(&r, None);

    if export {
        println!("Saved to file: {}", out.display())
    }

    Ok(())
}
