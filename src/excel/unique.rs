use crate::utils::cli_result::CliResult;
use crate::utils::column::Columns;
use crate::utils::constants::COMMA;
use crate::utils::excel::datatype_vec_to_string;
use crate::utils::filename::new_path;
use crate::utils::reader::ExcelReader;
use crate::utils::writer::Writer;
use std::path::Path;

pub fn run(
    path: &Path,
    sheet: usize,
    no_header: bool,
    cols: &str,
    keep_last: bool,
    export: bool,
) -> CliResult {
    let all_cols = cols == "-1";

    // wtr and rdr
    let out = new_path(path, "-drop-duplicates").with_extension("csv");
    let mut wtr = Writer::file_or_stdout(export, &out)?;
    let mut rdr = ExcelReader::new(path, sheet)?;

    // cols
    let cols = if all_cols {
        None
    } else {
        Some(Columns::new(cols).total_col(rdr.column_n()).parse())
    };

    // header
    if !no_header {
        let Some(r) = rdr.next() else { return Ok(()) };
        wtr.write_excel_line_unchecked(r, COMMA);
    }

    // read
    match (keep_last, all_cols) {
        (true, true) => keep_last_and_all_cols(&mut rdr, &mut wtr)?,
        (true, false) => keep_last_and_partial_cols(&mut rdr, &mut wtr, cols.unwrap())?,
        (false, true) => keep_first_and_all_cols(&mut rdr, &mut wtr)?,
        (false, false) => keep_first_and_partial_cols(&mut rdr, &mut wtr, cols.unwrap())?,
    }

    if export {
        println!("\nSaved to file: {}", out.display())
    }

    Ok(())
}

fn keep_first_and_all_cols(rdr: &mut ExcelReader, wtr: &mut Writer) -> CliResult {
    let mut unique_holder = ahash::HashSet::default();
    for r in rdr.iter().skip(rdr.next_called) {
        let r = datatype_vec_to_string(r);
        if !unique_holder.contains(&r) {
            wtr.write_str_unchecked(&r);
            unique_holder.insert(r);
        }
    }

    Ok(())
}

fn keep_first_and_partial_cols(
    rdr: &mut ExcelReader,
    wtr: &mut Writer,
    cols: Columns,
) -> CliResult {
    let mut unique_holder = ahash::HashSet::default();
    for r in rdr.iter().skip(rdr.next_called) {
        let p = cols.select_owned_string_from_excel_datatype(r);
        if !unique_holder.contains(&p) {
            wtr.write_excel_line_unchecked(r, COMMA);
            unique_holder.insert(p);
        }
    }

    Ok(())
}

fn keep_last_and_all_cols(rdr: &mut ExcelReader, wtr: &mut Writer) -> CliResult {
    let mut unique_n = ahash::HashMap::default();

    // first scan to locate record location
    for r in rdr.iter().skip(rdr.next_called) {
        let r = datatype_vec_to_string(r);
        *unique_n.entry(r).or_insert(0) += 1;
    }

    // second scan
    for r in rdr.iter().skip(rdr.next_called) {
        let r = datatype_vec_to_string(r);
        if unique_n[&r] == 1 {
            wtr.write_str_unchecked(r);
        } else {
            *unique_n.entry(r).or_insert(0) -= 1;
        }
    }

    Ok(())
}

fn keep_last_and_partial_cols(rdr: &mut ExcelReader, wtr: &mut Writer, cols: Columns) -> CliResult {
    let mut unique_n = ahash::HashMap::default();

    // first scan to locate record location
    for r in rdr.iter().skip(rdr.next_called) {
        let p = cols.select_owned_string_from_excel_datatype(r);
        *unique_n.entry(p).or_insert(0) += 1;
    }

    // second scan
    for r in rdr.iter().skip(rdr.next_called) {
        let p = cols.select_owned_string_from_excel_datatype(r);
        if unique_n[&p] == 1 {
            wtr.write_excel_line_unchecked(r, COMMA);
        } else {
            *unique_n.entry(p).or_insert(0) -= 1;
        }
    }

    Ok(())
}
