use crate::utils::chunk_reader::{ChunkReader, Task};
use crate::utils::cli_result::CliResult;
use crate::utils::file::estimate_line_count_by_mb;
use crate::utils::filename::{dir_file, str_to_filename};
use crate::utils::progress::Progress;
use crate::utils::util::{datetime_str, werr};
use crate::utils::writer::Writer;
use crossbeam_channel::bounded;
use dashmap::DashMap;
use rayon::prelude::*;
use std::error::Error;
use std::fs::create_dir;
use std::path::Path;
use std::{process, thread};

pub fn run(path: &Path, no_header: bool, sep: &str, col: usize, size: &Option<usize>) -> CliResult {
    let is_sequential_split = size.is_some();

    // new directory
    let dir = path.with_file_name(format!(
        "{}-split-{}",
        path.file_stem().unwrap().to_string_lossy(),
        datetime_str()
    ));
    create_dir(&dir)?;

    // open file and header
    let mut rdr = ChunkReader::new(path)?;
    let first_row = if no_header {
        "".to_owned()
    } else {
        let r = match rdr.next() {
            Some(v) => v?,
            None => return Ok(()),
        };
        if col >= r.split(sep).count() {
            werr!("column index out of range!");
            process::exit(1)
        }
        r
    };

    // work pip
    let (tx, rx) = bounded(1);

    // read
    let line_buffer_n = match is_sequential_split {
        true => size.unwrap(),
        false => estimate_line_count_by_mb(path, Some(50)),
    };
    thread::spawn(move || rdr.send_to_channel_by_chunks(tx, line_buffer_n));

    // process batch work
    let mut prog = Progress::new();
    match is_sequential_split {
        true => {
            let stem = path.file_stem().unwrap().to_string_lossy();
            let extension = path
                .extension()
                .and_then(|i| i.to_str())
                .unwrap_or_default();

            for task in rx {
                let mut out = dir.to_owned();
                out.push(format!("{}-split{}.{}", stem, task.chunk, extension));
                sequential_task_handle(task, &mut prog, &out, &first_row)?;
            }
        }
        false => {
            let header_inserted: DashMap<String, bool> = DashMap::new();
            for task in rx {
                task_handle(
                    task,
                    &mut prog,
                    sep,
                    no_header,
                    col,
                    &dir,
                    &first_row,
                    &header_inserted,
                )?
            }
        }
    }

    println!("\nSaved to directory: {}", dir.display());

    Ok(())
}

fn sequential_task_handle(
    task: Task,
    prog: &mut Progress,
    out: &Path,
    first_row: &str,
) -> Result<(), Box<dyn Error>> {
    // progress
    prog.add_chunks(1);
    prog.add_bytes(task.bytes);

    // write
    let mut wtr = Writer::append_to(out)?;
    wtr.write_header(first_row)?;
    wtr.write_lines(&task.lines)?;

    prog.print();

    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn task_handle(
    task: Task,
    prog: &mut Progress,
    sep: &str,
    no_header: bool,
    col: usize,
    dir: &Path,
    first_row: &str,
    header_inserted: &DashMap<String, bool>,
) -> Result<(), Box<dyn Error>> {
    // progress
    prog.add_chunks(1);
    prog.add_bytes(task.bytes);

    // parallel process
    let batch_work = DashMap::new();
    task.lines.par_iter().for_each(|r| {
        let seg = r.split(sep).collect::<Vec<_>>();
        if col >= r.len() {
            println!("[info] ignore a bad line, content is: {r:?}!");
        } else {
            batch_work.entry(seg[col]).or_insert_with(Vec::new).push(r);
        }
    });

    // parallel save to disk
    batch_work
        .into_iter()
        .collect::<Vec<(_, _)>>()
        .par_iter()
        .for_each(|(field, rows)| {
            save_to_disk(dir, field, rows, no_header, header_inserted, first_row).unwrap();
        });

    prog.print();

    Ok(())
}

fn save_to_disk(
    dir: &Path,
    field: &str,
    rows: &[&String],
    no_header: bool,
    header_inserted: &DashMap<String, bool>,
    first_row: &str,
) -> Result<(), Box<dyn Error>> {
    // file path
    let filename = str_to_filename(field) + ".csv";
    let out = dir_file(dir, &filename);

    // write
    let mut wtr = Writer::append_to(&out)?;
    if !no_header && !header_inserted.contains_key(&filename) {
        header_inserted.insert(filename, true);
        wtr.write_line(first_row)?
    }
    wtr.write_lines(rows)?;

    Ok(())
}
