use crossbeam_channel::bounded;
use rayon::prelude::*;

use crate::utils::column::Columns;
use crate::utils::file::estimate_line_count_by_mb;
use crate::utils::filename::{full_path_file, new_path};
use crate::utils::filter::Filter;
use crate::utils::progress::Progress;

use std::fs::File;
use std::io::{stdout, BufRead, BufWriter, Write};
use std::thread;
use std::{error::Error, io::BufReader};

struct Task {
    lines: Vec<String>,
    bytes: usize,
}

pub fn run(
    filename: &str,
    no_header: bool,
    sep: &str,
    cols: &str,
    filter: &str,
    export: bool,
) -> Result<(), Box<dyn Error>> {
    // current file
    let path = full_path_file(filename)?;

    // filters and cols
    let filter = Filter::new(filter);
    let cols = Columns::new(cols);

    // open file
    let f = match export {
        true => {
            let out_path = new_path(&path, "-selected");
            let f = File::create(&out_path)?;
            Box::new(f) as Box<dyn Write>
        }
        false => Box::new(stdout()) as Box<dyn Write>,
    };
    let mut wtr = BufWriter::new(f);
    let mut rdr = BufReader::new(File::open(&path)?).lines();

    // const
    let sep_bytes = sep.as_bytes();
    let terminator = &b"\n"[..];

    // header
    if !no_header {
        let row = rdr.next().unwrap()?;
        let row = row.split(sep).collect::<Vec<_>>();
        let record = match cols.all {
            true => row,
            false => cols.iter().map(|&i| row[i]).collect(),
        };
        print_record(&mut wtr, &record, sep_bytes, terminator)?;
    }

    // parallel queue
    let (tx, rx) = bounded(1);

    // read
    let line_buffer_n: usize = estimate_line_count_by_mb(filename, None);
    thread::spawn(move || {
        let mut lines = Vec::with_capacity(line_buffer_n);
        let mut n = 0;
        let mut bytes = 0;

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
    });

    // process
    let mut prog = Progress::new();
    for task in rx {
        // filter
        let filtered = task
            .lines
            .par_iter()
            .filter_map(|row| {
                let row = row.split(sep).collect::<Vec<_>>();
                if filter.record_is_valid(&row) {
                    Some(row)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        // write
        filtered.iter().for_each(|row| {
            if cols.all {
                print_record(&mut wtr, row, sep_bytes, terminator).unwrap()
            } else {
                let record = cols.iter().map(|&i| row[i]).collect::<Vec<_>>();
                print_record(&mut wtr, &record, sep_bytes, terminator).unwrap()
            }
        });

        if export {
            prog.add_chuncks(1);
            prog.add_bytes(task.bytes);
            prog.print();
        }
    }

    Ok(())
}

fn print_record(
    wtr: &mut BufWriter<Box<dyn Write>>,
    record: &Vec<&str>,
    sep_bytes: &[u8],
    terminator: &[u8],
) -> std::io::Result<()> {
    let mut it = record.iter().peekable();

    while let Some(&field) = it.next() {
        wtr.write(field.as_bytes())?;
        if it.peek().is_none() {
            wtr.write(terminator)?;
        } else {
            wtr.write(sep_bytes)?;
        }
    }

    Ok(())
}
