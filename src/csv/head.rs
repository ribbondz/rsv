use crate::utils::cli_result::CliResult;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use tabled::builder::Builder;
use tabled::Style;

pub fn run(path: &Path, no_header: bool, sep: &str, n: usize, tabled: bool) -> CliResult {
    // show head n
    let r = BufReader::new(File::open(path)?)
        .lines()
        .take(n + 1 - no_header as usize)
        .filter_map(|i| i.ok())
        .collect::<Vec<_>>();

    // tabled or not
    if tabled && !r.is_empty() {
        print_as_table(r, sep, no_header);
    } else {
        r.iter().for_each(|i| println!("{}", i));
    }

    Ok(())
}

pub fn print_as_table(records: Vec<String>, sep: &str, no_header: bool) {
    let mut rdr = records.iter();
    let mut builder = Builder::default();

    // header
    if !no_header {
        if let Some(row) = rdr.next() {
            let col = row.split(sep).collect::<Vec<_>>();
            builder.set_columns(col);
        }
    }

    // content
    for row in rdr {
        let r = row.split(sep).collect::<Vec<_>>();
        builder.add_record(r);
    }

    // build
    let mut table = builder.build();

    // style
    table.with(Style::blank());

    println!("{table}");
}
