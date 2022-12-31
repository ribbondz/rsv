use ahash::AHashMap;
use crossbeam_channel::bounded;
use dashmap::DashMap;
use rayon::prelude::*;
use std::fs::{create_dir, File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;
use std::thread;

use crate::utils::file::estimate_line_count_by_mb;
use crate::utils::filename::generate_filename;
use crate::utils::progress::Progress;
use crate::utils::util::datetime_str;

struct Task {
    lines: Vec<String>,
    bytes: usize,
}

pub fn partition(
    filename: &str,
    no_header: bool,
    sep: &str,
    col: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    // current file
    let mut path = std::env::current_dir()?;
    path.push(Path::new(filename));

    // new directory
    let mut filename_only = Path::new(filename).file_name().unwrap().to_str().unwrap();
    if filename_only.contains(".") {
        filename_only = filename_only.split(".").collect::<Vec<_>>()[0];
    }
    let mut dir = path.clone();
    dir.pop();
    dir.push(format!("{}-partition-{}", filename_only, datetime_str()));
    create_dir(&dir)?;

    // open file and header
    let mut rdr = BufReader::new(File::open(&path)?).lines();
    let first_row = if no_header {
        "".to_owned()
    } else {
        let first_row = rdr.next().unwrap()?;
        if col >= first_row.split(sep).count() {
            panic!("Column index out of its range!");
        } else {
            first_row
        }
    };

    let (tx, rx) = bounded(1);

    // read
    let line_buffer_n = estimate_line_count_by_mb(filename, Some(1024));
    thread::spawn(move || {
        let mut bytes = 0;
        let mut n = 0;
        let mut lines: Vec<String> = Vec::with_capacity(line_buffer_n);
        for l in rdr {
            let l = l.unwrap();
            bytes += l.len() + 1;
            n += 1;
            lines.push(l);
            if n >= line_buffer_n {
                tx.send(Task { lines, bytes }).unwrap();
                bytes = 0;
                n = 0;
                lines = Vec::with_capacity(line_buffer_n);
            }
        }
        if lines.len() > 0 {
            tx.send(Task { lines, bytes }).unwrap();
        }
        drop(tx);
    });

    // process batch work
    let mut header_inserted = AHashMap::new();
    let mut prog = Progress::new();
    let terminator = &[b'\n'];
    for task in rx {
        // progress
        prog.add_chuncks(1);
        prog.add_bytes(task.bytes);

        // process
        let batch_work = DashMap::new();
        task.lines.par_iter().for_each(|r| {
            let seg = r.split(sep).collect::<Vec<_>>();
            if col >= r.len() {
                println!("ignore a bad line, content is: {:?}!", r);
            } else {
                batch_work.entry(seg[col]).or_insert(Vec::new()).push(r);
            }
        });

        // save to disk
        for (k, v) in batch_work {
            // file path
            let filename = generate_filename(k, None);
            let mut path = dir.clone();
            path.push(&filename);

            // open file
            let f = OpenOptions::new()
                .write(true)
                .append(true)
                .create(true)
                .open(&path)?;
            let mut wtr = BufWriter::new(f);

            // header
            if !no_header && !header_inserted.contains_key(&filename) {
                header_inserted.insert(filename, true);
                wtr.write(first_row.as_bytes())?;
                wtr.write(terminator)?;
            }

            // content
            v.iter().for_each(|&r| {
                wtr.write(r.as_bytes()).unwrap();
                wtr.write(terminator).unwrap();
            });
        }

        prog.print();
    }

    println!("\nSaved to directory: {:?}", dir);
    Ok(())
}
