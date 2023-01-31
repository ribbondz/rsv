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
            Ok(_) => Some(String::from_utf8_lossy(&buf).trim_end().to_string()),
            Err(_) => return Ok(()),
        },
    };
    buf.clear();

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
        true => write_to_file(path, header, queue),
        false => print_to_stdout(header, queue),
    }

    Ok(())
}

fn write_to_file(path: &Path, header: Option<String>, queue: PriorityQueue<Vec<u8>>) {
    // new file
    let out = new_path(path, "-sampled");
    let mut wtr = Writer::new(&out).unwrap();
    if let Some(r) = header {
        wtr.write_line_unchecked(r);
    }
    for r in queue.into_sorted_items() {
        wtr.write_bytes_unchecked(&r.item);
    }

    println!("Saved to file: {}", out.display());
}

fn print_to_stdout(header: Option<String>, queue: PriorityQueue<Vec<u8>>) {
    let mut table = Table::new();

    let header = header.unwrap_or_default();
    if !header.is_empty() {
        table.add_record(vec!["#", "", &header]);
    }

    let v = queue
        .into_sorted_items()
        .into_iter()
        .map(|i| {
            (
                i.line_n_as_string(),
                String::from_utf8_lossy(&i.item).to_string(),
            )
        })
        .collect::<Vec<_>>();

    v.iter()
        .for_each(|(line_n, r)| table.add_record(vec![line_n.as_str(), "->", r]));

    table.print_blank_unchecked();
}
