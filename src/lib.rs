use crate::{
    args::{Count, Head, Headers, Size},
    utils::return_result::CliResultData,
};

pub mod args;
pub mod csv;
pub mod csv_lib;
pub mod excel;
pub mod io;
pub mod utils;

pub fn csv_count(file: &str, no_header: bool) -> CliResultData {
    let option = Count {
        filename: Some(file.to_owned()),
        no_header,
        sheet: 0,
    };
    Count::csv_run_lib(&option)
}

pub fn csv_head(file: &str, sep: char, quote: char, no_header: bool, n: usize) -> CliResultData {
    let option = Head {
        filename: Some(file.to_owned()),
        no_header,
        n,
        sheet: 0,
        export: false,
        sep,
        quote,
    };
    Head::csv_run_lib(&option)
}

pub fn csv_headers(file: &str, sep: char, quote: char) -> CliResultData {
    let option = Headers {
        filename: Some(file.to_owned()),
        sheet: 0,
        sep,
        quote,
    };
    Headers::csv_run_lib(&option)
}

pub fn csv_size(file: &str) -> CliResultData {
    let option = Size {
        filename: Some(file.to_owned()),
    };
    Size::csv_run_lib(&option)
}
