use crate::utils::cli_result::CliResult;
use crate::utils::column::Columns;
use crate::utils::constants::TERMINATOR;
use crate::utils::excel_reader::{ExcelChunkTask, ExcelReader};
use crate::utils::file::file_or_stdout_wtr;
use crate::utils::filename::new_path;
use crate::utils::filter::Filter;
use crate::utils::progress::Progress;
use crossbeam_channel::bounded;
use rayon::prelude::*;
use std::io::{BufWriter, Write};
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
    let f = file_or_stdout_wtr(export, &out_path)?;
    let mut wtr = BufWriter::new(f);
    let mut range = ExcelReader::new(path, sheet)?;

    // header
    if !no_header {
        let row = match range.next() {
            Some(v) => match cols.all {
                true => v.iter().map(|i| i.to_string()).collect::<Vec<_>>(),
                false => cols.iter().map(|&i| v[i].to_string()).collect::<Vec<_>>(),
            },
            None => {
                return Ok(());
            }
        };
        print_record(&mut wtr, &row)?;
    }

    // parallel queue
    let (tx, rx) = bounded(1);

    // read
    thread::spawn(move || range.send_to_channel_in_line_chunks(tx));

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
    wtr: &mut BufWriter<Box<dyn Write>>,
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
    filtered.into_iter().for_each(|row| {
        if cols.all {
            print_record(wtr, &row).unwrap()
        } else {
            let record = cols.iter().map(|&i| row[i].to_owned()).collect::<Vec<_>>();
            print_record(wtr, &record).unwrap()
        }
    });

    if export {
        prog.add_chunks(1);
        prog.add_lines(task.n);
        prog.print_lines();
    }
}

fn print_record(wtr: &mut BufWriter<Box<dyn Write>>, record: &[String]) -> std::io::Result<()> {
    let mut it = record.iter().peekable();

    while let Some(field) = it.next() {
        wtr.write_all(field.as_bytes())?;

        if it.peek().is_none() {
            wtr.write_all(TERMINATOR)?;
        } else {
            wtr.write_all(b",")?;
        }
    }

    Ok(())
}
