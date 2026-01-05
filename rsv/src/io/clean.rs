use crate::args::Clean;
use regex::bytes::Regex;
use rsv_lib::utils::cli_result::CliResult;
use rsv_lib::utils::writer::Writer;
use std::io::{BufRead, BufReader, stdin};

impl Clean {
    pub fn io_run(&self) -> CliResult {
        let mut rdr = BufReader::new(stdin().lock());

        // open files
        let mut wtr = Writer::stdout()?;

        // copy
        let re = Regex::new(&self.escape)?;
        let empty_bytes = b"";

        let mut buf = vec![];
        while let Ok(bytes_read) = rdr.read_until(b'\n', &mut buf) {
            if bytes_read == 0 {
                break;
            }

            let str = re.replace_all(&buf[..bytes_read], empty_bytes);
            wtr.write_bytes(&str)?;
            buf.clear();
        }

        Ok(())
    }
}
