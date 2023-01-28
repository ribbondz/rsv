use crate::utils::cli_result::CliResult;
use crate::utils::to::{csv_or_io_to_csv, csv_to_excel, is_valid_excel, is_valid_plain_text};
use std::path::Path;

pub fn run(path: &Path, no_header: bool, out: &str, sep: &str, outsep: &str) -> CliResult {
    let out = out.to_lowercase();
    let outsep = if out.ends_with("tsv") {
        '\t'.to_string()
    } else {
        outsep.to_owned()
    };

    match out.as_str() {
        v if is_valid_plain_text(v) => csv_or_io_to_csv(Some(path), sep, &outsep, &out)?,
        v if is_valid_excel(v) => csv_to_excel(path, sep, &out, no_header)?,
        _ => return Err(format!("output file format <{}> is un-recognized.", out).into()),
    };

    Ok(())
}
