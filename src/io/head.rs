use crate::utils::{cli_result::CliResult, filename::new_file, reader::IoReader, writer::Writer};

pub fn run(no_header: bool, n: usize, export: bool) -> CliResult {
    let out = new_file("sorted.csv");
    let mut wtr = Writer::file_or_stdout(export, &out)?;

    // show head n
    let r = IoReader::new().no_header(no_header).top_n(n).lines();

    // tabled or not
    wtr.write_lines_unchecked(&r);

    if export {
        println!("Saved to file: {}", out.display())
    }

    Ok(())
}
