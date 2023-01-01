use crossbeam_channel::bounded;
use dashmap::DashMap;
use rayon::prelude::*;
use std::path::Path;
use std::thread;
use tabled::builder::Builder;
use tabled::Style;

use crate::utils::chunk_reader::ChunkReader;
use crate::utils::column::Columns;
use crate::utils::file::{self, estimate_line_count_by_mb};
use crate::utils::filename;
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
    let mut path = std::env::current_dir()?;
    path.push(Path::new(filename));

    // cols
    let col = Columns::new(cols);

    // open file and header
    let mut rdr = ChunkReader::new(&path)?;
    let names: Vec<String> = if no_header {
        artificial_cols(&col)
    } else {
        let first_row = rdr.next()?;
        let r = first_row.split(sep).collect::<Vec<_>>();
        if col.max() >= r.len() {
            println!("read a bad line # {:?}!", r);
            artificial_cols(&col)
        } else {
            col.select_and_append_n(&r)
        }
    };

    // read file
    let (tx, rx) = bounded(1);
    let line_buffer_n: usize = estimate_line_count_by_mb(filename, None);
    thread::spawn(move || rdr.send_chunks_to_channel(tx, line_buffer_n));

    // process
    let freq = DashMap::new();
    let mut prog = Progress::new();
    for task in rx {
        task.lines.par_iter().for_each(|r| {
            let r = r.split(sep).collect::<Vec<_>>();
            if col.max() >= r.len() {
                println!("ignore a bad line # {:?}!", r);
            } else {
                let r = col.iter().map(|&i| r[i]).collect::<Vec<_>>().join(",");
                *freq.entry(r).or_insert(0) += 1;
            }
        });

        prog.add_chuncks(1);
        prog.add_bytes(task.bytes);
        prog.print();
    }

    println!("");

    // apply ascending
    let mut freq = freq.into_iter().collect::<Vec<(_, _)>>();
    if ascending {
        freq.sort_by(|a, b| a.1.cmp(&b.1));
    } else {
        freq.sort_by(|a, b| b.1.cmp(&a.1));
    }

    // apply head n
    if n > 0 && freq.len() > n as usize {
        freq = freq[..(n as usize)].to_vec();
    }

    // export or print
    if export {
        let new_path = filename::new_path(&path, "-frequency");
        file::write_to_csv(&new_path, &names, freq);
        println!("Saved to file: {}", new_path.display());
    } else {
        print_table(&names, freq)
    }

    Ok(())
}

fn artificial_cols(col: &Columns) -> Vec<String> {
    col.iter()
        .map(|&i| String::from("col") + &i.to_string())
        .chain(std::iter::once("n".to_owned()))
        .collect::<Vec<_>>()
}

fn print_table(names: &Vec<String>, freq: Vec<(String, i32)>) {
    let mut builder = Builder::default();

    // header
    if names.len() > 0 {
        builder.set_columns(names);
    }

    // content
    for (key, n) in freq {
        let r = key
            .split(",")
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
