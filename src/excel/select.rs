use crate::utils::cli_result::CliResult;
use crate::utils::column::Columns;
use crate::utils::excel_reader::{ExcelChunkTask, ExcelReader};
use crate::utils::filename::new_path;
use crate::utils::filter::Filter;
use crate::utils::progress::Progress;
use crate::utils::writer::Writer;
use crossbeam_channel::bounded;
use rayon::prelude::*;
use std::path::Path;
use std::thread;

pub fn run(
    path: &Path,
    no_header: bool,
    sheet: usize,
    cols: &str,
    filter: &str,
    export: bool,
) -> CliResult {
    // out path
    let out_path = new_path(path, "-selected");

    // filters and cols
    let filter = Filter::new(filter);
    let cols = Columns::new(cols);

    // open file
    let mut wtr = Writer::file_or_stdout(export, &out_path)?;
    let mut range = ExcelReader::new(path, sheet)?;

    // header
    if !no_header {
        match range.next() {
            Some(r) => match cols.all {
                true => wtr.write_excel_line_unchecked(r),
                false => {
                    let r = cols.iter().map(|&i| r[i].to_string()).collect::<Vec<_>>();
                    wtr.write_line_by_field_unchecked(&r, None);
                }
            },
            None => return Ok(()),
        };
    }

    // task queue
    let (tx, rx) = bounded(1);

    // read
    thread::spawn(move || range.send_to_channel_in_line_chunks(tx, None));

    // process
    let mut prog = Progress::new();
    for task in rx {
        handle_task(task, &filter, &cols, &mut wtr, export, &mut prog)
    }

    if export {
        println!("\nSaved to file: {}", out_path.display())
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn handle_task(
    task: ExcelChunkTask,
    filter: &Filter,
    cols: &Columns,
    wtr: &mut Writer,
    export: bool,
    prog: &mut Progress,
) {
    // filter
    let filtered = task
        .lines
        .par_iter()
        .filter_map(|row| filter.excel_record_valid_map(row))
        .collect::<Vec<_>>();

    // write
    filtered.iter().for_each(|r| match cols.all {
        true => wtr.write_line_by_field_unchecked(&r, None),
        false => {
            let r = cols.iter().map(|&i| &r[i]).collect::<Vec<_>>();
            wtr.write_line_by_field_unchecked(&r, None);
        }
    });

    if export {
        prog.add_chunks(1);
        prog.add_lines(task.n);
        prog.print_lines();
    }
}
