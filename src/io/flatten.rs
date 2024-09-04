use crate::{
    args::Flatten,
    utils::{cli_result::CliResult, reader::IoReader, row_split::CsvRow, table::Table},
};

impl Flatten {
    pub fn io_run(&self) -> CliResult {
        let n = if self.n <= 0 {
            usize::MAX - 10
        } else {
            self.n as usize
        };

        // open file and header
        let lines = IoReader::new().no_header(self.no_header).top_n(n).lines();

        // too few rows
        if lines.len() <= 1 - self.no_header as usize {
            return Ok(());
        }

        let columns: Vec<String> = if self.no_header {
            (1..=self.row_field_count(&lines[0]))
                .map(|i| format!("col{i}"))
                .collect::<Vec<_>>()
        } else {
            self.split_row_to_owned_vec(&lines[0])
        };

        // read file
        lines
            .iter()
            .skip(1 - self.no_header as usize)
            .enumerate()
            .for_each(|(i, l)| {
                println!(" {}row{}", &self.delimiter, i + 1);
                let r = CsvRow::new(l)
                    .split(self.sep, self.quote)
                    .zip(&columns)
                    .map(|(v, k)| [k.as_str(), v])
                    .collect::<Vec<_>>();
                Table::from_records(r).print_blank().unwrap();
            });

        Ok(())
    }
}
