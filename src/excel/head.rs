use crate::utils::cli_result::CliResult;
use crate::utils::constants::COMMA;
use crate::utils::filename::new_path;
use crate::utils::reader::ExcelReader;
use crate::utils::writer::Writer;
use std::path::Path;

pub fn run(path: &Path, sheet: usize, no_header: bool, n: usize, export: bool) -> CliResult {
    let out = new_path(path, "-head").with_extension("csv");
    let mut wtr = Writer::file_or_stdout(export, &out)?;
    let range = ExcelReader::new(path, sheet)?;

    // show head n
    range
        .iter()
        .take(n + 1 - (no_header as usize))
        .for_each(|r| {
            wtr.write_excel_line_unchecked(r, COMMA);
        });

    if export {
        println!("Saved to file: {}", out.display())
    }

    Ok(())
}
