use crate::{
    args::Flatten,
    utils::{cli_result::CliResult, reader::ExcelReader, table::Table},
};

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
        let n = if self.n <= 0 {
            usize::MAX
        } else {
            self.n as usize
        };
        let mut rdr = range.iter().skip(range.next_called).take(n).peekable();
        while let Some(l) = rdr.next() {
            let r = l
                .iter()
                .zip(&columns)
                .map(|(v, k)| [k.to_owned(), v.to_string()])
                .collect::<Vec<_>>();
            Table::from_records(r).print_blank()?;

            if rdr.peek().is_some() {
                println!(" {}", &self.delimiter);
            } else {
                println!();
            }
        }

        Ok(())
    }
}
