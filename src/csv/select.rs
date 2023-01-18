use crate::utils::chunk_reader::{ChunkReader, Task};
use crate::utils::cli_result::CliResult;
use crate::utils::column::Columns;
use crate::utils::file::estimate_line_count_by_mb;
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
    sep: &str,
    cols: &str,
    filter: &str,
    export: bool,
) -> CliResult {
    // current file
    let out_path = new_path(path, "-selected");

    // filters and cols
    let filter = Filter::new(filter);
    let cols = Columns::new(cols);

    // open file
    let mut wtr = Writer::file_or_stdout(export, &out_path)?;
    let mut rdr = ChunkReader::new(path)?;

    // const
    let sep_bytes = sep.as_bytes();

    // header
    if !no_header {
        let r = rdr.next();

        if r.is_none() {
            return Ok(());
        }

        let r = r.unwrap()?;

        match cols.all {
            true => wtr.write_line_unchecked(&r),
            false => {
                let mut r = r.split(sep).collect::<Vec<_>>();
                r = cols.iter().map(|&i| r[i]).collect();
                wtr.write_line_by_field_unchecked(&r, Some(sep_bytes));
            }
        };
    }

    // parallel queue
    let (tx, rx) = bounded(1);

    // read
    let line_buffer_n: usize = estimate_line_count_by_mb(path, Some(10));
    thread::spawn(move || rdr.send_to_channel_by_chunks(tx, line_buffer_n));

    // process
    let mut prog = Progress::new();
    for task in rx {
        handle_task(
            task, &filter, sep, &cols, &mut wtr, sep_bytes, export, &mut prog,
        )
    }

    if export {
        println!("\nSaved to file: {}", out_path.display())
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn handle_task(
    task: Task,
    filter: &Filter,
    sep: &str,
    cols: &Columns,
    wtr: &mut Writer,
    sep_bytes: &[u8],
    export: bool,
    prog: &mut Progress,
) {
    // filter
    let filtered = task
        .lines
        .par_iter()
        .filter_map(|row| filter.record_valid_map(row, sep))
        .collect::<Vec<(_, _)>>();

    // write
    for (r, f) in filtered {
        match cols.all {
            true => wtr.write_line_unchecked(r.unwrap()),
            false => {
                let f = f.unwrap();
                let row = cols.iter().map(|&i| f[i]).collect::<Vec<_>>();
                wtr.write_line_by_field_unchecked(&row, Some(sep_bytes));
            }
        }
    }

    if export {
        prog.add_chunks(1);
        prog.add_bytes(task.bytes);
        prog.print();
    }
}
