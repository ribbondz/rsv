use crate::utils::chunk_reader::{ChunkReader, Task};
use crate::utils::constants::TERMINATOR;
use crate::utils::file::estimate_line_count_by_mb;
use crate::utils::filename::{full_path, str_clean_as_filename};
use crate::utils::progress::Progress;
use crate::utils::util::datetime_str;

use ahash::AHashMap;
use crossbeam_channel::bounded;
use dashmap::DashMap;
use rayon::prelude::*;
use std::error::Error;
use std::fs::{create_dir, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::Path;
use std::thread;

pub fn run(filename: &str, no_header: bool, sep: &str, col: usize) -> Result<(), Box<dyn Error>> {
    // current file
    let path = full_path(filename)?;

    // new directory
    let stem = Path::new(filename).file_stem().unwrap();
    let dir = path.with_file_name(format!(
        "{}-partition-{}",
        stem.to_string_lossy(),
        datetime_str()
    ));
    create_dir(&dir)?;

    // open file and header
    let mut rdr = ChunkReader::new(&path)?;
    let first_row = if no_header {
        "".to_owned()
    } else {
        let first_row = rdr.next()?;
        if col >= first_row.split(sep).count() {
            panic!("Column index out of its range!");
        } else {
            first_row
        }
    };

    let (tx, rx) = bounded(1);
    // read
    let line_buffer_n = estimate_line_count_by_mb(filename, Some(512));
    thread::spawn(move || rdr.send_to_channel_in_line_chunks(tx, line_buffer_n));

    // process batch work
    let mut header_inserted: AHashMap<String, bool> = AHashMap::new();
    let mut prog = Progress::new();
    for task in rx {
        task_handle(
            task,
            &mut prog,
            sep,
            no_header,
            col,
            &dir,
            &first_row,
            &mut header_inserted,
        )?
    }

    println!("\nSaved to directory: {}", dir.display());

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
    header_inserted: &mut AHashMap<String, bool>,
) -> Result<(), Box<dyn Error>> {
    // progress
    prog.add_chunks(1);
    prog.add_bytes(task.bytes);

    // process
    let batch_work = DashMap::new();
    task.lines.par_iter().for_each(|r| {
        let seg = r.split(sep).collect::<Vec<_>>();
        if col >= r.len() {
            println!("ignore a bad line, content is: {:?}!", r);
        } else {
            batch_work.entry(seg[col]).or_insert_with(Vec::new).push(r);
        }
    });

    // save to disk
    for (field, rows) in batch_work {
        save_to_disk(dir, field, &rows, no_header, header_inserted, first_row)?
    }

    prog.print();

    Ok(())
}

fn save_to_disk(
    dir: &Path,
    field: &str,
    rows: &[&String],
    no_header: bool,
    header_inserted: &mut AHashMap<String, bool>,
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
        wtr.write_all(r.as_bytes()).unwrap();
        wtr.write_all(TERMINATOR).unwrap();
    });

    Ok(())
}
