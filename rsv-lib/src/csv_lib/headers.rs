use crate::utils::return_result::{CliResultData, ResultData};
use crate::utils::row_split::CsvRowSplitter;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

pub fn csv_headers(file: &PathBuf, sep: char, quote: char) -> CliResultData {
    let mut out = ResultData::new();

    // open file and header
    let mut rdr = BufReader::new(File::open(&file)?).lines();

    out.insert_header(vec!["column_name".to_string()]);
    if let Some(r) = rdr.next() {
        CsvRowSplitter::new(&r?, sep, quote).for_each(|v| out.insert_record(vec![v.to_string()]));
    };

    Ok(Some(out))
}
