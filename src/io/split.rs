use crate::utils::cli_result::CliResult;
use crate::utils::constants::TERMINATOR;
use crate::utils::filename::{new_file, str_clean_as_filename};
use crate::utils::util::datetime_str;
use dashmap::DashMap;
use rayon::prelude::*;
use std::error::Error;
use std::fs::{create_dir, OpenOptions};
use std::io::{stdin, BufRead, BufWriter, Write};
use std::path::Path;

pub fn run(no_header: bool, sep: &str, col: usize) -> CliResult {
    // new directory
    let dir = "split-".to_owned() + &datetime_str();
    let dir = new_file(&dir);
    create_dir(&dir)?;

    // open file and header
    let mut rdr = stdin().lock().lines();
    let first_row = if no_header {
        "".to_owned()
    } else {
        match rdr.next() {
            Some(v) => v?,
            None => return Ok(()),
        }
    };

    let header_inserted: DashMap<String, bool> = DashMap::new();
    let mut n = 0;
    let buffer = 10000;
    let mut lines = Vec::with_capacity(buffer);

    for r in rdr {
        let r = r?;
        n += 1;
        lines.push(r);
        if n >= buffer {
            task_handle(
                lines,
                sep,
                no_header,
                col,
                &dir,
                &first_row,
                &header_inserted,
            )?;
            lines = Vec::with_capacity(buffer);
            n = 0;
        }
    }

    if !lines.is_empty() {
        task_handle(
            lines,
            sep,
            no_header,
            col,
            &dir,
            &first_row,
            &header_inserted,
        )?;
    }

    println!("Saved to directory: {}", dir.display());

    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn task_handle(
    lines: Vec<String>,
    sep: &str,
    no_header: bool,
    col: usize,
    dir: &Path,
    first_row: &str,
    header_inserted: &DashMap<String, bool>,
) -> Result<(), Box<dyn Error>> {
    // parallel process
    let batch_work = DashMap::new();

    lines.par_iter().for_each(|r| {
        let seg = r.split(sep).collect::<Vec<_>>();
        match col >= r.len() {
            true => println!("[info] ignore a bad line, content is: {:?}!", r),
            false => batch_work.entry(seg[col]).or_insert_with(Vec::new).push(r),
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
