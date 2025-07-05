use crate::args::Flatten;
use rsv_lib::utils::{cli_result::CliResult, reader::ExcelReader, table::Table};

impl Flatten {
    pub fn excel_run(&self) -> CliResult {
        let path = &self.path();

        // open file and header
        let mut range = ExcelReader::new(path, self.sheet)?;

        // columns
        let columns: Vec<String> = if self.no_header {
            (1..=range.column_n())
                .map(|i| format!("col{i}"))
                .collect::<Vec<_>>()
        } else {
            let Some(r) = range.next() else { return Ok(()) };
            r.iter().map(|i| i.to_string()).collect::<Vec<_>>()
        };

        // read file
        let n = self.n as usize; // overflow is allowed when self.n is negative.
        range
            .iter()
            .skip(range.next_called)
            .take(n)
            .enumerate()
            .for_each(|(i, l)| {
                println!(" {}row{}", &self.delimiter, i + 1);
                let r = l
                    .iter()
                    .zip(&columns)
                    .map(|(v, k)| [k.to_owned(), v.to_string()])
                    .collect::<Vec<_>>();
                Table::from_records(r).print_blank().unwrap();
            });

        Ok(())
    }
}
