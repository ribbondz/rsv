use crate::utils::cli_result::CliResult;
use crate::utils::constants::COMMA;
use crate::utils::excel_reader::ExcelReader;
use crate::utils::filename::new_path;
use crate::utils::sort::SortColumns;
use crate::utils::writer::Writer;
use std::path::Path;

pub fn run(path: &Path, sheet: usize, no_header: bool, cols: &str, export: bool) -> CliResult {
    // open file and count
    let mut range = ExcelReader::new(path, sheet)?;
    let out = new_path(path, "-sorted").with_extension("csv");
    let mut wtr = Writer::file_or_stdout(export, &out)?;

    // cols
    let cols = SortColumns::from(cols)?;

    // header
    if !no_header {
        match range.next() {
            Some(v) => wtr.write_excel_line_unchecked(v, COMMA),
            None => return Ok(()),
        }
    }

    // lines
    let mut lines = range
        .iter()
        .skip(range.next_called)
        .map(|i| i.iter().map(|j| j.to_string()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // sort
    cols.sort_excel_and_write(&mut lines, &mut wtr)?;

    if export {
        println!("Saved to file: {}", out.display())
    }

    Ok(())
}
