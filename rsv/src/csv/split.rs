use crate::args::Split;
use crossbeam_channel::bounded;
use dashmap::DashMap;
use rayon::prelude::*;
use rsv_lib::utils::cli_result::CliResult;
use rsv_lib::utils::filename::{dir_file, str_to_filename};
use rsv_lib::utils::progress::Progress;
use rsv_lib::utils::reader::{ChunkReader, Task};
use rsv_lib::utils::util::{datetime_str, werr_exit};
use rsv_lib::utils::writer::Writer;
use std::fs::create_dir;
use std::path::Path;
use std::thread;

impl Split {
    pub fn csv_run(&self) -> CliResult {
        let path = &self.path();
        let is_sequential_split = self.size.is_some();

        // new directory
        let dir = path.with_file_name(format!(
            "{}-split-{}",
            path.file_stem().unwrap().to_string_lossy(),
            datetime_str()
        ));
        create_dir(&dir)?;

        // open file and header
        let mut rdr = ChunkReader::new(path)?;
        let first_row = if self.no_header {
            String::new()
        } else {
            let Some(r) = rdr.next() else {
                return Ok(());
            };
            let r = r?;
            if self.col >= self.row_field_count(&r) {
                werr_exit!("column index out of range!");
            }
            r
        };

        // work pip
        let (tx, rx) = bounded(1);

        // read
        let line_buffer_n = match is_sequential_split {
            true => self.size.unwrap(),
            false => 50_000,
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
                    task_handle(self, task, &mut prog, &dir, &first_row, &header_inserted)?;
                }
            }
        }

        println!("\nSaved to directory: {}", dir.display());

        Ok(())
    }
}

fn sequential_task_handle(
    task: Task,
    prog: &mut Progress,
    out: &Path,
    first_row: &str,
) -> CliResult {
    // progress
    prog.add_chunks(1);
    prog.add_bytes(task.bytes);

    // write
    let mut wtr = Writer::append_to(out)?;
    wtr.write_header(first_row)?;
    wtr.write_strings(&task.lines)?;

    prog.print();

    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn task_handle(
    args: &Split,
    task: Task,
    prog: &mut Progress,
    dir: &Path,
    first_row: &str,
    header_inserted: &DashMap<String, bool>,
) -> CliResult {
    // progress
    prog.add_chunks(1);
    prog.add_bytes(task.bytes);

    // parallel process
    let batch_work = DashMap::new();
    task.lines.par_iter().for_each(|r| {
        let seg = args.split_row_to_vec(r);
        if args.col >= r.len() {
            println!("[info] ignore a bad line, content is: {r:?}!");
            return;
        }
        batch_work
            .entry(seg[args.col])
            .or_insert_with(Vec::new)
            .push(r);
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

    prog.print();

    Ok(())
}
