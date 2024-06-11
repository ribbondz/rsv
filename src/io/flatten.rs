use crate::{
    args::Flatten,
    utils::{cli_result::CliResult, reader::IoReader, table::Table, util::valid_sep},
};

impl Flatten {
    pub fn io_run(&self) -> CliResult {
        let sep = valid_sep(&self.sep);
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
            (1..=lines[0].split(&sep).count())
                .map(|i| format!("col{i}"))
                .collect::<Vec<_>>()
        } else {
            lines[0].split(&sep).map(String::from).collect::<Vec<_>>()
        };

        // read file
        let mut rdr = lines.iter().skip(1 - self.no_header as usize).peekable();
        while let Some(l) = rdr.next() {
            let r = l
                .split(&sep)
                .zip(&columns)
                .map(|(v, k)| [k.as_str(), v])
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
