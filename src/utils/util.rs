use std::io::{stdout, BufWriter, Write};

pub fn datetime_str() -> String {
    let t = chrono::offset::Local::now();
    t.format("%Y%m%d%H%M%S").to_string()
    // "".to_owned()
}

pub fn is_null(s: &str) -> bool {
    s.is_empty() || s == "NA" || s == "Na" || s == "na" || s == "NULL" || s == "Null" || s == "null"
}

pub fn is_tab(sep: &str) -> bool {
    sep == "\\t" || sep == "'\\t'" || sep == "\"\\t\""
}

pub fn valid_sep(sep: &str) -> String {
    match is_tab(sep) {
        true => '\t'.to_string(),
        false => sep.to_owned(),
    }
}

/// early return when pipeline closed
pub fn print_frequency_table(names: &[String], freq: Vec<(String, usize)>) {
    let mut wtr = BufWriter::new(stdout());

    if writeln!(wtr, "{}", names.join(",")).is_err() {
        return;
    };

    for (k, n) in &freq {
        if writeln!(wtr, "{k},{n}").is_err() {
            return;
        }
    }
}

macro_rules! werr_exit {
    ($($arg:tt)*) => ({
        use std::io::Write;
        use std::process;
        (writeln!(&mut ::std::io::stderr(), $($arg)*)).unwrap();
        process::exit(1);
    });
}

pub(crate) use werr_exit;
