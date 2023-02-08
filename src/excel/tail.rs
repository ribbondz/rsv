use crate::utils::cli_result::CliResult;
use crate::utils::constants::COMMA;
use crate::utils::reader::ExcelReader;
use crate::utils::filename::new_path;
use crate::utils::writer::Writer;
use std::path::Path;

pub fn run(path: &Path, sheet: usize, no_header: bool, n: usize, export: bool) -> CliResult {
    let out = new_path(path, "-tail").with_extension("csv");
    let mut wtr = Writer::file_or_stdout(export, &out)?;
    let mut range = ExcelReader::new(path, sheet)?;

    // header
    if !no_header {
        let Some(r) = range.next() else {
            return Ok(())
        };
        wtr.write_excel_line_unchecked(r, COMMA);
    }

    // show head n
    range
        .iter()
        .skip(range.next_called)
        .rev()
        .take(n)
        .rev()
        .for_each(|r| wtr.write_excel_line_unchecked(r, COMMA));

    if export {
        println!("Saved to file: {}", out.display())
    }

    Ok(())
}
