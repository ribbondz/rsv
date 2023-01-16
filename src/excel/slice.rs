use crate::utils::cli_result::CliResult;
use crate::utils::constants::TERMINATOR;
use crate::utils::excel::datatype_vec_to_string;
use crate::utils::excel_reader::ExcelReader;
use crate::utils::filename::new_path;
use crate::utils::file::file_or_stdout_wtr;
use crate::utils::util::werr;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::process;

#[allow(clippy::too_many_arguments)]
pub fn run(
    path: &Path,
    sheet: usize,
    no_header: bool,
    start: usize,
    end: Option<usize>,
    length: Option<usize>,
    index: Option<usize>,
    export: bool,
) -> CliResult {
    // out file
    let out_path = new_path(path, "-slice");

    // open file
    let f = file_or_stdout_wtr(export, &out_path)?;

    let mut wtr = BufWriter::new(f);
    let mut rdr = ExcelReader::new(path, sheet)?;

    // header
    if !no_header {
        let r = match rdr.next() {
            Some(v) => datatype_vec_to_string(v),
            None => return Ok(()),
        };
        wtr.write_all(r.as_bytes())?;
        wtr.write_all(TERMINATOR)?;
    }

    // slice
    match index {
        Some(index) => write_by_index(&mut rdr, &mut wtr, index)?,
        None => {
            let end = end
                .or_else(|| length.map(|l| start + l - 1).or(Some(usize::MAX)))
                .unwrap();
            if start > end {
                werr!("Error: end index should be equal to or bigger than start index.");
                process::exit(1)
            }
            write_by_range(&mut rdr, &mut wtr, start, end)?;
        }
    }

    if export {
        println!("Saved to file: {}", out_path.display())
    }

    Ok(())
}

fn write_by_index(
    rdr: &mut ExcelReader,
    wtr: &mut BufWriter<Box<dyn Write>>,
    index: usize,
) -> std::io::Result<()> {
    for r in rdr.iter().skip(rdr.next_called + index).take(1) {
        let r = datatype_vec_to_string(r);
        wtr.write_all(r.as_bytes())?;
        wtr.write_all(TERMINATOR)?;
    }

    Ok(())
}

fn write_by_range(
    rdr: &mut ExcelReader,
    wtr: &mut BufWriter<Box<dyn Write>>,
    start: usize,
    end: usize,
) -> std::io::Result<()> {
    for r in rdr
        .iter()
        .skip(rdr.next_called + start)
        .take(end - start + 1)
    {
        let r = datatype_vec_to_string(r);
        wtr.write_all(r.as_bytes())?;
        wtr.write_all(TERMINATOR)?;
    }

    Ok(())
}
