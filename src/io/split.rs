use crate::args::Split;
use crate::utils::cli_result::CliResult;
use crate::utils::filename::{dir_file, new_file, str_to_filename};
use crate::utils::util::datetime_str;
use crate::utils::writer::Writer;
use dashmap::DashMap;
use rayon::prelude::*;
use std::fs::create_dir;
use std::io::{stdin, BufRead};
use std::path::Path;

impl Split {
    pub fn io_run(&self) -> CliResult {
        let is_sequential_split = self.size.is_some();

        // new directory
        let dir = format!("split-{}", datetime_str());
        let dir = new_file(&dir);
        create_dir(&dir)?;

        // open file and header
        let mut rdr = stdin().lock().lines();
        let first_row = if self.no_header {
            String::new()
        } else {
            let Some(r) = rdr.next() else { return Ok(()) };
            r?
        };

        let header_inserted: DashMap<String, bool> = DashMap::new();
        let mut n = 0;
        let buffer = if is_sequential_split {
            self.size.unwrap()
        } else {
            10000
        };
        let mut chunk = 1;
        let mut lines = Vec::with_capacity(buffer);
        for r in rdr {
            let r = r?;
            n += 1;
            lines.push(r);
            if n >= buffer {
                task_handle(self, chunk, lines, &dir, &first_row, &header_inserted)?;
                lines = Vec::with_capacity(buffer);
                n = 0;
                chunk += 1;
            }
        }

        if !lines.is_empty() {
            task_handle(self, chunk, lines, &dir, &first_row, &header_inserted)?;
        }

        println!("Saved to directory: {}", dir.display());

        Ok(())
    }
}

#[allow(clippy::too_many_arguments)]
fn task_handle(
    args: &Split,
    chunk: usize,
    lines: Vec<String>,
    dir: &Path,
    first_row: &str,
    header_inserted: &DashMap<String, bool>,
) -> CliResult {
    match args.size.is_some() {
        true => sequential_task_handle(chunk, lines, dir, first_row)?,
        false => col_split_task_handle(args, lines, dir, first_row, header_inserted)?,
    };

    Ok(())
}

fn sequential_task_handle(
    chunk: usize,
    lines: Vec<String>,
    dir: &Path,
    first_row: &str,
) -> CliResult {
    let mut out = dir.to_owned();
    out.push(format!("split{}.csv", chunk));

    // write
    let mut wtr = Writer::append_to(&out)?;
    wtr.write_header(first_row)?;
    wtr.write_strings(&lines)?;

    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn col_split_task_handle(
    args: &Split,
    lines: Vec<String>,
    dir: &Path,
    first_row: &str,
    header_inserted: &DashMap<String, bool>,
) -> CliResult {
    // parallel process
    let batch_work = DashMap::new();

    lines.par_iter().for_each(|r| {
        let seg = args.split_row_to_vec(r);
        if args.col >= r.len() {
            println!("[info] ignore a bad line, content is: {r:?}!");
            return;
        }
        batch_work
            .entry(seg[args.col])
            .or_insert_with(Vec::new)
            .push(r)
    });

    // parallel save to disk
    batch_work
        .into_iter()
        .collect::<Vec<(_, _)>>()
        .par_iter()
        .for_each(|(field, rows)| {
            // file path
            let filename = str_to_filename(field) + ".csv";
            let out = dir_file(dir, &filename);

            // write
            let mut wtr = Writer::append_to(&out).unwrap();
            if !args.no_header && !header_inserted.contains_key(&filename) {
                header_inserted.insert(filename, true);
                wtr.write_str(first_row).unwrap();
            }
            wtr.write_strings(rows).unwrap();
        });

    Ok(())
}
