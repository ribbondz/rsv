use crate::utils::column::Columns;
use crate::utils::filename::new_file;
use crate::utils::regex::Re;
use crate::utils::{cli_result::CliResult, writer::Writer};
use std::io::{self, BufRead};

pub fn run(
    sep: &str,
    filter: &str,
    cols: &str,
    pattern: &str,
    no_header: bool,
    export: bool,
) -> CliResult {
    // wtr and rdr
    let out = new_file("searched.csv");
    let mut wtr = Writer::file_or_stdout(export, &out)?;
    let mut cols = Columns::new(cols);
    let mut filter = Columns::new(filter);

    // read
    let mut rdr = io::stdin().lock().lines();

    // header
    if !no_header {
        let Some(r) = rdr.next() else { return Ok(()) };
        let r = r?;

        let mut fields = r.split(sep).collect::<Vec<_>>();
        cols = cols.total_col(fields.len()).parse();
        filter = filter.total_col(fields.len()).parse();

        if cols.select_all {
            wtr.write_line_unchecked(&r)
        } else {
            fields = cols.iter().map(|&i| fields[i]).collect();
            wtr.write_line_by_field_unchecked(&fields, Some(sep.as_bytes()));
        }
    }

    // regex
    let re = Re::new(pattern)?;
    let mut matched = 0;
    for r in rdr {
        let r = r?;

        if !cols.parsed {
            let n = r.split(sep).count();
            cols = cols.total_col(n).parse();
        }
        if !filter.parsed {
            let n = r.split(sep).count();
            filter = filter.total_col(n).parse();
        }

        match (cols.select_all, filter.select_all) {
            (true, true) => {
                if re.is_match(&r) {
                    matched += 1;
                    wtr.write_line(&r)?;
                }
            }
            (true, false) => {
                let f = r.split(sep).collect::<Vec<_>>();
                if filter.iter().any(|&i| re.is_match(f[i])) {
                    wtr.write_line(&r)?;
                }
            }
            (false, true) => {
                if re.is_match(&r) {
                    let r = r.split(sep).collect::<Vec<_>>();
                    let r = cols.iter().map(|&i| r[i]).collect::<Vec<_>>();
                    wtr.write_line_by_field_unchecked(&r, Some(sep.as_bytes()))
                }
            }
            (false, false) => {
                let r = r.split(sep).collect::<Vec<_>>();
                if filter.iter().any(|&i| re.is_match(r[i])) {
                    let r = cols.iter().map(|&i| r[i]).collect::<Vec<_>>();
                    wtr.write_line_by_field_unchecked(&r, Some(sep.as_bytes()))
                }
            }
        }
    }

    if export {
        println!("Matched rows: {matched}");
        println!("Saved to file: {}", out.display())
    }

    Ok(())
}
