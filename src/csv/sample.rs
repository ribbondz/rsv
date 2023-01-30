use crate::utils::cli_result::CliResult;
use crate::utils::filename::new_path;
use crate::utils::priority_queue::PriorityQueue;
use crate::utils::table::Table;
use crate::utils::writer::Writer;
use rand::rngs::StdRng;
use rand::thread_rng;
use rand::{Rng, SeedableRng};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::Instant;

pub fn run(
    path: &Path,
    no_header: bool,
    n: usize,
    seed: Option<usize>,
    export: bool,
    time_limit: f32,
) -> CliResult {
    // open files
    let mut rdr = BufReader::new(File::open(path)?);

    // header
    let mut buf = vec![];
    let header = match no_header {
        true => None,
        false => match rdr.read_until(b'\n', &mut buf) {
            Ok(_) => {
                let r = String::from_utf8_lossy(&buf).trim_end().to_string();
                buf.clear();
                Some(r)
            }
            Err(_) => return Ok(()),
        },
    };

    // seed
    let mut rng = match seed {
        Some(s) => StdRng::seed_from_u64(s as u64),
        None => StdRng::from_rng(thread_rng())?,
    };

    // read
    let mut queue = PriorityQueue::with_capacity(n);
    let mut line_n = 0;
    let time = Instant::now();
    while let Ok(bytes_read) = rdr.read_until(b'\n', &mut buf) {
        if bytes_read == 0 {
            break;
        }

        let priority = rng.gen::<f64>();
        if queue.can_insert(priority) {
            let line = buf[..bytes_read].to_owned();
            queue.push(line_n, priority, line);
        }

        buf.clear();
        line_n += 1;

        if time_limit > 0.0 && line_n % 10000 == 0 && time.elapsed().as_secs_f32() >= time_limit {
            break;
        }
    }

    match export {
        true => write_to_file(path, export, header, queue),
        false => print_to_stdout(header, queue),
    }

    Ok(())
}

fn write_to_file(path: &Path, export: bool, header: Option<String>, queue: PriorityQueue<Vec<u8>>) {
    // new file
    let out = new_path(path, "-sampled");
    let mut wtr = Writer::file_or_stdout(export, &out).unwrap();
    if let Some(r) = header {
        wtr.write_line_unchecked(r);
    }
    for r in queue.iter() {
        wtr.write_bytes_unchecked(&r.item);
    }

    println!("Saved to file: {}", out.display());
}

fn print_to_stdout(header: Option<String>, queue: PriorityQueue<Vec<u8>>) {
    let mut sample = vec![];
    if let Some(r) = header {
        sample.push(vec!["#".to_owned(), "".to_owned(), r])
    }

    for r in queue.iter() {
        sample.push(vec![
            r.line_n.to_string(),
            "->".to_owned(),
            String::from_utf8_lossy(&r.item).trim_end().to_owned(),
        ])
    }

    Table::from_records(sample).print_blank_unchecked();
}
