use crate::args::Sample;
use rand::rng;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use rsv_lib::utils::cli_result::CliResult;
use rsv_lib::utils::filename::new_path;
use rsv_lib::utils::priority_queue::PriorityQueue;
use rsv_lib::utils::table::Table;
use rsv_lib::utils::writer::Writer;
use std::borrow::Cow;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::Instant;

impl Sample {
    pub fn csv_run(&self) -> CliResult {
        let path = &self.path();
        let time_limit = (self.time_limit - 0.7).clamp(0.0, f32::MAX);

        // open files
        let mut rdr = BufReader::new(File::open(path)?);

        // header
        let mut buf = vec![];
        let header = match self.no_header {
            true => None,
            false => match rdr.read_until(b'\n', &mut buf) {
                Ok(_) => Some(String::from_utf8_lossy(&buf).trim().to_string()),
                Err(_) => return Ok(()),
            },
        };
        buf.clear();

        // seed
        let mut rng = match self.seed {
            Some(s) => StdRng::seed_from_u64(s as u64),
            None => StdRng::from_rng(&mut rng()),
        };

        // read
        let mut queue = PriorityQueue::with_capacity(self.n);
        let mut line_n = 0;
        let time = Instant::now();
        while let Ok(bytes_read) = rdr.read_until(b'\n', &mut buf) {
            if bytes_read == 0 {
                break;
            }

            let priority = rng.random::<f64>();
            if queue.can_insert(priority) {
                let line = buf[..bytes_read].to_owned();
                queue.push(line_n, priority, line);
            }

            buf.clear();
            line_n += 1;

            if time_limit > 0.0 && line_n % 10000 == 0 && time.elapsed().as_secs_f32() >= time_limit
            {
                break;
            }
        }

        match (self.export, self.show_number) {
            (true, _) => write_to_file(path, header, queue),
            (false, true) => print_to_stdout(header, queue),
            (false, false) => print_to_stdout_no_number(header, queue),
        }

        Ok(())
    }
}

fn write_to_file(path: &Path, header: Option<String>, queue: PriorityQueue<Vec<u8>>) {
    // new file
    let out = new_path(path, "-sampled");
    let mut wtr = Writer::new(&out).unwrap();
    if let Some(r) = header {
        wtr.write_str_unchecked(r);
    }
    for r in queue.into_sorted_items() {
        wtr.write_bytes_unchecked(&r.item);
    }

    println!("Saved to file: {}", out.display());
}

fn print_to_stdout(header: Option<String>, queue: PriorityQueue<Vec<u8>>) {
    let mut table = Table::new();

    if let Some(h) = header {
        table.add_record([Cow::Borrowed("#"), Cow::Borrowed(""), Cow::from(h)]);
    }

    queue.into_sorted_items().into_iter().for_each(|i| {
        table.add_record([
            Cow::from(i.line_n_as_string()),
            Cow::Borrowed("->"),
            Cow::from(String::from_utf8_lossy(&i.item).trim().to_string()),
        ])
    });

    table.print_blank_unchecked();
}

fn print_to_stdout_no_number(header: Option<String>, queue: PriorityQueue<Vec<u8>>) {
    let mut wtr = Writer::stdout().unwrap();

    if let Some(h) = header {
        wtr.write_str_unchecked(h);
    }

    queue.into_sorted_items().into_iter().for_each(|i| {
        wtr.write_bytes_unchecked(&i.item);
    });
}
