use crate::utils::{cli_result::CliResult, excel_reader::ExcelReader, util::print_table};
use std::path::Path;

pub fn run(path: &Path, no_header: bool, sheet: usize, delimiter: &str, n: i32) -> CliResult {
    // open file and header
    let mut range = ExcelReader::new(path, sheet)?;

    // columns
    let columns: Vec<String> = if no_header {
        let column_n = range.column_n();
        (1..=column_n)
            .map(|i| "col".to_owned() + &i.to_string())
            .collect::<Vec<_>>()
    } else {
        let first_row = match range.next() {
            Some(v) => v,
            None => return Ok(()),
        };
        first_row.iter().map(|i| i.to_string()).collect::<Vec<_>>()
    };

    // read file
    let n = if n <= 0 { usize::MAX } else { n as usize };
    let mut rdr = range.iter().skip(range.next_called).take(n).peekable();
    while let Some(l) = rdr.next() {
        let r = l
            .iter()
            .zip(&columns)
            .map(|(v, k)| vec![k.to_owned(), v.to_string()])
            .collect::<Vec<_>>();
        print_table(r);

        if rdr.peek().is_some() {
            println!(" {delimiter}");
        } else {
            println!();
        }
    }

    Ok(())
}
