mod csv_lib;
mod excel_lib;
mod general_lib;
pub mod utils;

pub use crate::utils::return_result::ResultData;
use crate::utils::{file::is_excel, filename::full_path, return_result::CliResultData};
// general
pub use general_lib::size::file_size;

// count
use csv_lib::count::csv_count;
use excel_lib::count::excel_count;
pub fn file_count(file: &str, no_header: bool, sheet: usize) -> CliResultData {
    let path = full_path(file);
    match is_excel(&path) {
        true => excel_count(&path, no_header, sheet),
        false => csv_count(&path, no_header),
    }
}

// head
use csv_lib::head::csv_head;
use excel_lib::head::excel_head;
pub fn file_head(
    file: &str,
    no_header: bool,
    sep: char,
    quote: char,
    sheet: usize,
    n: usize,
) -> CliResultData {
    let path = full_path(file);
    match is_excel(&path) {
        true => excel_head(&path, no_header, sheet, n),
        false => csv_head(&path, no_header, sep, quote, n),
    }
}

// headers
use csv_lib::headers::csv_headers;
use excel_lib::headers::excel_headers;
pub fn file_headers(file: &str, sep: char, quote: char, sheet: usize) -> CliResultData {
    let path = full_path(file);
    match is_excel(&path) {
        true => excel_headers(&path, sheet),
        false => csv_headers(&path, sep, quote),
    }
}

use csv_lib::stats::csv_stats;
use excel_lib::stats::excel_stats;

pub fn file_stats(
    file: &str,
    sep: char,
    quote: char,
    no_header: bool,
    cols: String,
    sheet: usize,
    text_columns: &Vec<usize>,
) -> CliResultData {
    let path = full_path(file);
    match is_excel(&path) {
        true => excel_stats(&path, no_header, cols, sheet),
        false => csv_stats(&path, sep, quote, no_header, cols, text_columns),
    }
}
