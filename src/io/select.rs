use crate::utils::cli_result::CliResult;
use crate::utils::column::Columns;
use crate::utils::constants::TERMINATOR;
use crate::utils::file::file_or_stdout_wtr;
use crate::utils::filename::new_file;
use crate::utils::filter::Filter;
use std::io::{stdin, BufRead, BufWriter, Write};

pub fn run(no_header: bool, sep: &str, cols: &str, filter: &str, export: bool) -> CliResult {
    // current file
    let out_path = new_file("selected.csv");

    // filters and cols
    let filter = Filter::new(filter);
    let cols = Columns::new(cols);

    // open file
    let f = file_or_stdout_wtr(export, &out_path)?;
    let mut wtr = BufWriter::new(f);
    let mut rdr = stdin().lock().lines();

    // const
    let sep_bytes = sep.as_bytes();

    // header
    if !no_header {
        match rdr.next() {
            Some(v) => {
                let v = v?;
                let row = v.split(sep).collect::<Vec<_>>();
                let record = match cols.all {
                    true => row,
                    false => cols.iter().map(|&i| row[i]).collect(),
                };
                print_record(&mut wtr, &record, sep_bytes)?;
            }
            None => return Ok(()),
        };
    }

    for r in rdr {
        let r = r?;
        let r = r.split(sep).collect::<Vec<_>>();
        if filter.record_is_valid(&r) {
            if cols.all {
                print_record(&mut wtr, &r, sep_bytes).unwrap()
            } else {
                let record = cols.iter().map(|&i| r[i]).collect::<Vec<_>>();
                print_record(&mut wtr, &record, sep_bytes).unwrap()
            }
        }
    }

    if export {
        println!("Saved to file: {}", out_path.display())
    }
    Ok(())
}

fn print_record(
    wtr: &mut BufWriter<Box<dyn Write>>,
    record: &[&str],
    sep_bytes: &[u8],
) -> std::io::Result<()> {
    let mut it = record.iter().peekable();

    while let Some(&field) = it.next() {
        wtr.write_all(field.as_bytes())?;

        if it.peek().is_none() {
            wtr.write_all(TERMINATOR)?;
        } else {
            wtr.write_all(sep_bytes)?;
        }
    }

    Ok(())
}
