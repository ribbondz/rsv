use crate::args::Table;
use crate::utils::excel::datatype_vec_to_string_vec;
use crate::utils::{cli_result::CliResult, reader::ExcelReader, table::Table as T};

impl Table {
    pub fn excel_run(&self) -> CliResult {
        // rdr
        let range = ExcelReader::new(&self.path(), self.sheet)?;

        let rows = range
            .iter()
            .map(datatype_vec_to_string_vec)
            .collect::<Vec<_>>();

        T::from_records(rows).print_blank()?;

        Ok(())
    }
}
