use crate::utils::cli_result::CliResult;
use crate::utils::filename::new_path;
use crate::utils::writer::Writer;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn run(path: &Path, no_header: bool, n: usize, export: bool) -> CliResult {
    let n = if n == 0 { usize::MAX - 10 } else { n };
    let out = new_path(path, "-tail");
    let mut rdr = BufReader::new(File::open(path)?);
    let mut wtr = Writer::file_or_stdout(export, &out)?;

    // show head n
    let mut lines = VecDeque::new();

    // header
    let mut buf = vec![];
    if !no_header {
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

    if export {
        println!("Saved to file: {}", out.display())
    }

    Ok(())
}
