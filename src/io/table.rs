use crate::{
    args::Table,
    utils::{cli_result::CliResult, reader::IoReader, table::Table as T, util::valid_sep},
};

impl Table {
    pub fn io_run(&self) -> CliResult {
        let sep = valid_sep(&self.sep);

        let lines = IoReader::new().lines();
        let lines = lines
            .iter()
            .map(|r| r.split(&sep).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        T::from_records(lines).print_blank()?;

        Ok(())
    }
}
