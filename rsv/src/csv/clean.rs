use crate::args::Clean;
use regex::bytes::Regex;
use rsv_lib::utils::cli_result::CliResult;
use rsv_lib::utils::filename;
use rsv_lib::utils::progress::Progress;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

impl Clean {
    pub fn csv_run(&self) -> CliResult {
        let path = &self.path();

        // new file
        let new_path = match self.output.is_empty() {
            true => filename::new_path(path, "-cleaned"),
            false => Path::new(&self.output).into(),
        };

        // open files
        let mut rdr = BufReader::new(File::open(path)?);
        let mut wtr = BufWriter::new(File::create(&new_path)?);

        // progress
        let mut prog = Progress::new();

        // copy
        let re = Regex::new(&self.escape)?;
        let empty_bytes = b"";

        let mut buf = vec![];
        let mut i = 0;
        while let Ok(bytes_read) = rdr.read_until(b'\n', &mut buf) {
            if bytes_read == 0 {
                break;
            }

            let str = re.replace_all(&buf[..bytes_read], empty_bytes);
            wtr.write_all(&str)?;
            buf.clear();

            // progress print
            prog.add_bytes(bytes_read);
            if i % 50_000 == 0 {
                prog.add_chunks(1);
                prog.print();
            }

            i += 1;
        }

        prog.print();

        println!("\nSaved to file: {}", new_path.display());

        Ok(())
    }
}
