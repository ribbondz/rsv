use crate::utils::cli_result::CliResult;
use crate::utils::column::Columns;
use crate::utils::column_stats::ColumnStats;
use crate::utils::column_type::ColumnTypes;
use crate::utils::excel_reader::{ExcelChunkTask, ExcelReader};
use crate::utils::filename::new_path;
use crate::utils::progress::Progress;
use crossbeam_channel::{bounded, unbounded, Sender};

use rayon::ThreadPoolBuilder;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

pub fn run(path: &Path, sheet: usize, no_header: bool, cols: &str, export: bool) -> CliResult {
    // file
    let mut range = ExcelReader::new(path, sheet)?;
    let column_n = range.column_n();

    // Column filter
    let cols = Columns::new(cols);
    let col_type = ColumnTypes::guess_from_excel(path, sheet, no_header, &cols)?;

    // header
    let name = if no_header {
        cols.artificial_n_cols(column_n)
    } else {
        match range.next() {
            Some(v) => match cols.all {
                true => v.iter().map(|i| i.to_string()).collect::<Vec<_>>(),
                false => cols.iter().map(|&i| v[i].to_string()).collect::<Vec<_>>(),
            },
            None => return Ok(()),
        }
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
    pool.spawn(move || range.send_to_channel_in_line_chunks(tx_chunk));

    // parallel process
    pool.scope(|s| {
        // add chunk to threadpool for process
        s.spawn(|_| {
            for task in rx_chunk {
                tx_chunk_n_control.send(0).unwrap();

                let tx = tx_result.clone();
                let st = empty_stat.clone();
                // println!("dispatch........");
                pool.spawn(move || parse_chunk(task, tx, st));
            }

            drop(tx_result);
            drop(tx_chunk_n_control);
        });

        // receive result
        for ExcelChunkResult { n, stat: o } in rx_result {
            rx_chunk_n_control.recv().unwrap();
            // println!("result-----------");
            // this is bottleneck, merge two hashset is very slow.
            stat.merge(o);

            prog.add_lines(n);
            prog.add_chunks(1);
            prog.print_lines();
        }

        prog.clear();
    });

    // refine result
    stat.cal_unique_and_mean();

    // print
    if export {
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

struct ExcelChunkResult {
    n: usize,
    stat: ColumnStats,
}

fn parse_chunk(task: ExcelChunkTask, tx: Sender<ExcelChunkResult>, mut stat: ColumnStats) {
    let ExcelChunkTask { lines, n } = task;
    lines.into_iter().for_each(|i| stat.parse_excel_row(i));

    tx.send(ExcelChunkResult { n, stat }).unwrap()
}
