use std::io::{BufWriter, Error, Write, stdout};

pub fn datetime_str() -> String {
    let t = chrono::offset::Local::now();
    t.format("%Y%m%d%H%M%S").to_string()
    // "".to_owned()
}

pub fn is_null(s: &str) -> bool {
    s.is_empty() || s == "NA" || s == "Na" || s == "na" || s == "NULL" || s == "Null" || s == "null"
}

pub fn get_valid_sep(sep: &str) -> Result<char, Error> {
    let cleaned_sep = sep.replace("\"", "").replace("'", "");

    if cleaned_sep == "\\t" || cleaned_sep == "t" {
        Ok('\t')
    } else if cleaned_sep == "," {
        Ok(',')
    } else if cleaned_sep.len() == 1 {
        Ok(cleaned_sep.chars().next().unwrap())
    } else {
        werr_exit!("cannot parse separator <{}>.", sep);
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

#[macro_export]
macro_rules! werr_exit {
    ($($arg:tt)*) => ({
        use std::io::Write;
        use std::process;
        (writeln!(&mut ::std::io::stderr(), $($arg)*)).unwrap();
        process::exit(1);
    });
}

pub use werr_exit;
