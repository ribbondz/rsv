use crate::utils::{cli_result::CliResult, filename::new_file, reader::IoReader, writer::Writer};

pub fn run(no_header: bool, n: usize, export: bool) -> CliResult {
    let out = new_file("tail.csv");
    let mut wtr = Writer::file_or_stdout(export, &out)?;

    // lines
    let lines = IoReader::new().lines();

    // header
    if !no_header && !lines.is_empty() {
        wtr.write_line_unchecked(&lines[0]);
    }

    // show tail n
    lines
        .iter()
        .skip(1 - no_header as usize)
        .rev()
        .take(n)
        .rev()
        .for_each(|r| wtr.write_line_unchecked(r));

    if export {
        println!("Saved to file: {}", out.display())
    }

    Ok(())
}
