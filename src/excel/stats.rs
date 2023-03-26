use crate::utils::cli_result::CliResult;
use crate::utils::column::Columns;
use crate::utils::column_stats::ColumnStats;
use crate::utils::column_type::ColumnTypes;
use crate::utils::filename::new_path;
use crate::utils::reader::ExcelReader;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

pub fn run(path: &Path, sheet: usize, no_header: bool, cols: &str, export: bool) -> CliResult {
    // read file
    let mut rdr = ExcelReader::new(path, sheet)?;

    // Column type
    let cols = Columns::new(cols).total_col(rdr.column_n()).parse();
    let col_type = ColumnTypes::guess_from_excel(&rdr, no_header, &cols).unwrap();

    // header
    let name = match no_header {
        true => cols.artificial_n_cols(rdr.column_n()),
        false => {
            let Some(r) = rdr.next() else {
                return Ok(())
            };
            r.iter().map(|i| i.to_string()).collect::<Vec<_>>()
        }
    };

    // stats holder
    let mut stat = ColumnStats::new(&col_type, &name);

    // read
    rdr.iter()
        .skip(rdr.next_called)
        .for_each(|r| stat.parse_excel_row(r));

    // refine result
    stat.cal_unique_and_mean();

    // print
    if export {
        let out = new_path(path, "-stats").with_extension("csv");
        let mut wtr = BufWriter::new(File::create(&out)?);
        wtr.write_all(stat.to_string().as_bytes())?;
        println!("Saved to file: {}", out.display());
    } else {
        stat.print();
    }

    println!("Total rows: {}", stat.rows);

    Ok(())
}
