use crate::utils::cli_result::CliResult;
use crate::utils::filename::{dir_file, new_file, str_to_filename};
use crate::utils::util::datetime_str;
use crate::utils::writer::Writer;
use dashmap::DashMap;
use rayon::prelude::*;
use std::error::Error;
use std::fs::create_dir;
use std::io::{stdin, BufRead};
use std::path::Path;

struct Args {
    no_header: bool,
    col: usize,
    is_sequential_split: bool,
    chunk: usize,
}

pub fn run(no_header: bool, sep: &str, col: usize, size: &Option<usize>) -> CliResult {
    let is_sequential_split = size.is_some();
    let mut args = Args {
        no_header,
        col,
        is_sequential_split,
        chunk: 1,
    };

    // new directory
    let dir = format!("split-{}", datetime_str());
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
    let buffer = match is_sequential_split {
        true => size.unwrap(),
        false => 10000,
    };
    let mut lines = Vec::with_capacity(buffer);
    for r in rdr {
        let r = r?;
        n += 1;
        lines.push(r);
        if n >= buffer {
            task_handle(&args, lines, sep, &dir, &first_row, &header_inserted)?;
            lines = Vec::with_capacity(buffer);
            n = 0;
            args.chunk += 1;
        }
    }

    if !lines.is_empty() {
        task_handle(&args, lines, sep, &dir, &first_row, &header_inserted)?;
    }

    println!("Saved to directory: {}", dir.display());

    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn task_handle(
    args: &Args,
    lines: Vec<String>,
    sep: &str,
    dir: &Path,
    first_row: &str,
    header_inserted: &DashMap<String, bool>,
) -> Result<(), Box<dyn Error>> {
    match args.is_sequential_split {
        true => sequential_task_handle(args, lines, dir, first_row)?,
        false => col_split_task_handle(args, lines, sep, dir, first_row, header_inserted)?,
    }

    Ok(())
}

fn sequential_task_handle(
    args: &Args,
    lines: Vec<String>,
    dir: &Path,
    first_row: &str,
) -> Result<(), Box<dyn Error>> {
    let mut out = dir.to_owned();
    out.push(format!("split{}.csv", args.chunk));

    // write
    let mut wtr = Writer::append_to(&out)?;
    wtr.write_header(first_row)?;
    wtr.write_lines(&lines)?;

    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn col_split_task_handle(
    args: &Args,
    lines: Vec<String>,
    sep: &str,
    dir: &Path,
    first_row: &str,
    header_inserted: &DashMap<String, bool>,
) -> Result<(), Box<dyn Error>> {
    // parallel process
    let batch_work = DashMap::new();

    lines.par_iter().for_each(|r| {
        let seg = r.split(sep).collect::<Vec<_>>();
        match args.col >= r.len() {
            true => println!("[info] ignore a bad line, content is: {r:?}!"),
            false => batch_work
                .entry(seg[args.col])
                .or_insert_with(Vec::new)
                .push(r),
        }
    });

    // parallel save to disk
    batch_work
        .into_iter()
        .collect::<Vec<(_, _)>>()
        .par_iter()
        .for_each(|(field, rows)| {
            save_to_disk(args, dir, field, rows, header_inserted, first_row).unwrap();
        });

    Ok(())
}

fn save_to_disk(
    args: &Args,
    dir: &Path,
    field: &str,
    rows: &[&String],
    header_inserted: &DashMap<String, bool>,
    first_row: &str,
) -> Result<(), Box<dyn Error>> {
    // file path
    let filename = str_to_filename(field) + ".csv";
    let out = dir_file(dir, &filename);

    // write
    let mut wtr = Writer::append_to(&out)?;
    if !args.no_header && !header_inserted.contains_key(&filename) {
        header_inserted.insert(filename, true);
        wtr.write_line(first_row)?;
    }
    wtr.write_lines(rows)?;

    Ok(())
}
