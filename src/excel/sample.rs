use crate::utils::cli_result::CliResult;
use crate::utils::excel::datatype_vec_to_string;
use crate::utils::filename::new_path;
use crate::utils::priority_queue::PriorityQueue;
use crate::utils::reader::ExcelReader;
use crate::utils::table::Table;
use crate::utils::writer::Writer;
use rand::rngs::StdRng;
use rand::thread_rng;
use rand::{Rng, SeedableRng};
use std::borrow::Cow;
use std::path::Path;
use std::time::Instant;

pub fn run(
    path: &Path,
    sheet: usize,
    no_header: bool,
    n: usize,
    seed: Option<usize>,
    export: bool,
    time_limit: f32,
) -> CliResult {
    // open files
    let mut range = ExcelReader::new(path, sheet)?;

    // header
    let header = match no_header {
        true => None,
        false => match range.next() {
            Some(r) => Some(datatype_vec_to_string(r)),
            None => return Ok(()),
        },
    };

    // seed
    let mut rng = match seed {
        Some(s) => StdRng::seed_from_u64(s as u64),
        None => StdRng::from_rng(thread_rng())?,
    };

    // read
    let time = Instant::now();
    let mut queue = PriorityQueue::with_capacity(n);
    for (line_n, r) in range.iter().skip(range.next_called).enumerate() {
        let priority = rng.gen::<f64>();
        if queue.can_insert(priority) {
            let line = datatype_vec_to_string(r);
            queue.push(line_n, priority, line);
        }

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

fn write_to_file(path: &Path, header: Option<String>, queue: PriorityQueue<String>) {
    // new file
    let out = new_path(path, "-sampled").with_extension("csv");
    let mut wtr = Writer::new(&out).unwrap();
    if let Some(r) = header {
        wtr.write_line_unchecked(&r);
    }
    for r in queue.into_sorted_items() {
        wtr.write_line_unchecked(&r.item);
    }

    println!("Saved to file: {}", out.display());
}

fn print_to_stdout(header: Option<String>, queue: PriorityQueue<String>) {
    let mut table = Table::new();

    // header
    let header = match header {
        Some(v) => Cow::Owned(v),
        None => Cow::Borrowed(""),
    };
    if !header.is_empty() {
        table.add_record(vec!["#", "", &header]);
    }

    // samples
    let v = queue
        .into_sorted_items()
        .into_iter()
        .map(|i| (i.line_n_as_string(), i.item))
        .collect::<Vec<_>>();
    v.iter()
        .for_each(|(line_n, r)| table.add_record(vec![line_n.as_str(), "->", r]));

    table.print_blank_unchecked();
}
