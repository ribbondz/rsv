use crate::utils::excel_reader::ExcelReader;
use std::error::Error;
use std::path::Path;

pub fn run(path: &Path, sheet: usize) -> Result<(), Box<dyn Error>> {
    // open file and header
    let mut range = ExcelReader::new(path, sheet)?;

    range
        .next()
        .unwrap()
        .iter()
        .enumerate()
        .for_each(|(u, r)| println!("{:<5}{}", u, r));

    Ok(())
}
