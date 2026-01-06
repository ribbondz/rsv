use crate::args::Stats;
use crossbeam_channel::{bounded, unbounded};
use rsv_lib::utils::chunk::{ChunkParser, ChunkResult};
use rsv_lib::utils::cli_result::CliResult;
use rsv_lib::utils::column::Columns;
use rsv_lib::utils::column_stats::ColumnStats;
use rsv_lib::utils::column_type::ColumnTypes;
use rsv_lib::utils::file::column_n;
use rsv_lib::utils::filename::new_path;
use rsv_lib::utils::progress::Progress;
use rsv_lib::utils::reader::ChunkReader;
use std::fs::File;
use std::io::{BufWriter, Write};

impl Stats {
    pub fn csv_run(&self) -> CliResult {
        let path = &self.path();

        // Column
        let cols = Columns::new(&self.cols)
            .total_col_of(path, self.sep, self.quote)
            .parse();
        let Some(col_type) = ColumnTypes::guess_from_csv(
            path,
            self.sep,
            self.quote,
            self.no_header,
            &cols,
            &self.text_columns,
            &[],
        )?
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
        let (chunk_sender, chunk_receiver) = bounded(rayon::current_num_threads() * 2);
        let (result_sender, result_receiver) = unbounded();

        // progress bar
        let mut prog = Progress::new();

        // chunk parser
        let parser = ChunkParser::new(self.sep, self.quote);

        // parallel process
        rayon::scope(|s| {
            // read chunks
            s.spawn(|_| rdr.send_to_channel_by_chunks(chunk_sender, 20_000));

            // add chunk to threadpool for process
            s.spawn(|s| {
                for task in chunk_receiver {
                    let tx = result_sender.clone();
                    let st = empty_stat.clone();

                    // process chunk in parallel
                    s.spawn(|_| parser.parse(task, tx, st));
                }

                drop(result_sender);
            });

            // receive result
            for ChunkResult { bytes, stat: o } in result_receiver {
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
