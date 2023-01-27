use crate::utils::cli_result::CliResult;
use crate::utils::to::{io_to_csv, io_to_excel, is_valid_excel, is_valid_plain_text};

pub fn run(sep: &str, out: &str, outsep: &str) -> CliResult {
    let out = out.to_lowercase();
    let outsep = if out.ends_with("tsv") {
        '\t'.to_string()
    } else {
        outsep.to_owned()
    };

    match out.as_str() {
        v if is_valid_plain_text(v) => io_to_csv(sep, &outsep, &out)?,
        v if is_valid_excel(v) => io_to_excel(sep, &out)?,
        _ => return Err("the out file format is not supported.".into()),
    };

    Ok(())
}
