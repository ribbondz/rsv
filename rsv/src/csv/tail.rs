use crate::args::Tail;
use rsv_lib::utils::cli_result::CliResult;
use rsv_lib::utils::filename::new_path;
use rsv_lib::utils::writer::Writer;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

impl Tail {
    pub fn csv_run(&self) -> CliResult {
        let path = &self.path();
        let n = if self.n == 0 { usize::MAX - 10 } else { self.n };
        let out = new_path(path, "-tail");
        let mut rdr = BufReader::new(File::open(path)?);
        let mut wtr = Writer::file_or_stdout(self.export, &out)?;

        // show head n
        let mut lines = VecDeque::new();

        // header
        let mut buf = vec![];
        if !self.no_header {
            rdr.read_until(b'\n', &mut buf)?;
            wtr.write_bytes_unchecked(&buf);
        };
        buf.clear();

        // read
        while let Ok(nread) = rdr.read_until(b'\n', &mut buf) {
            if nread == 0 {
                break;
            }
            if lines.len() >= n {
                lines.pop_front();
            }
            lines.push_back(buf[..nread].to_owned());
            buf.clear();
        }

        lines.iter().for_each(|r| wtr.write_bytes_unchecked(r));

        if self.export {
            println!("Saved to file: {}", out.display())
        }

        Ok(())
    }
}
