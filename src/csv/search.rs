use crate::utils::chunk_reader::ChunkReader;
use crate::utils::cli_result::CliResult;
use crate::utils::file::estimate_line_count_by_mb;
use crate::utils::filename::new_path;
use crate::utils::progress::Progress;
use crate::utils::regex::Re;
use crate::utils::writer::Writer;
use crossbeam_channel::bounded;
use rayon::prelude::*;
use std::path::Path;
use std::thread;

pub fn run(path: &Path, pattern: &str, no_header: bool, export: bool) -> CliResult {
    // wtr and rdr
    let out = new_path(path, "-searched");
    let mut wtr = Writer::file_or_stdout(export, &out)?;
    let mut rdr = ChunkReader::new(path)?;

    // header
    if !no_header {
        match rdr.next() {
            Some(r) => wtr.write_header_unchecked(&r?),
            None => return Ok(()),
        };
    };

    // read file
    let (tx, rx) = bounded(2);
    let line_buffer_n: usize = estimate_line_count_by_mb(path, Some(10));
    thread::spawn(move || rdr.send_to_channel_by_chunks(tx, line_buffer_n));

    // progress for export option
    let mut prog = Progress::new();

    // regex search
    let re = Re::new(pattern)?;
    let mut matched_n = 0;
    for task in rx {
        let lines = task
            .lines
            .par_iter()
            .filter(|&i| re.is_match(i))
            .collect::<Vec<_>>();

        // pipeline could be closed, e.g., when rsv head take enough items
        // ignore the error
        wtr.write_lines_unchecked(&lines);

        if export {
            matched_n += lines.len();
            prog.add_chunks(1);
            prog.add_bytes(task.bytes);
            prog.print();
        }
    }

    if export {
        println!("\nMatched rows: {matched_n}");
        println!("Saved to file: {}", out.display());
    }

    Ok(())
}
