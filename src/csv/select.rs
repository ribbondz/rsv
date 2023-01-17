use crate::utils::chunk_reader::{ChunkReader, Task};
use crate::utils::cli_result::CliResult;
use crate::utils::column::Columns;
use crate::utils::constants::TERMINATOR;
use crate::utils::file::{estimate_line_count_by_mb, file_or_stdout_wtr};
use crate::utils::filename::new_path;
use crate::utils::filter::Filter;
use crate::utils::progress::Progress;
use crossbeam_channel::bounded;
use rayon::prelude::*;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::{process, thread};

pub fn run(
    path: &Path,
    no_header: bool,
    sep: &str,
    cols: &str,
    filter: &str,
    export: bool,
) -> CliResult {
    // current file
    let out_path = new_path(path, "-selected");

    // filters and cols
    let filter = Filter::new(filter);
    let cols = Columns::new(cols);

    // open file
    let f = file_or_stdout_wtr(export, &out_path)?;
    let mut wtr = BufWriter::new(f);
    let mut rdr = ChunkReader::new(path)?;

    // const
    let sep_bytes = sep.as_bytes();

    // header
    if !no_header {
        match rdr.next()? {
            Some(r) => {
                let r = r.split(sep).collect::<Vec<_>>();
                let r = match cols.all {
                    true => r,
                    false => cols.iter().map(|&i| r[i]).collect(),
                };
                print_record(&mut wtr, &r, sep_bytes)
            }
            None => return Ok(()),
        }
    }

    // parallel queue
    let (tx, rx) = bounded(1);

    // read
    let line_buffer_n: usize = estimate_line_count_by_mb(path, Some(10));
    thread::spawn(move || rdr.send_to_channel_by_chunks(tx, line_buffer_n));

    // process
    let mut prog = Progress::new();
    for task in rx {
        handle_task(
            task, &filter, sep, &cols, &mut wtr, sep_bytes, export, &mut prog,
        )
    }

    if export {
        println!("\nSaved to file: {}", out_path.display())
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn handle_task(
    task: Task,
    filter: &Filter,
    sep: &str,
    cols: &Columns,
    wtr: &mut BufWriter<Box<dyn Write>>,
    sep_bytes: &[u8],
    export: bool,
    prog: &mut Progress,
) {
    // filter
    let filtered = task
        .lines
        .par_iter()
        .filter_map(|row| filter.record_valid_map(row, sep))
        .collect::<Vec<_>>();

    // write
    for row in filtered {
        match cols.all {
            true => print_record(wtr, &row, sep_bytes),
            false => {
                let record = cols.iter().map(|&i| row[i]).collect::<Vec<_>>();
                print_record(wtr, &record, sep_bytes)
            }
        }
    }

    if export {
        prog.add_chunks(1);
        prog.add_bytes(task.bytes);
        prog.print();
    }
}

/// terminate program when pipeline closed
fn print_record(wtr: &mut BufWriter<Box<dyn Write>>, record: &[&str], sep_bytes: &[u8]) {
    let mut it = record.iter().peekable();

    while let Some(&field) = it.next() {
        if wtr.write_all(field.as_bytes()).is_err() {
            process::exit(0);
        };

        let t = if it.peek().is_none() {
            TERMINATOR
        } else {
            sep_bytes
        };
        if wtr.write_all(t).is_err() {
            process::exit(0);
        };
    }
}
