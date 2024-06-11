use crate::args::Split;
use crate::utils::cli_result::CliResult;
use crate::utils::constants::COMMA;
use crate::utils::excel::datatype_vec_to_string_vec;
use crate::utils::filename::{dir_file, str_to_filename};
use crate::utils::progress::Progress;
use crate::utils::reader::{ExcelChunkTask, ExcelReader};
use crate::utils::util::{datetime_str, werr_exit};
use crate::utils::writer::Writer;
use crossbeam_channel::bounded;
use dashmap::DashMap;
use rayon::prelude::*;
use std::error::Error;
use std::fs::create_dir;
use std::path::Path;
use std::thread;

impl Split {
    pub fn excel_run(&self) -> CliResult {
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
        let mut range = ExcelReader::new(path, self.sheet)?;
        let first_row = if self.no_header {
            String::new()
        } else {
            let Some(r) = range.next() else { return Ok(()) };
            if self.col >= r.len() {
                werr_exit!("Error: column index out of range!");
            };
            datatype_vec_to_string_vec(r).join(",")
        };

        let (tx, rx) = bounded(1);
        // read
        let buffer_size = if is_sequential_split { self.size } else { None };
        thread::spawn(move || range.send_to_channel_by_chunk(tx, buffer_size));

        // process batch work
        let mut prog = Progress::new();
        match is_sequential_split {
            true => {
                let stem = path.file_stem().unwrap().to_string_lossy();
                for task in rx {
                    let mut out = dir.to_owned();
                    out.push(format!("{}-split{}.csv", stem, task.chunk));
                    sequential_task_handle(task, &mut prog, &out, &first_row)?;
                }
            }
            false => {
                let header_inserted: DashMap<String, bool> = DashMap::new();
                for task in rx {
                    task_handle(&self, task, &mut prog, &dir, &first_row, &header_inserted)?
                }
            }
        }

        println!("\nSaved to directory: {}", dir.display());

        Ok(())
    }
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

    // write
    let mut wtr = Writer::append_to(out)?;
    wtr.write_header(first_row)?;
    wtr.write_excel_lines(&task.lines, COMMA)?;

    prog.print();

    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn task_handle(
    options: &Split,
    task: ExcelChunkTask,
    prog: &mut Progress,
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
        if options.col >= r.len() {
            println!("[info] ignore a bad line, content is: {r:?}!");
        } else {
            batch_work
                .entry(r[options.col].to_string())
                .or_insert_with(Vec::new)
                .push(r);
        }
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
            if !options.no_header && !header_inserted.contains_key(&filename) {
                header_inserted.insert(filename, true);
                wtr.write_str(first_row).unwrap();
            }
            wtr.write_excel_lines_by_ref(rows, COMMA).unwrap();
        });

    prog.print();

    Ok(())
}
