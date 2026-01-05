use crate::args::Table;
use rsv_lib::utils::{cli_result::CliResult, table::Table as T};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

impl Table {
    pub fn csv_run(&self) -> CliResult {
        // rdr
        let rdr = BufReader::new(File::open(self.path())?);

        let rows = rdr
            .lines()
            .filter_map(|r| r.ok())
            .map(|r| self.split_row_to_owned_vec(&r))
            .collect::<Vec<_>>();

        T::from_records(rows).print_blank()?;

        Ok(())
    }
}
