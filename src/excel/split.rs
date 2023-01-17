use crate::utils::cli_result::CliResult;
use crate::utils::constants::TERMINATOR;
use crate::utils::excel_reader::{ExcelChunkTask, ExcelReader};
use crate::utils::filename::str_clean_as_filename;
use crate::utils::progress::Progress;
use crate::utils::util::{datetime_str, werr};
use calamine::DataType;
use crossbeam_channel::bounded;
use dashmap::DashMap;
use rayon::prelude::*;
use std::error::Error;
use std::fs::{create_dir, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::Path;
use std::{process, thread};

pub fn run(
    path: &Path,
    sheet: usize,
    no_header: bool,
    col: usize,
    size: &Option<usize>,
) -> CliResult {
    let is_sequential_split = size.is_some();

    // new directory
    let dir = path.with_file_name(format!(
        "{}-split-{}",
        path.file_stem().unwrap().to_string_lossy(),
        datetime_str()
    ));
    create_dir(&dir)?;

    // open file and header
    let mut range = ExcelReader::new(path, sheet)?;
    let first_row = if no_header {
        "".to_owned()
    } else {
        let first_row = match range.next() {
            Some(v) => v,
            None => return Ok(()),
        };
        if col >= first_row.len() {
            werr!("Error: column index out of range!");
            process::exit(1);
        };
        first_row
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .join(",")
    };

    let (tx, rx) = bounded(1);
    // read
    let buffer_size = if is_sequential_split {
        size.clone()
    } else {
        None
    };
    thread::spawn(move || range.send_to_channel_in_line_chunks(tx, buffer_size));

    // process batch work
    let mut prog = Progress::new();
    match is_sequential_split {
        true => {
            let stem = path.file_stem().unwrap();
            for task in rx {
                let mut out = dir.to_owned();
                out.push(format!(
                    "{}-split{}.csv",
                    stem.to_string_lossy(),
                    task.chunk
                ));
                sequential_task_handle(task, &mut prog, &out, &first_row)?;
            }
        }
        false => {
            let header_inserted: DashMap<String, bool> = DashMap::new();
            for task in rx {
                task_handle(
                    task,
                    &mut prog,
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

#[allow(clippy::too_many_arguments)]
fn sequential_task_handle(
    task: ExcelChunkTask,
    prog: &mut Progress,
    out: &Path,
    first_row: &str,
) -> Result<(), Box<dyn Error>> {
    // progress
    prog.add_chunks(1);
    prog.add_lines(task.n);

    // open file
    let f = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(out)?;
    let mut wtr = BufWriter::new(f);

    // header
    if !first_row.is_empty() {
        wtr.write_all(first_row.as_bytes())?;
        wtr.write_all(TERMINATOR)?;
    }

    // content
    task.lines.iter().for_each(|r| {
        let r = r
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .join(",");
        wtr.write_all(r.as_bytes()).unwrap();
        wtr.write_all(TERMINATOR).unwrap();
    });

    prog.print();

    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn task_handle(
    task: ExcelChunkTask,
    prog: &mut Progress,
    no_header: bool,
    col: usize,
    dir: &Path,
    first_row: &str,
    header_inserted: &DashMap<String, bool>,
) -> Result<(), Box<dyn Error>> {
    // progress
    prog.add_chunks(1);
    prog.add_lines(task.n);

    // parallel process
    let batch_work = DashMap::new();
    task.lines.par_iter().for_each(|r| {
        if col >= r.len() {
            println!("[info] ignore a bad line, content is: {:?}!", r);
        } else {
            batch_work
                .entry(r[col].to_string())
                .or_insert_with(Vec::new)
                .push(r);
        }
    });

    // parallel save to disk
    batch_work
        .into_iter()
        .collect::<Vec<(_, _)>>()
        .par_iter()
        .for_each(|(a, b)| {
            save_to_disk(dir, a, b, no_header, header_inserted, first_row).unwrap();
        });

    prog.print();

    Ok(())
}

fn save_to_disk(
    dir: &Path,
    field: &str,
    rows: &[&Vec<DataType>],
    no_header: bool,
    header_inserted: &DashMap<String, bool>,
    first_row: &str,
) -> Result<(), Box<dyn Error>> {
    // file path
    let filename = str_clean_as_filename(field, None);
    let mut path = dir.to_path_buf();
    path.push(&filename);

    // open file
    let f = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(&path)?;
    let mut wtr = BufWriter::new(f);

    // header
    if !no_header && !header_inserted.contains_key(&filename) {
        header_inserted.insert(filename, true);
        wtr.write_all(first_row.as_bytes())?;
        wtr.write_all(TERMINATOR)?;
    }

    // content
    rows.iter().for_each(|&r| {
        let r = r
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .join(",");
        wtr.write_all(r.as_bytes()).unwrap();
        wtr.write_all(TERMINATOR).unwrap();
    });

    Ok(())
}
