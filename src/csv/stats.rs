use crate::args::Stats;
use crate::utils::cli_result::CliResult;
use crate::utils::column::Columns;
use crate::utils::column_stats::ColumnStats;
use crate::utils::column_type::ColumnTypes;
use crate::utils::file::{column_n, estimate_line_count_by_mb};
use crate::utils::filename::new_path;
use crate::utils::progress::Progress;
use crate::utils::reader::{ChunkReader, Task};
use crossbeam_channel::{bounded, unbounded, Sender};
use rayon::ThreadPoolBuilder;
use std::fs::File;
use std::io::{BufWriter, Write};

impl Stats {
    pub fn csv_run(&self) -> CliResult {
        let path = &self.path();

        // Column
        let cols = Columns::new(&self.cols)
            .total_col_of(path, self.sep, self.quote)
            .parse();
        let Some(col_type) =
            ColumnTypes::guess_from_csv(path, self.sep, self.quote, self.no_header, &cols)?
        else {
            return Ok(());
        };

        // open file
        let mut rdr = ChunkReader::new(path)?;

        // header
        let name = if self.no_header {
            let Some(n) = column_n(path, self.sep, self.quote)? else {
                return Ok(());
            };
            cols.artificial_n_cols(n)
        } else {
            let Some(r) = rdr.next() else { return Ok(()) };
            self.split_row_to_owned_vec(&r?)
        };

        // stats holder
        let mut stat = ColumnStats::new(&col_type, &name);
        let empty_stat = stat.clone();

        // parallel channels
        let (tx_chunk, rx_chunk) = bounded(2);
        let (tx_chunk_n_control, rx_chunk_n_control) = bounded(200);
        let (tx_result, rx_result) = unbounded();

        // progress bar
        let mut prog = Progress::new();

        // threadpool
        let pool = ThreadPoolBuilder::new().build().unwrap();

        // read
        let n = estimate_line_count_by_mb(path, Some(5));
        pool.spawn(move || rdr.send_to_channel_by_chunks(tx_chunk, n));

        // parallel process
        pool.scope(|s| {
            // add chunk to threadpool for process
            s.spawn(|_| {
                for task in rx_chunk {
                    tx_chunk_n_control.send(()).unwrap();

                    let tx = tx_result.clone();
                    let st = empty_stat.clone();
                    let sep_inner = self.sep;
                    let quote_inner = self.quote;
                    // println!("dispatch........");
                    pool.spawn(move || parse_chunk(task, tx, st, sep_inner, quote_inner));
                }

                drop(tx_result);
                drop(tx_chunk_n_control);
            });

            // receive result
            for ChunkResult { bytes, stat: o } in rx_result {
                rx_chunk_n_control.recv().unwrap();
                // println!("result-----------");
                // this is bottleneck, merge two hashset is very slow.
                stat.merge(o);

                prog.add_bytes(bytes);
                prog.add_chunks(1);
                prog.print();
            }

            prog.clear();
        });

        // refine result
        stat.cal_unique_and_mean();

        // print
        if self.export {
            let out = new_path(path, "-stats");
            let mut wtr = BufWriter::new(File::create(&out)?);
            wtr.write_all(stat.to_string().as_bytes())?;
            println!("Saved to file: {}", out.display());
        } else {
            stat.print();
        }

        println!("Total rows: {}", stat.rows);
        prog.print_elapsed_time();

        Ok(())
    }
}

struct ChunkResult {
    bytes: usize,
    stat: ColumnStats,
}

fn parse_chunk(task: Task, tx: Sender<ChunkResult>, mut stat: ColumnStats, sep: char, quote: char) {
    for l in task.lines {
        stat.parse_line(&l, sep, quote)
    }

    tx.send(ChunkResult {
        bytes: task.bytes,
        stat,
    })
    .unwrap()
}
