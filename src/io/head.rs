use crate::utils::{cli_result::CliResult, filename::new_file, table::Table, writer::Writer};
use std::io::{stdin, BufRead};

pub fn run(no_header: bool, sep: &str, n: usize, tabled: bool, export: bool) -> CliResult {
    let out = new_file("sorted.csv");
    let mut wtr = Writer::file_or_stdout(export, &out)?;

    // show head n
    let r = stdin()
        .lock()
        .lines()
        .take(n + 1 - no_header as usize)
        .filter_map(|i| i.ok())
        .collect::<Vec<_>>();

    // tabled or not
    if export || !tabled {
        wtr.write_lines_unchecked(&r);
    } else {
        Table::from_rows(&r, sep).print_blank()?;
    }

    if export {
        println!("Saved to file: {}", out.display())
    }

    Ok(())
}
