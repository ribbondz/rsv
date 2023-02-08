use crate::utils::{
    cli_result::CliResult, constants::TERMINATOR, reader::ExcelReader, writer::Writer,
};
use std::path::Path;

pub fn run(path: &Path, sheet: usize, sep: &str) -> CliResult {
    // new file
    let out = path.with_extension("csv");

    // open files
    let range = ExcelReader::new(path, sheet)?;
    let mut wtr = Writer::new(&out)?;

    // const
    let sep_bytes = sep.as_bytes();

    // excel2csv
    for r in range.iter() {
        let mut r = r.iter().peekable();
        while let Some(v) = r.next() {
            wtr.write_bytes(v.to_string().trim().as_bytes())?;
            if r.peek().is_some() {
                wtr.write_bytes(sep_bytes)?;
            } else {
                wtr.write_bytes(TERMINATOR)?;
            }
        }
    }

    println!("Saved to file: {}", out.display());

    Ok(())
}
