use crate::args::Search;
use crate::utils::cli_result::CliResult;
use crate::utils::column::Columns;
use crate::utils::file::estimate_line_count_by_mb;
use crate::utils::filename::new_path;
use crate::utils::progress::Progress;
use crate::utils::reader::ChunkReader;
use crate::utils::regex::Re;
use crate::utils::util::valid_sep;
use crate::utils::writer::Writer;
use crossbeam_channel::bounded;
use rayon::prelude::*;
use std::thread;

impl Search {
    pub fn csv_run(&self) -> CliResult {
        let sep = valid_sep(&self.sep);
        let path = &self.path();
        let cols = Columns::new(&self.cols).total_col_of(path, &sep).parse();
        let filter = Columns::new(&self.filter).total_col_of(path, &sep).parse();

        // wtr and rdr
        let out = new_path(path, "-searched");
        let mut wtr = Writer::file_or_stdout(self.export, &out)?;
        let mut rdr = ChunkReader::new(path)?;

        // header
        if !self.no_header {
            let Some(r) = rdr.next() else { return Ok(()) };
            let r = r?;
            if cols.select_all {
                wtr.write_str_unchecked(&r)
            } else {
                let mut r = r.split(&sep).collect::<Vec<_>>();
                r = cols.iter().map(|&i| r[i]).collect();
                wtr.write_fields_unchecked(&r, Some(&sep.as_bytes()));
            }
        }

        // read file
        let (tx, rx) = bounded(2);
        let line_buffer_n: usize = estimate_line_count_by_mb(path, Some(10));
        thread::spawn(move || rdr.send_to_channel_by_chunks(tx, line_buffer_n));

        // progress for export option
        let mut prog = Progress::new();

        // regex search
        let re = Re::new(&self.pattern)?;
        let mut matched_n = 0;
        for task in rx {
            matched_n += match (filter.select_all, cols.select_all) {
                (true, true) => {
                    let lines = task
                        .lines
                        .par_iter()
                        .filter(|&i| re.is_match(i))
                        .collect::<Vec<_>>();
                    wtr.write_strings_unchecked(&lines);
                    lines.len()
                }
                (true, false) => {
                    let lines = task
                        .lines
                        .par_iter()
                        .filter_map(|r| {
                            re.is_match(r).then_some({
                                let r = r.split(&sep).collect::<Vec<_>>();
                                cols.iter().map(|&i| r[i]).collect::<Vec<_>>()
                            })
                        })
                        .collect::<Vec<_>>();
                    wtr.write_fields_of_lines_unchecked(&lines, Some(&sep.as_bytes()));
                    lines.len()
                }
                (false, true) => {
                    let lines = task
                        .lines
                        .par_iter()
                        .filter(|r| {
                            let r = r.split(&sep).collect::<Vec<_>>();
                            filter.iter().any(|&i| re.is_match(r[i]))
                        })
                        .collect::<Vec<_>>();
                    wtr.write_strings_unchecked(&lines);
                    lines.len()
                }
                (false, false) => {
                    let lines = task
                        .lines
                        .par_iter()
                        .filter_map(|r| {
                            let r = r.split(&sep).collect::<Vec<_>>();
                            filter
                                .iter()
                                .any(|&i| re.is_match(r[i]))
                                .then_some(cols.iter().map(|&i| r[i]).collect::<Vec<_>>())
                        })
                        .collect::<Vec<_>>();
                    wtr.write_fields_of_lines_unchecked(&lines, Some(&sep.as_bytes()));
                    lines.len()
                }
            };

            if self.export {
                prog.add_chunks(1);
                prog.add_bytes(task.bytes);
                prog.print();
            }
        }

        if self.export {
            println!("\nMatched rows: {matched_n}");
            println!("Saved to file: {}", out.display());
        }

        Ok(())
    }
}
