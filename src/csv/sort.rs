use crate::utils::cli_result::CliResult;
use crate::utils::filename::new_path;
use crate::utils::sort::SortColumns;
use crate::utils::writer::Writer;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn run(path: &Path, no_header: bool, sep: &str, cols: &str, export: bool) -> CliResult {
    // rdr and wtr
    let mut rdr = BufReader::new(File::open(path)?).lines();
    let out = new_path(path, "-sorted");
    let mut wtr = Writer::file_or_stdout(export, &out)?;

    // cols
    let cols = SortColumns::from(cols)?;

    // header
    if !no_header {
        let Some(r) = rdr.next() else { return Ok(()) };
        wtr.write_line_unchecked(r?);
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
