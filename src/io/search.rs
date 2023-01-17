use crate::utils::cli_result::CliResult;
use crate::utils::constants::TERMINATOR;
use crate::utils::file::file_or_stdout_wtr;
use crate::utils::filename::new_file;

use regex::RegexBuilder;
use std::io::{self, BufRead, BufWriter, Write};

pub fn run(pattern: &str, no_header: bool, export: bool) -> CliResult {
    // wtr and rdr
    let out = new_file("searched.csv");
    let f = file_or_stdout_wtr(export, &out)?;
    let mut wtr = BufWriter::new(f);

    // regex
    let re = RegexBuilder::new(pattern).case_insensitive(true).build()?;

    // read
    let mut handle = io::stdin().lock().lines();

    if !no_header {
        match handle.next() {
            Some(row) => write_row(&mut wtr, export, &row?)?,
            None => return Ok(()),
        }
    }

    let mut matched = 0;
    for l in handle {
        let l = l?;
        if re.is_match(&l) {
            matched += 1;
            write_row(&mut wtr, export, &l)?;
        }
    }

    if export {
        println!("Matched rows: {}", matched);
        println!("Saved to file: {}", out.display())
    }

    Ok(())
}

fn write_row(wtr: &mut BufWriter<Box<dyn Write>>, export: bool, row: &str) -> CliResult {
    if export {
        wtr.write_all(row.as_bytes())?;
        wtr.write_all(TERMINATOR)?;
    } else {
        println!("{}", row)
    }

    Ok(())
}
