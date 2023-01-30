use crate::utils::cli_result::CliResult;
use crate::utils::excel::datatype_vec_to_string_vec;
use crate::utils::excel_reader::ExcelReader;
use crate::utils::filename::new_path;
use crate::utils::priority_queue::PriorityQueue;
use crate::utils::table::Table;
use crate::utils::writer::Writer;
use rand::rngs::StdRng;
use rand::thread_rng;
use rand::{Rng, SeedableRng};
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
            Some(r) => {
                let r = datatype_vec_to_string_vec(r);
                Some(r)
            }
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
            let line = datatype_vec_to_string_vec(r);
            queue.push(line_n, priority, line);
        }

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

fn write_to_file(
    path: &Path,
    export: bool,
    header: Option<Vec<String>>,
    queue: PriorityQueue<Vec<String>>,
) {
    // new file
    let out = new_path(path, "-sampled").with_extension("csv");
    let mut wtr = Writer::file_or_stdout(export, &out).unwrap();
    if let Some(r) = header {
        wtr.write_line_by_field_unchecked(&r, None);
    }
    for r in queue.iter() {
        wtr.write_line_by_field_unchecked(&r.item, None);
    }

    println!("Saved to file: {}", out.display());
}

fn print_to_stdout(header: Option<Vec<String>>, queue: PriorityQueue<Vec<String>>) {
    let mut sample = vec![];
    if let Some(r) = header {
        sample.push(vec!["#".to_owned(), "".to_owned(), r.join(",")])
    }

    for r in queue.iter() {
        sample.push(vec![
            r.line_n.to_string(),
            "->".to_owned(),
            r.item.join(","),
        ])
    }

    Table::from_records(sample).print_blank_unchecked();
}
