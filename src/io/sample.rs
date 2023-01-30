use crate::utils::cli_result::CliResult;
use crate::utils::filename::new_file;
use crate::utils::priority_queue::PriorityQueue;
use crate::utils::table::Table;
use crate::utils::writer::Writer;
use rand::rngs::StdRng;
use rand::thread_rng;
use rand::{Rng, SeedableRng};
use std::io::{stdin, BufRead};
use std::time::Instant;

pub fn run(
    no_header: bool,
    n: usize,
    seed: Option<usize>,
    export: bool,
    time_limit: f32,
) -> CliResult {
    // open files
    let lines = stdin()
        .lock()
        .lines()
        .filter_map(|i| i.ok())
        .collect::<Vec<_>>();

    if lines.is_empty() {
        return Ok(());
    }

    // header
    let header = match no_header {
        true => None,
        false => Some(lines[0].to_owned()),
    };

    // seed
    let mut rng = match seed {
        Some(s) => StdRng::seed_from_u64(s as u64),
        None => StdRng::from_rng(thread_rng())?,
    };

    // read
    let mut queue = PriorityQueue::with_capacity(n);
    let time = Instant::now();
    for (line_n, r) in lines.into_iter().skip(1 - no_header as usize).enumerate() {
        let priority = rng.gen::<f64>();
        if queue.can_insert(priority) {
            queue.push(line_n, priority, r);
        }

        if time_limit > 0.0 && line_n % 10000 == 0 && time.elapsed().as_secs_f32() >= time_limit {
            break;
        }
    }

    match export {
        true => write_to_file(header, queue)?,
        false => print_to_stdout(header, queue),
    }

    Ok(())
}

fn write_to_file(header: Option<String>, queue: PriorityQueue<String>) -> CliResult {
    // new file
    let out = new_file("sampled.csv");
    let mut wtr = Writer::new(&out)?;
    if let Some(r) = header {
        wtr.write_line_unchecked(r);
    }
    for r in queue.iter() {
        wtr.write_line_unchecked(&r.item);
    }

    println!("Saved to file: {}", out.display());

    Ok(())
}

fn print_to_stdout(header: Option<String>, queue: PriorityQueue<String>) {
    let mut sample = vec![];
    if let Some(r) = header {
        sample.push(vec!["#".to_owned(), "".to_owned(), r])
    }

    for r in queue.v {
        sample.push(vec![r.line_n.to_string(), "->".to_owned(), r.item])
    }

    Table::from_records(sample).print_blank_unchecked();
}
