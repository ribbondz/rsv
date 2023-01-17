use crate::utils::chunk_reader::{ChunkReader, Task};
use crate::utils::cli_result::CliResult;
use crate::utils::constants::TERMINATOR;
use crate::utils::file::{estimate_line_count_by_mb, file_or_stdout_wtr};
use crate::utils::filename::new_path;
use crate::utils::progress::Progress;
use crossbeam_channel::bounded;
use rayon::prelude::*;
use regex::RegexBuilder;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::{process, thread};

pub fn run(path: &Path, pattern: &str, no_header: bool, export: bool) -> CliResult {
    // wtr and rdr
    let out_path = new_path(path, "-searched");
    let f = file_or_stdout_wtr(export, &out_path)?;
    let mut wtr = BufWriter::new(f);
    let mut rdr = ChunkReader::new(path)?;

    // header
    if !no_header {
        match rdr.next() {
            Ok(r) => write(&mut wtr, &r),
            Err(_) => return Ok(()),
        }
    };

    // read file
    let (tx, rx) = bounded(2);
    let line_buffer_n: usize = estimate_line_count_by_mb(path, Some(10));
    thread::spawn(move || rdr.send_to_channel_by_chunks(tx, line_buffer_n));

    // progress for export option
    let mut prog = Progress::new();

    // regex search
    let re = RegexBuilder::new(pattern).case_insensitive(true).build()?;
    for Task {
        lines,
        bytes,
        chunk: _,
    } in rx
    {
        let lines = lines
            .into_par_iter()
            .filter(|i| re.is_match(i))
            .collect::<Vec<_>>();

        // pipeline could be closed, e.g., when rsv head take enough items
        // ignore the error
        for l in &lines {
            write(&mut wtr, l)
        }

        if export {
            prog.add_lines(lines.len());
            prog.add_chunks(1);
            prog.add_bytes(bytes);
            prog.print();
        }
    }

    if export {
        println!("\nMatched rows: {}", prog.lines);
        println!("Saved to file: {}", out_path.display());
    }

    Ok(())
}

fn write(wtr: &mut BufWriter<Box<dyn Write>>, data: &str) {
    if wtr.write_all(data.as_bytes()).is_err() {
        process::exit(0)
    };
    if wtr.write_all(TERMINATOR).is_err() {
        process::exit(0)
    };
}
