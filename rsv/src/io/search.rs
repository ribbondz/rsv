use crate::args::Search;
use rsv_lib::utils::column::Columns;
use rsv_lib::utils::filename::new_file;
use rsv_lib::utils::regex::Re;
use rsv_lib::utils::{cli_result::CliResult, writer::Writer};
use std::io::{self, BufRead};

impl Search {
    pub fn io_run(&self) -> CliResult {
        // wtr and rdr
        let out = new_file("searched.csv");
        let mut wtr = Writer::file_or_stdout(self.export, &out)?;
        let mut cols = Columns::new(&self.cols);
        let mut filter = Columns::new(&self.filter);

        // read
        let mut rdr = io::stdin().lock().lines();

        // header
        if !self.no_header {
            let Some(r) = rdr.next() else { return Ok(()) };
            let r = r?;

            let mut fields = self.split_row_to_vec(&r);
            cols = cols.total_col(fields.len()).parse();
            filter = filter.total_col(fields.len()).parse();

            if cols.select_all {
                wtr.write_str_unchecked(&r)
            } else {
                fields = cols.iter().map(|&i| fields[i]).collect();
                wtr.write_fields_unchecked(&fields);
            }
        }

        // regex
        let re = Re::new(&self.pattern)?;
        let mut matched = 0;
        for r in rdr {
            let r = r?;

            if !cols.parsed {
                let n = self.row_field_count(&r);
                cols = cols.total_col(n).parse();
            }
            if !filter.parsed {
                let n = self.row_field_count(&r);
                filter = filter.total_col(n).parse();
            }

            match (cols.select_all, filter.select_all) {
                (true, true) => {
                    if re.is_match(&r) {
                        matched += 1;
                        wtr.write_str(&r)?;
                    }
                }
                (true, false) => {
                    let f = self.split_row_to_vec(&r);
                    if filter.iter().any(|&i| re.is_match(f[i])) {
                        wtr.write_str(&r)?;
                    }
                }
                (false, true) => {
                    if re.is_match(&r) {
                        let r = self.split_row_to_vec(&r);
                        let r = cols.iter().map(|&i| r[i]).collect::<Vec<_>>();
                        wtr.write_fields_unchecked(&r)
                    }
                }
                (false, false) => {
                    let r = self.split_row_to_vec(&r);
                    if filter.iter().any(|&i| re.is_match(r[i])) {
                        let r = cols.iter().map(|&i| r[i]).collect::<Vec<_>>();
                        wtr.write_fields_unchecked(&r)
                    }
                }
            }
        }

        if self.export {
            println!("Matched rows: {matched}");
            println!("Saved to file: {}", out.display())
        }

        Ok(())
    }
}
