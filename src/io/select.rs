use crate::utils::column::Columns;
use crate::utils::filename::new_file;
use crate::utils::filter::Filter;
use crate::utils::{cli_result::CliResult, writer::Writer};
use std::io::{stdin, BufRead};

pub fn run(no_header: bool, sep: &str, cols: &str, filter: &str, export: bool) -> CliResult {
    // current file
    let out_path = new_file("selected.csv");

    // filters and cols
    let filter = Filter::new(filter);
    let cols = Columns::new(cols);

    // open file
    let mut wtr = Writer::file_or_stdout(export, &out_path)?;
    let mut rdr = stdin().lock().lines();

    // const
    let sep_bytes = sep.as_bytes();

    // header
    if !no_header {
        match (rdr.next(), cols.all) {
            (Some(r), true) => wtr.write_line_unchecked(&r?),
            (Some(r), false) => {
                let r = r?;
                let mut r = r.split(sep).collect::<Vec<_>>();
                r = cols.iter().map(|&i| r[i]).collect();
                wtr.write_line_by_field_unchecked(&r, Some(sep_bytes));
            }
            (None, _) => return Ok(()),
        };
    }

    for r in rdr {
        let r = r?;
        if filter.is_empty() && cols.all {
            wtr.write_line_unchecked(r);
            continue;
        }
        let mut f = r.split(sep).collect::<Vec<_>>();
        if !filter.is_empty() && !filter.record_is_valid(&f) {
            continue;
        }
        if !cols.all {
            f = cols.iter().map(|&i| f[i]).collect();
        }
        wtr.write_line_by_field_unchecked(&f, Some(sep_bytes));
    }

    if export {
        println!("Saved to file: {}", out_path.display())
    }

    Ok(())
}
