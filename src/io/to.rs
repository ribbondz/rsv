use crate::utils::cli_result::CliResult;
use crate::utils::to::{csv_or_io_to_csv, io_to_excel, is_valid_excel, is_valid_plain_text};

pub fn run(sep: &str, no_header: bool, out: &str, outsep: &str) -> CliResult {
    let out = out.to_lowercase();
    let outsep = if out.ends_with("tsv") {
        '\t'.to_string()
    } else {
        outsep.to_owned()
    };

    match out.as_str() {
        v if is_valid_plain_text(v) => csv_or_io_to_csv(None, sep, &outsep, &out)?,
        v if is_valid_excel(v) => io_to_excel(sep, no_header, &out)?,
        _ => return Err(format!("output file format <{out}> is un-recognized.").into()),
    };

    Ok(())
}
