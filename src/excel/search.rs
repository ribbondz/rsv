use crate::utils::cli_result::CliResult;
use crate::utils::constants::COMMA;
use crate::utils::excel::datatype_vec_to_string_vec;
use crate::utils::filename::new_path;
use crate::utils::reader::ExcelReader;
use crate::utils::regex::Re;
use crate::utils::writer::Writer;
use std::path::Path;

pub fn run(path: &Path, sheet: usize, pattern: &str, no_header: bool, export: bool) -> CliResult {
    // wtr and rdr
    let out = new_path(path, "-searched").with_extension("csv");
    let mut wtr = Writer::file_or_stdout(export, &out)?;
    let mut rdr = ExcelReader::new(path, sheet)?;

    // header
    if !no_header {
        let Some(r) = rdr.next() else {
            return Ok(())
        };
        wtr.write_excel_line_unchecked(r, COMMA)
    };

    // regex search
    let re = Re::new(pattern)?;
    let mut matched = 0;

    // read file
    rdr.iter().skip(rdr.next_called).for_each(|r| {
        let r = datatype_vec_to_string_vec(r);
        if r.iter().any(|i| re.is_match(i)) {
            // pipeline could be closed,
            // e.g., when rsv head take enough items
            wtr.write_line_by_field_unchecked(&r, None);
            matched += 1;
        }
    });

    if export {
        println!("\nMatched rows: {matched}");
        println!("Saved to file: {}", out.display());
    }

    Ok(())
}
