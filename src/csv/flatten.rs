use crate::args::Flatten;
use crate::utils::cli_result::CliResult;
use crate::utils::file;
use crate::utils::row_split::CsvRow;
use crate::utils::table::Table;
use std::fs::File;
use std::io::{BufRead, BufReader};

impl Flatten {
    pub fn csv_run(&self) -> CliResult {
        let path = &self.path();

        // open file and header
        let mut rdr = BufReader::new(File::open(path)?).lines();

        // header
        let columns: Vec<String> = if self.no_header {
            match file::column_n(path, self.sep, self.quote)? {
                Some(n) => (1..=n).map(|i| format!("col{i}")).collect::<Vec<_>>(),
                None => return Ok(()),
            }
        } else {
            match rdr.next() {
                Some(r) => self.split_row_to_owned_vec(&r?),
                None => return Ok(()),
            }
        };
        println!("{:?}", &columns);

        // read file
        let n = self.n as usize; // overflow is allowed when self.n is negative.
        rdr.take(n).flatten().enumerate().for_each(|(i, l)| {
            println!(" {} row{}", &self.delimiter, i + 1);

            let r = CsvRow::new(&l)
                .split(self.sep, self.quote)
                .zip(&columns)
                .map(|(v, k)| [k.as_str(), v])
                .collect::<Vec<_>>();
            Table::from_records(r).print_blank().unwrap();
        });

        Ok(())
    }
}
