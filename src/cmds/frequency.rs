use crossbeam_channel::bounded;
use dashmap::DashMap;
use rayon::prelude::*;
use std::thread;
use tabled::builder::Builder;
use tabled::Style;

use crate::utils::chunk_reader::ChunkReader;
use crate::utils::column::Columns;
use crate::utils::file::{self, estimate_line_count_by_mb};
use crate::utils::filename;
use crate::utils::filename::full_path;
use crate::utils::progress::Progress;

pub fn run(
    filename: &str,
    no_header: bool,
    sep: &str,
    cols: &str,
    ascending: bool,
    n: i32,
    export: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    // current file
    let path = full_path(filename)?;

    // cols
    let col = Columns::new(cols);

    // open file and header
    let mut rdr = ChunkReader::new(&path)?;
    let names: Vec<String> = if no_header {
        col.artificial_cols_with_appended_n()
    } else {
        let first_row = rdr.next()?;
        let r = first_row.split(sep).collect::<Vec<_>>();
        if col.max() >= r.len() {
            println!("read a bad line # {:?}!", r);
            col.artificial_cols_with_appended_n()
        } else {
            col.select_owned_vector_and_append_n(&r)
        }
    };

    // read file
    let (tx, rx) = bounded(1);
    let line_buffer_n: usize = estimate_line_count_by_mb(filename, Some(10));
    thread::spawn(move || rdr.send_to_channel_in_line_chunks(tx, line_buffer_n));

    // process
    let freq = DashMap::new();
    let mut prog = Progress::new();
    for task in rx {
        task.lines.par_iter().for_each(|r| {
            let r = r.split(sep).collect::<Vec<_>>();
            if col.max() >= r.len() {
                println!("ignore a bad line # {:?}!", r);
            } else {
                let r = col.select_owned_string(&r);
                *freq.entry(r).or_insert(0) += 1;
            }
        });

        prog.add_chunks(1);
        prog.add_bytes(task.bytes);
        prog.print();
    }

    let mut freq: Vec<(String, i32)> = freq.into_iter().collect::<Vec<(_, _)>>();
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
    println!();
    if export {
        let new_path = filename::new_path(&path, "-frequency");
        file::write_to_csv(&new_path, &names, freq);
        println!("Saved to file: {}", new_path.display());
    } else {
        print_table(&names, freq)
    }

    Ok(())
}

fn print_table(names: &Vec<String>, freq: Vec<(String, i32)>) {
    let mut builder = Builder::default();

    // header
    if !names.is_empty() {
        builder.set_columns(names);
    }

    // content
    for (key, n) in freq {
        let r = key
            .split(',')
            .map(|i| i.to_owned())
            .chain(std::iter::once(n.to_string()))
            .collect::<Vec<_>>();
        builder.add_record(r);
    }

    // build
    let mut table = builder.build();

    // style
    table.with(Style::blank());

    println!("{table}");
}
