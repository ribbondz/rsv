use crate::utils::return_result::{CliResultData, ResultData};
use crate::utils::row_split::CsvRowSplitter;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

pub fn csv_head(
    file: &PathBuf,
    no_header: bool,
    sep: char,
    quote: char,
    n: usize,
) -> CliResultData {
    let mut out = ResultData::new();

    // show head n
    let mut lines = BufReader::new(File::open(file)?)
        .lines()
        .take(n + 1 - no_header as usize);

    // Process header
    if let Some(Ok(h)) = lines.next() {
        let h = CsvRowSplitter::new(&h, sep, quote).collect_owned();
        out.insert_header(h);
    }

    lines.for_each(|r| {
        if let Ok(r) = r {
            let r = CsvRowSplitter::new(&r, sep, quote).collect_owned();
            out.insert_record(r);
        }
    });

    Ok(Some(out))
}
