use crate::{
    args::Table,
    utils::{cli_result::CliResult, table::Table as T, util::valid_sep},
};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

impl Table {
    pub fn csv_run(&self) -> CliResult {
        let sep = valid_sep(&self.sep);

        // rdr
        let rdr = BufReader::new(File::open(&self.path())?);

        let rows = rdr
            .lines()
            .filter_map(|r| r.ok())
            .map(|r| r.split(&sep).map(String::from).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        T::from_records(rows).print_blank()?;

        Ok(())
    }
}
