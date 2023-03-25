use crate::utils::cli_result::CliResult;
use crate::utils::column::Columns;
use crate::utils::constants::COMMA;
use crate::utils::excel::datatype_vec_to_string_vec;
use crate::utils::filename::new_path;
use crate::utils::filter::Filter;
use crate::utils::reader::ExcelReader;
use crate::utils::writer::Writer;
use std::path::Path;

pub fn run(
    path: &Path,
    no_header: bool,
    sheet: usize,
    cols: &str,
    filter: &str,
    export: bool,
) -> CliResult {
    // out path
    let out = new_path(path, "-selected").with_extension("csv");

    // filters
    let filter = Filter::new(filter);

    // open file
    let mut wtr = Writer::file_or_stdout(export, &out)?;
    let mut rdr = ExcelReader::new(path, sheet)?;

    // cols
    let cols = Columns::new(cols).total_col(rdr.column_n()).parse();

    // header
    if !no_header {
        let Some(r) = rdr.next() else {
            return Ok(())
        };
        if cols.select_all {
            wtr.write_excel_line_unchecked(r, COMMA);
        } else {
            let r = cols.iter().map(|&i| r[i].to_string()).collect::<Vec<_>>();
            wtr.write_line_by_field_unchecked(&r, None);
        }
    }

    // read
    rdr.iter().skip(rdr.next_called).for_each(|r| {
        let r = datatype_vec_to_string_vec(r);
        if filter.excel_record_is_valid(&r) {
            match cols.select_all {
                true => wtr.write_line_by_field_unchecked(&r, None),
                false => {
                    let r = cols.iter().map(|&i| &r[i]).collect::<Vec<_>>();
                    wtr.write_line_by_field_unchecked(&r, None);
                }
            }
        }
    });

    if export {
        println!("\nSaved to file: {}", out.display())
    }

    Ok(())
}
