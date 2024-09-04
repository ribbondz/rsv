use crate::args::Select;
use crate::utils::cli_result::CliResult;
use crate::utils::column::Columns;
use crate::utils::file::estimate_line_count_by_mb;
use crate::utils::filename::new_path;
use crate::utils::filter::Filter;
use crate::utils::progress::Progress;
use crate::utils::reader::{ChunkReader, Task};
use crate::utils::writer::Writer;
use crossbeam_channel::bounded;
use rayon::prelude::*;
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
        let line_buffer_n: usize = estimate_line_count_by_mb(path, Some(10));
        thread::spawn(move || rdr.send_to_channel_by_chunks(tx, line_buffer_n));

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
