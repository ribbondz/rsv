use crate::args::Sort;
use crate::utils::cli_result::CliResult;
use crate::utils::constants::COMMA;
use crate::utils::excel::datatype_vec_to_string_vec;
use crate::utils::filename::new_path;
use crate::utils::reader::ExcelReader;
use crate::utils::sort::SortColumns;
use crate::utils::writer::Writer;

impl Sort {
    pub fn excel_run(&self) -> CliResult {
        let path = &self.path();

        // open file and count
        let mut range = ExcelReader::new(path, self.sheet)?;
        let out = new_path(path, "-sorted").with_extension("csv");
        let mut wtr = Writer::file_or_stdout(self.export, &out)?;

        // cols
        let cols = SortColumns::from(&self.cols)?;

        // header
        if !self.no_header {
            let Some(r) = range.next() else { return Ok(()) };
            wtr.write_excel_line_unchecked(r, COMMA);
        }

        // lines
        let mut lines = range
            .iter()
            .skip(range.next_called)
            .map(datatype_vec_to_string_vec)
            .collect::<Vec<_>>();

        // sort
        cols.sort_excel_and_write(&mut lines, &mut wtr)?;

        if self.export {
            println!("Saved to file: {}", out.display())
        }

        Ok(())
    }
}
