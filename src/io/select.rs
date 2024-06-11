use crate::args::Select;
use crate::utils::column::Columns;
use crate::utils::filename::new_file;
use crate::utils::filter::Filter;
use crate::utils::util::valid_sep;
use crate::utils::{cli_result::CliResult, writer::Writer};
use std::io::{stdin, BufRead};

impl Select {
    pub fn io_run(&self) -> CliResult {
        let sep = valid_sep(&self.sep);
        // current file
        let out = new_file("selected.csv");

        // filters and cols
        let mut filter = Filter::new(&self.filter);
        let mut col = Columns::new(&self.cols);

        // open file
        let mut wtr = Writer::file_or_stdout(self.export, &out)?;
        let mut rdr = stdin().lock().lines();

        // const
        let sep_bytes = &sep.as_bytes();

        // header
        if !self.no_header {
            let Some(r) = rdr.next() else { return Ok(()) };
            let r = r?;

            let fields = r.split(&sep).collect::<Vec<_>>();
            col = col.total_col(fields.len()).parse();
            filter = filter.total_col(fields.len()).parse();

            if col.select_all {
                wtr.write_str_unchecked(&r)
            } else {
                let r = col.iter().map(|&i| fields[i]).collect::<Vec<_>>();
                wtr.write_fields_unchecked(&r, Some(sep_bytes));
            }
        }

        for r in rdr {
            let r = r?;

            if !col.parsed {
                let n = r.split(&sep).count();
                col = col.total_col(n).parse();
            }
            if !filter.parsed {
                let n = r.split(&sep).count();
                filter = filter.total_col(n).parse();
            }

            if filter.is_empty() && col.select_all {
                wtr.write_str_unchecked(r);
                continue;
            }

            let mut f = r.split(&sep).collect::<Vec<_>>();
            if !filter.is_empty() && !filter.record_is_valid(&f) {
                continue;
            }

            if !col.select_all {
                f = col.iter().map(|&i| f[i]).collect();
            }

            wtr.write_fields_unchecked(&f, Some(sep_bytes));
        }

        if self.export {
            println!("Saved to file: {}", out.display())
        }

        Ok(())
    }
}
