use crate::utils::{cli_result::CliResult, reader::ExcelReader, table::Table};
use std::path::Path;

pub fn run(path: &Path, no_header: bool, sheet: usize, delimiter: &str, n: i32) -> CliResult {
    // open file and header
    let mut range = ExcelReader::new(path, sheet)?;

    // columns
    let columns: Vec<String> = if no_header {
        (1..=range.column_n())
            .map(|i| format!("col{i}"))
            .collect::<Vec<_>>()
    } else {
        let Some(r) = range.next() else {
           return Ok(())
        };
        r.iter().map(|i| i.to_string()).collect::<Vec<_>>()
    };

    // read file
    let n = if n <= 0 { usize::MAX } else { n as usize };
    let mut rdr = range.iter().skip(range.next_called).take(n).peekable();
    while let Some(l) = rdr.next() {
        let r = l
            .iter()
            .zip(&columns)
            .map(|(v, k)| [k.to_owned(), v.to_string()])
            .collect::<Vec<_>>();
        Table::from_records(r).print_blank()?;

        if rdr.peek().is_some() {
            println!(" {delimiter}");
        } else {
            println!();
        }
    }

    Ok(())
}
