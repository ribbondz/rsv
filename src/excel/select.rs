use crate::args::Select;
use crate::utils::cli_result::CliResult;
use crate::utils::column::Columns;
use crate::utils::constants::COMMA;
use crate::utils::excel::datatype_vec_to_string_vec;
use crate::utils::filename::new_path;
use crate::utils::filter::Filter;
use crate::utils::reader::ExcelReader;
use crate::utils::writer::Writer;

impl Select {
    pub fn excel_run(&self) -> CliResult {
        let path = &self.path();

        // out path
        let out = new_path(path, "-selected").with_extension("csv");

        // open file
        let mut wtr = Writer::file_or_stdout(self.export, &out)?;
        let mut rdr = ExcelReader::new(path, self.sheet)?;

        // cols and filters
        let n = rdr.column_n();
        let cols = Columns::new(&self.cols).total_col(n).parse();
        let filter = Filter::new(&self.filter).total_col(n).parse();

        // header
        if !self.no_header {
            let Some(r) = rdr.next() else { return Ok(()) };
            if cols.select_all {
                wtr.write_excel_line_unchecked(r, COMMA);
            } else {
                let r = cols.iter().map(|&i| r[i].to_string()).collect::<Vec<_>>();
                wtr.write_fields_unchecked(&r, None);
            }
        }

        // read
        rdr.iter().skip(rdr.next_called).for_each(|r| {
            let r = datatype_vec_to_string_vec(r);
            if filter.excel_record_is_valid(&r) {
                match cols.select_all {
                    true => wtr.write_fields_unchecked(&r, None),
                    false => {
                        let r = cols.iter().map(|&i| &r[i]).collect::<Vec<_>>();
                        wtr.write_fields_unchecked(&r, None);
                    }
                }
            }
        });

        if self.export {
            println!("\nSaved to file: {}", out.display())
        }

        Ok(())
    }
}
