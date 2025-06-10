use crate::args::Select;
use crate::utils::column::Columns;
use crate::utils::filename::new_file;
use crate::utils::filter::Filter;
use crate::utils::{cli_result::CliResult, writer::Writer};
use std::io::{stdin, BufRead};

impl Select {
    pub fn io_run(&self) -> CliResult {
        // current file
        let out = new_file("selected.csv");

        // filters and cols
        let mut filter = Filter::new(&self.filter);
        let mut col = Columns::new(&self.cols);

        // open file
        let mut wtr = Writer::file_or_stdout(self.export, &out)?;
        let mut rdr = stdin().lock().lines();

        // header
        if !self.no_header {
            let Some(r) = rdr.next() else { return Ok(()) };
            let r = r?;

            let fields = self.split_row_to_vec(&r);
            col = col.total_col(fields.len()).parse();
            filter = filter.total_col(fields.len()).parse();

            if col.select_all {
                wtr.write_str_unchecked(&r)
            } else {
                let r = col.iter().map(|&i| fields[i]).collect::<Vec<_>>();
                wtr.write_fields_unchecked(&r);
            }
        }

        for r in rdr {
            let r = r?;

            if !col.parsed {
                let n = self.row_field_count(&r);
                col = col.total_col(n).parse();
            }
            if !filter.parsed {
                let n = self.row_field_count(&r);
                filter = filter.total_col(n).parse();
            }

            if filter.is_empty() && col.select_all {
                wtr.write_str_unchecked(r);
                continue;
            }

            let mut f = self.split_row_to_vec(&r);
            if !filter.is_empty() && !filter.record_is_valid(&f) {
                continue;
            }

            if !col.select_all {
                f = col.iter().map(|&i| f[i]).collect();
            }

            wtr.write_fields_unchecked(&f);
        }

        if self.export {
            println!("Saved to file: {}", out.display())
        }

        Ok(())
    }
}
