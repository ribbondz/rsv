use chrono;
use tabled::builder::Builder;
use tabled::Style;

pub fn datetime_str() -> String {
    let t = chrono::offset::Local::now();
    t.format("%Y%m%d%H%M%S").to_string()
}

pub fn is_null(s: &str) -> bool {
    s.is_empty() || s == "NA" || s == "Na" || s == "na" || s == "NULL" || s == "Null" || s == "null"
}

pub fn print_table(records: Vec<Vec<String>>) {
    let mut builder = Builder::default();

    records.iter().for_each(|r| {
        builder.add_record(r);
    });

    // build
    let mut table = builder.build();

    // style
    table.with(Style::blank());

    println!("{table}");
}

pub fn print_frequency_table(names: &Vec<String>, freq: Vec<(String, i32)>) {
    let mut builder = Builder::default();

    // header
    if !names.is_empty() {
        builder.set_columns(names);
    }

    // content
    for (key, n) in freq {
        let r = key
            .split(',')
            .map(|i| i.to_owned())
            .chain(std::iter::once(n.to_string()))
            .collect::<Vec<_>>();
        builder.add_record(r);
    }

    // build
    let mut table = builder.build();

    // style
    table.with(Style::blank());

    println!("{table}");
}

macro_rules! werr {
    ($($arg:tt)*) => ({
        use std::io::Write;
        (writeln!(&mut ::std::io::stderr(), $($arg)*)).unwrap();
    });
}

pub(crate) use werr;
