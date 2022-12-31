use crossbeam_channel::bounded;
use dashmap::DashMap;
use rayon::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::thread;
use tabled::builder::Builder;
use tabled::Style;

use crate::utils::file::{self, estimate_line_count_by_mb};
use crate::utils::filename;
use crate::utils::progress::Progress;

struct Task {
    lines: Vec<String>,
    bytes: usize,
}

pub fn frequency(
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
    let col = parse_cols(cols).expect("Error in col syntax!");
    if col.iter().any(|&i| i < 0) {
        panic!("Col index should be >=0!")
    }
    let col: Vec<_> = col.iter().map(|&i| i as usize).collect();
    let max_col = *col.iter().max().unwrap();

    // open file and header
    let mut rdr = BufReader::new(File::open(&path)?).lines();
    let names: Vec<String> = if no_header {
        null_col_names(&col)
    } else {
        let first_row = rdr.next().unwrap()?;
        let r = first_row.split(sep).collect::<Vec<_>>();
        if max_col >= r.len() {
            println!("read a bad line # {:?}!", r);
            null_col_names(&col)
        } else {
            col.iter()
                .map(|&i| r[i].to_owned())
                .chain(std::iter::once("n".to_owned()))
                .collect::<Vec<String>>()
        }
    };

    // read file
    let (tx, rx) = bounded(1);
    let line_buffer_n: usize = estimate_line_count_by_mb(filename, None);
    thread::spawn(move || {
        let mut bytes = 0;
        let mut lines: Vec<String> = Vec::with_capacity(line_buffer_n);
        let mut n = 0;
        for l in rdr {
            let l = l.unwrap();
            n += 1;
            bytes += l.len();
            lines.push(l);
            if n >= line_buffer_n {
                tx.send(Task { lines, bytes }).unwrap();
                n = 0;
                bytes = 0;
                lines = Vec::with_capacity(line_buffer_n);
            }
        }
        if lines.len() > 0 {
            tx.send(Task { lines, bytes }).unwrap();
        }
        drop(tx);
    });

    // process
    let freq = DashMap::new();
    let mut prog = Progress::new();
    for task in rx {
        task.lines.par_iter().for_each(|r| {
            let r = r.split(sep).collect::<Vec<_>>();
            if max_col >= r.len() {
                println!("ignore a bad line # {:?}!", r);
            } else {
                let r = col.iter().map(|&i| r[i]).collect::<Vec<&str>>().join(",");
                *freq.entry(r).or_insert(0) += 1;
            }
        });

        prog.add_chuncks(1);
        prog.add_bytes(task.bytes);
        prog.print();
    }
    println!("");

    let mut f2 = Vec::with_capacity(freq.len());
    for k in freq {
        f2.push(k);
    }
    let mut freq = f2;
    // apply ascending
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
        println!("Saved to new file: {:?}.", new_path);
    } else {
        print_table(&names, freq)
    }
    Ok(())
}

fn parse_cols(cols: &str) -> Result<Vec<isize>, std::num::ParseIntError> {
    let cols: Result<Vec<isize>, _> = cols
        .split(",")
        .filter(|&i| i != "")
        .map(|i| i.parse())
        .collect();
    cols
}

fn null_col_names(col: &Vec<usize>) -> Vec<String> {
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
        let r: Vec<String> = key
            .split(",")
            .map(|i| i.to_owned())
            .chain(std::iter::once(n.to_string()))
            .collect();
        builder.add_record(r);
    }
    // build
    let mut table = builder.build();
    // style
    table.with(Style::blank());
    println!("{table}");
}
