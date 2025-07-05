use crate::utils::excel::datatype_vec_to_string_vec;
use crate::utils::reader::ExcelReader;
use crate::utils::return_result::{CliResultData, ResultData};
use std::path::PathBuf;

pub fn excel_head(file: &PathBuf, no_header: bool, sheet: usize, n: usize) -> CliResultData {
    let range = ExcelReader::new(&file, sheet)?;

    let mut out = ResultData::new();

    // show head n
    let mut lines = range.iter().take(n + 1 - (no_header as usize));

    if let Some(header) = lines.next() {
        out.insert_header(datatype_vec_to_string_vec(header));
    }

    lines.for_each(|r| {
        out.insert_record(datatype_vec_to_string_vec(r));
    });

    Ok(Some(out))
}
