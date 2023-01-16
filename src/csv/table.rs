use crate::utils::{
    excel_reader::ExcelReader, file::is_excel, filename::full_path, util::print_table,
};
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

pub fn run(filename: &Option<String>, sheet: usize, sep: &str) -> Result<(), Box<dyn Error>> {
    match filename {
        Some(f) => {
            let path = full_path(f);

            if is_excel(&path) {
                print_excel_file(&path, sheet)?
            } else {
                print_csv_file(&path, sep)?
            }
        }
        None => print_cmd_output(sep)?,
    }

    Ok(())
}

fn print_cmd_output(sep: &str) -> Result<(), Box<dyn Error>> {
    let mut rows = vec![];

    for l in io::stdin().lock().lines() {
        let l = l?.split(sep).map(|i| i.to_owned()).collect::<Vec<_>>();
        rows.push(l);
    }

    print_table(rows);

    Ok(())
}

fn print_csv_file(path: &Path, sep: &str) -> Result<(), Box<dyn Error>> {
    // rdr
    let rdr = BufReader::new(File::open(path)?);

    let rows = rdr
        .lines()
        .into_iter()
        .filter_map(|r| r.ok())
        .map(|r| r.split(sep).map(|i| i.to_owned()).collect::<Vec<_>>())
        .collect();

    print_table(rows);

    Ok(())
}

fn print_excel_file(path: &Path, sheet: usize) -> Result<(), Box<dyn Error>> {
    // rdr
    let range = ExcelReader::new(path, sheet)?;

    let rows = range
        .iter()
        .map(|r| r.iter().map(|i| i.to_string()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    print_table(rows);

    Ok(())
}
