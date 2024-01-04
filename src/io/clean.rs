use crate::utils::cli_result::CliResult;
use crate::utils::writer::Writer;
use regex::bytes::Regex;
use std::io::{stdin, BufRead, BufReader};

pub fn run(escape: &str) -> CliResult {
    let mut rdr = BufReader::new(stdin().lock());

    // open files
    let mut wtr = Writer::stdout()?;

    // copy
    let re = Regex::new(escape)?;
    let empty_bytes = &b""[..];

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
