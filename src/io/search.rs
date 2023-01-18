use crate::utils::filename::new_file;
use crate::utils::{cli_result::CliResult, writer::Writer};

use regex::RegexBuilder;
use std::io::{self, BufRead};

pub fn run(pattern: &str, no_header: bool, export: bool) -> CliResult {
    // wtr and rdr
    let out = new_file("searched.csv");
    let mut wtr = Writer::file_or_stdout(export, &out)?;

    // regex
    let re = RegexBuilder::new(pattern).case_insensitive(true).build()?;

    // read
    let mut handle = io::stdin().lock().lines();

    if !no_header {
        match handle.next() {
            Some(r) => wtr.write_line(&r?)?,
            None => return Ok(()),
        }
    }

    let mut matched = 0;
    for l in handle {
        let l = l?;
        if re.is_match(&l) {
            matched += 1;
            wtr.write_line(&l)?;
        }
    }

    if export {
        println!("Matched rows: {}", matched);
        println!("Saved to file: {}", out.display())
    }

    Ok(())
}
