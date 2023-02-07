use crate::utils::cli_result::CliResult;
use crate::utils::to::{excel_to_csv, is_valid_plain_text};
use std::path::Path;

pub fn run(path: &Path, sheet: usize, out: &str, outsep: &str) -> CliResult {
    let out = out.to_lowercase();
    let outsep = if out.ends_with("tsv") {
        '\t'.to_string()
    } else {
        outsep.to_owned()
    };

    if !is_valid_plain_text(&out) {
        let msg = format!("output file format of <{out}> is un-recognized.");
        return Err(msg.into());
    }

    excel_to_csv(path, sheet, &outsep, &out)?;

    Ok(())
}
