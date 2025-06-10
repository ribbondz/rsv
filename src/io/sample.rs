use crate::args::Sample;
use crate::utils::cli_result::CliResult;
use crate::utils::filename::new_file;
use crate::utils::priority_queue::PriorityQueue;
use crate::utils::reader::IoReader;
use crate::utils::table::Table;
use crate::utils::writer::Writer;
use rand::rng;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::borrow::Cow;
use std::time::Instant;

impl Sample {
    pub fn io_run(&self) -> CliResult {
        let time_limit = (self.time_limit - 0.7).clamp(0.0, f32::MAX);

        // open files
        let lines = IoReader::new().lines();

        if lines.is_empty() {
            return Ok(());
        }

        // header
        let header = match self.no_header {
            true => None,
            false => Some(lines[0].to_owned()),
        };

        // seed
        let mut rng = match self.seed {
            Some(s) => StdRng::seed_from_u64(s as u64),
            None => StdRng::from_rng(&mut rng()),
        };

        // read
        let mut queue = PriorityQueue::with_capacity(self.n);
        let time = Instant::now();
        for (line_n, r) in lines
            .into_iter()
            .skip(1 - self.no_header as usize)
            .enumerate()
        {
            let priority = rng.random::<f64>();
            if queue.can_insert(priority) {
                queue.push(line_n, priority, r);
            }

            if time_limit > 0.0 && line_n % 10000 == 0 && time.elapsed().as_secs_f32() >= time_limit
            {
                break;
            }
        }

        match (self.export, self.show_number) {
            (true, _) => write_to_file(header, queue)?,
            (false, true) => print_to_stdout(header, queue)?,
            (false, false) => print_to_stdout_no_number(header, queue)?,
        };

        Ok(())
    }
}

fn write_to_file(header: Option<String>, queue: PriorityQueue<String>) -> CliResult {
    // new file
    let out = new_file("sampled.csv");
    let mut wtr = Writer::new(&out)?;
    if let Some(r) = header {
        wtr.write_str_unchecked(r);
    }
    for r in queue.into_sorted_items() {
        wtr.write_str_unchecked(&r.item);
    }

    println!("Saved to file: {}", out.display());

    Ok(())
}

fn print_to_stdout(header: Option<String>, queue: PriorityQueue<String>) -> CliResult {
    let mut table = Table::new();

    if let Some(h) = header {
        table.add_record([Cow::Borrowed("#"), Cow::Borrowed(""), Cow::from(h)]);
    }

    queue.into_sorted_items().into_iter().for_each(|i| {
        table.add_record([
            Cow::from(i.line_n_as_string()),
            Cow::Borrowed("->"),
            Cow::from(i.item),
        ])
    });

    table.print_blank_unchecked();

    Ok(())
}

fn print_to_stdout_no_number(header: Option<String>, queue: PriorityQueue<String>) -> CliResult {
    let mut wtr = Writer::stdout().unwrap();

    // header
    if let Some(h) = header {
        wtr.write_str_unchecked(h);
    }

    // samples
    queue.into_sorted_items().into_iter().for_each(|i| {
        wtr.write_str_unchecked(i.item);
    });

    Ok(())
}
