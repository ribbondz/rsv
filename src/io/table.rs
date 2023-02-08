use crate::utils::{cli_result::CliResult, reader::IoReader, table::Table};

pub fn run(sep: &str) -> CliResult {
    let lines = IoReader::new().lines();
    let lines = lines
        .iter()
        .map(|r| r.split(sep).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    Table::from_records(lines).print_blank()?;

    Ok(())
}
