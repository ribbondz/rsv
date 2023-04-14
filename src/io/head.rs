use std::io::{stdin, BufRead};

use crate::utils::{cli_result::CliResult, filename::new_file, writer::Writer};

pub fn run(no_header: bool, n: usize, export: bool) -> CliResult {
    let out = new_file("sorted.csv");
    let mut wtr = Writer::file_or_stdout(export, &out)?;

    // show head n
    // open file and header
    stdin()
        .lock()
        .lines()
        .take(n + 1 - no_header as usize)
        .for_each(|r| {
            if let Ok(r) = r {
                wtr.write_line_unchecked(&r);
            }
        });

    if export {
        println!("Saved to file: {}", out.display())
    }

    Ok(())
}
