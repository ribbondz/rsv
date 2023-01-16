use crate::utils::constants::TERMINATOR;
use crate::utils::excel_reader::{ExcelChunkTask, ExcelReader};
use crate::utils::file::file_or_stdout_wtr;
use crate::utils::filename::new_path;
use crate::utils::progress::Progress;
use calamine::DataType;
use crossbeam_channel::bounded;
use rayon::prelude::*;
use regex::RegexBuilder;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::thread;

pub fn run(
    path: &Path,
    sheet: usize,
    pattern: &str,
    no_header: bool,
    export: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    // wtr and rdr
    let out_path = new_path(path, "-searched").with_extension("csv");
    let f = file_or_stdout_wtr(export, &out_path)?;
    let mut wtr = BufWriter::new(f);
    let mut range = ExcelReader::new(path, sheet)?;

    // header
    if !no_header {
        let first_row = match range.next() {
            Some(v) => v
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<_>>()
                .join(","),
            None => return Ok(()),
        };
        wtr.write_all(first_row.as_bytes())?;
        wtr.write_all(TERMINATOR)?;
    };

    // read file
    let (tx, rx) = bounded(2);
    thread::spawn(move || range.send_to_channel_in_line_chunks(tx));

    // progress for export option
    let mut prog = Progress::new();

    // regex search
    let re = RegexBuilder::new(pattern).case_insensitive(true).build()?;
    let verify_excel_row = |i: Vec<DataType>| {
        let v = i.iter().map(|j| j.to_string()).collect::<Vec<_>>();
        match v.iter().any(|i| re.is_match(i)) {
            true => Some(v.join(",")),
            false => None,
        }
    };
    let mut matched = 0;
    for ExcelChunkTask { lines, n } in rx {
        let lines = lines
            .into_par_iter()
            .filter_map(verify_excel_row)
            .collect::<Vec<_>>();

        matched += lines.len();

        lines.iter().for_each(|i| {
            wtr.write_all(i.as_bytes()).unwrap();
            wtr.write_all(TERMINATOR).unwrap();
        });

        if export {
            prog.add_lines(n);
            prog.add_chunks(1);
            prog.print_lines();
        }
    }

    if export {
        println!("\nMatched rows: {}", matched);
        println!("Saved to file: {}", out_path.display());
    }

    Ok(())
}
