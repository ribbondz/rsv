use crate::utils::reader::ChunkReader;
use crate::utils::cli_result::CliResult;
use crate::utils::column::Columns;
use crate::utils::file::{self, estimate_line_count_by_mb};
use crate::utils::filename;
use crate::utils::progress::Progress;
use crate::utils::util::print_frequency_table;
use crossbeam_channel::bounded;
use dashmap::DashMap;
use rayon::prelude::*;
use std::path::Path;
use std::thread;

pub fn run(
    path: &Path,
    no_header: bool,
    sep: &str,
    cols: &str,
    ascending: bool,
    n: i32,
    export: bool,
) -> CliResult {
    // cols
    let col = Columns::new(cols);

    // open file and header
    let mut rdr = ChunkReader::new(path)?;
    let names: Vec<String> = if no_header {
        col.artificial_cols_with_appended_n()
    } else {
        let Some(r) = rdr.next() else {
            return Ok(())
        };
        let r = r?;
        let r = r.split(sep).collect::<Vec<_>>();
        if col.max() >= r.len() {
            println!("[info] ignore a bad line # {r:?}!");
            col.artificial_cols_with_appended_n()
        } else {
            col.select_owned_vector_and_append_n(&r)
        }
    };

    // read file
    let (tx, rx) = bounded(1);
    let line_buffer_n: usize = estimate_line_count_by_mb(path, Some(10));
    thread::spawn(move || rdr.send_to_channel_by_chunks(tx, line_buffer_n));

    // process
    let freq = DashMap::new();
    let mut prog = Progress::new();
    for task in rx {
        task.lines.par_iter().for_each(|r| {
            let r = r.split(sep).collect::<Vec<_>>();
            if col.max() >= r.len() {
                println!("[info] ignore a bad line # {r:?}!");
            } else {
                let r = col.select_owned_string(&r);
                *freq.entry(r).or_insert(0) += 1;
            }
        });

        if export {
            prog.add_chunks(1);
            prog.add_bytes(task.bytes);
            prog.print();
        }
    }

    let mut freq = freq.into_iter().collect::<Vec<(_, _)>>();
    if ascending {
        freq.sort_by(|a, b| a.1.cmp(&b.1));
    } else {
        freq.sort_by(|a, b| b.1.cmp(&a.1));
    }

    // apply head n
    if n > 0 {
        freq = freq.into_iter().take(n as usize).collect()
    }

    // export or print
    if export {
        let new_path = filename::new_path(path, "-frequency");
        file::write_frequency_to_csv(&new_path, &names, freq);
        println!("\nSaved to file: {}", new_path.display());
    } else {
        print_frequency_table(&names, freq)
    }

    Ok(())
}
