use crate::utils::sort::SortColumns;
use crate::utils::writer::Writer;
use crate::utils::{cli_result::CliResult, filename::new_file};
use std::io::{stdin, BufRead};

pub fn run(no_header: bool, sep: &str, cols: &str, export: bool) -> CliResult {
    // rdr and wtr
    let mut rdr = stdin().lock().lines();
    let out = new_file("sorted.csv");
    let mut wtr = Writer::file_or_stdout(export, &out)?;

    // cols
    let cols = SortColumns::from(cols);

    // header
    if !no_header {
        match rdr.next() {
            Some(v) => wtr.write_line_unchecked(v?),
            None => return Ok(()),
        }
    }

    // lines
    let lines = rdr.filter_map(|i| i.ok()).collect::<Vec<_>>();

    // sort
    cols.sort_and_write(&lines, sep, &mut wtr)?;

    if export {
        println!("Saved to file: {}", out.display())
    }

    Ok(())
}
