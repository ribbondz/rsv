use crate::args::Frequency;
use crate::utils::column::Columns;
use crate::utils::file;
use crate::utils::filename::new_file;
use crate::utils::util::print_frequency_table;
use crate::utils::{cli_result::CliResult, reader::IoReader};
use dashmap::DashMap;

impl Frequency {
    pub fn io_run(&self) -> CliResult {
        let lines = IoReader::new().lines();

        if lines.is_empty() {
            return Ok(());
        }

        // cols
        let n = self.row_field_count(&lines[0]);
        let col = Columns::new(&self.cols).total_col(n).parse();

        // open file and header

        let names: Vec<String> = if self.no_header {
            col.artificial_cols_with_appended_n()
        } else {
            let r = self.split_row_to_vec(&lines[0]);
            if col.max >= r.len() {
                println!("[info] ignore a bad line # {r:?}!");
                col.artificial_cols_with_appended_n()
            } else {
                col.select_owned_vector_and_append_n(&r)
            }
        };

        let freq = DashMap::new();
        for r in &lines[(1 - self.no_header as usize)..] {
            let r = self.split_row_to_vec(r);
            if col.max >= r.len() {
                println!("[info] ignore a bad line # {r:?}!");
            } else {
                let r = col.select_owned_string(&r);
                *freq.entry(r).or_insert(0) += 1;
            }
        }

        let mut freq = freq.into_iter().collect::<Vec<(_, _)>>();
        if self.ascending {
            freq.sort_by(|a, b| a.1.cmp(&b.1));
        } else {
            freq.sort_by(|a, b| b.1.cmp(&a.1));
        }

        // apply head n
        if self.n > 0 {
            freq.truncate(self.n as usize)
        }

        // export or print
        if self.export {
            let out = new_file("frequency.csv");
            file::write_frequency_to_csv(&out, &names, freq);
            println!("Saved to file: {}", out.display());
        } else {
            print_frequency_table(&names, freq)
        }

        Ok(())
    }
}
