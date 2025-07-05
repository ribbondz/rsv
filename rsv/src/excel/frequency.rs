use crate::args::Frequency;
use dashmap::DashMap;
use rsv_lib::utils::cli_result::CliResult;
use rsv_lib::utils::column::Columns;
use rsv_lib::utils::file;
use rsv_lib::utils::filename;
use rsv_lib::utils::reader::ExcelReader;
use rsv_lib::utils::util::print_frequency_table;

impl Frequency {
    pub fn excel_run(&self) -> CliResult {
        let path = &self.path();

        // open file and header
        let mut rdr = ExcelReader::new(path, self.sheet)?;

        // cols
        let col = Columns::new(&self.cols).total_col(rdr.column_n()).parse();

        // header
        let names: Vec<String> = if self.no_header {
            col.artificial_cols_with_appended_n()
        } else {
            let Some(r) = rdr.next() else { return Ok(()) };
            if col.max >= r.len() {
                println!("[info] ignore a bad line # {r:?}!");
                col.artificial_cols_with_appended_n()
            } else {
                col.select_owned_vec_from_excel_datatype(r)
            }
        };

        // read file
        let freq = DashMap::new();
        rdr.iter().skip(rdr.next_called).for_each(|r| {
            if col.max >= r.len() {
                println!("[info] ignore a bad line # {r:?}!");
            } else {
                let r = col.select_owned_string_from_excel_datatype(r);
                *freq.entry(r).or_insert(0) += 1;
            }
        });

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
            let new_path = filename::new_path(path, "-frequency").with_extension("csv");
            file::write_frequency_to_csv(&new_path, &names, freq);
            println!("\nSaved to file: {}", new_path.display());
        } else {
            print_frequency_table(&names, freq)
        }

        Ok(())
    }
}
