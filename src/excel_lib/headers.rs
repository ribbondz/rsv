use std::path::PathBuf;

use crate::utils::excel::datatype_vec_to_string_vec;
use crate::utils::reader::ExcelReader;
use crate::utils::return_result::{CliResultData, ResultData};

pub fn excel_headers(file: &PathBuf, sheet: usize) -> CliResultData {
    // open file and header
    let mut range = ExcelReader::new(&file, sheet)?;

    let mut out = ResultData::from_header(vec!["column_name".to_string()]);
    if let Some(r) = range.next() {
        out.insert_record(datatype_vec_to_string_vec(r));
    }

    Ok(Some(out))
}
