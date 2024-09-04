use crate::{
    args::Table,
    utils::{cli_result::CliResult, reader::IoReader, table::Table as T},
};

impl Table {
    pub fn io_run(&self) -> CliResult {
        let lines = IoReader::new().lines();
        let lines = lines
            .iter()
            .map(|r| self.split_row_to_vec(r))
            .collect::<Vec<_>>();

        T::from_records(lines).print_blank()?;

        Ok(())
    }
}
