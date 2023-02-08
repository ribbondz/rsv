use crate::utils::cli_result::CliResult;
use crate::utils::constants::COMMA;
use crate::utils::reader::ExcelReader;
use crate::utils::filename::new_path;
use crate::utils::progress::Progress;
use crate::utils::regex::Re;
use crate::utils::writer::Writer;
use crossbeam_channel::bounded;
use rayon::prelude::*;
use std::path::Path;
use std::thread;

pub fn run(path: &Path, sheet: usize, pattern: &str, no_header: bool, export: bool) -> CliResult {
    // wtr and rdr
    let out = new_path(path, "-searched").with_extension("csv");
    let mut wtr = Writer::file_or_stdout(export, &out)?;
    let mut range = ExcelReader::new(path, sheet)?;

    // header
    if !no_header {
        let Some(r) = range.next() else {
            return Ok(())
        };
        wtr.write_excel_line_unchecked(r, COMMA)
    };

    // read file
    let (tx, rx) = bounded(2);
    thread::spawn(move || range.send_to_channel_in_line_chunks(tx, None));

    // progress for export option
    let mut prog = Progress::new();

    // regex search
    let re = Re::new(pattern)?;
    let mut matched = 0;
    for task in rx {
        let lines = task
            .lines
            .par_iter()
            .filter_map(|i| re.verify_excel_row_map(i))
            .collect::<Vec<_>>();

        matched += lines.len();

        // pipeline could be closed,
        // e.g., when rsv head take enough items
        wtr.write_lines_by_field_unchecked(&lines, None);

        if export {
            prog.add_lines(task.n);
            prog.add_chunks(1);
            prog.print_lines();
        }
    }

    if export {
        println!("\nMatched rows: {matched}");
        println!("Saved to file: {}", out.display());
    }

    Ok(())
}
