use crate::args::Select;
use rsv_lib::utils::filter::Filter;
use rsv_lib::utils::writer::Writer;
use crossbeam_channel::bounded;
use rayon::prelude::*;
use rsv_lib::utils::cli_result::CliResult;
use rsv_lib::utils::column::Columns;
use rsv_lib::utils::filename::new_path;
use rsv_lib::utils::progress::Progress;
use rsv_lib::utils::reader::{ChunkReader, Task};
use std::thread;

impl Select {
    pub fn csv_run(&self) -> CliResult {
        let path = &self.path();

        // filters and cols
        let filter = Filter::new(&self.filter)
            .total_col_of(path, self.sep, self.quote)
            .parse();
        let cols = Columns::new(&self.cols)
            .total_col_of(path, self.sep, self.quote)
            .parse();

        // wtr and rdr
        let out = new_path(path, "-selected");
        let mut wtr = Writer::file_or_stdout(self.export, &out)?;
        let mut rdr = ChunkReader::new(path)?;

        // header
        if !self.no_header {
            let Some(r) = rdr.next() else { return Ok(()) };
            let r = r?;
            if cols.select_all {
                wtr.write_str_unchecked(&r)
            } else {
                let mut r = self.split_row_to_vec(&r);
                r = cols.iter().map(|&i| r[i]).collect();
                wtr.write_fields_unchecked(&r);
            }
        }

        // parallel queue
        let (tx, rx) = bounded(1);

        // read
        thread::spawn(move || rdr.send_to_channel_by_chunks(tx, 10_000));

        // process
        let mut prog = Progress::new();
        for task in rx {
            handle_task(self, task, &filter, &cols, &mut wtr, &mut prog)
        }

        if self.export {
            println!("\nSaved to file: {}", out.display())
        }

        Ok(())
    }
}

#[allow(clippy::too_many_arguments)]
fn handle_task(
    args: &Select,
    task: Task,
    filter: &Filter,
    cols: &Columns,
    wtr: &mut Writer,
    prog: &mut Progress,
) {
    // filter
    let filtered = task
        .lines
        .par_iter()
        .filter_map(|row| filter.record_valid_map(row, args.sep, args.quote))
        .collect::<Vec<(_, _)>>();

    // write
    for (r, f) in filtered {
        // write the line directly
        if cols.select_all {
            wtr.write_str_unchecked(r.unwrap());
            continue;
        }

        // write by fields
        let f = f.unwrap_or_else(|| args.split_row_to_vec(r.unwrap()));
        let row = cols.iter().map(|&i| f[i]).collect::<Vec<_>>();
        wtr.write_fields_unchecked(&row);
    }

    if args.export {
        prog.add_chunks(1);
        prog.add_bytes(task.bytes);
        prog.print();
    }
}
