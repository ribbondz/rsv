use crate::utils::reader::ExcelReader;
use crate::utils::return_result::{CliResultData, ResultData};
use std::path::PathBuf;
extern crate bytecount;

pub fn excel_count(file: &PathBuf, no_header: bool, sheet: usize) -> CliResultData {
    // open file and count
    let range = ExcelReader::new(&file, sheet)?;
    let mut n = range.len();

    // default to have a header
    if !no_header && n > 0 {
        n -= 1;
    }

    Ok(Some(ResultData {
        header: vec!["count".to_string()],
        data: vec![vec![n.to_string()]],
    }))
}
