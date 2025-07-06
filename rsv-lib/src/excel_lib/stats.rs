use crate::utils::column::Columns;
use crate::utils::column_stats::{CStat, ColumnStats};
use crate::utils::column_type::ColumnTypes;
use crate::utils::reader::ExcelReader;
use crate::utils::return_result::{CliResultData, ResultData};
use std::path::PathBuf;

pub fn excel_stats(file: &PathBuf, no_header: bool, cols: String, sheet: usize) -> CliResultData {
    let mut result_data = ResultData::new();

    // read file
    let mut rdr = ExcelReader::new(file, sheet)?;

    // Column type
    let cols = Columns::new(cols.as_str())
        .total_col(rdr.column_n())
        .parse();
    let col_type = ColumnTypes::guess_from_excel(&rdr, no_header, &cols).unwrap();

    // header
    let name = match no_header {
        true => cols.artificial_n_cols(rdr.column_n()),
        false => {
            let Some(r) = rdr.next() else {
                return Ok(Some(result_data));
            };
            r.iter().map(|i| i.to_string()).collect::<Vec<_>>()
        }
    };
    result_data.insert_header(CStat::get_fields.iter().map(|s| s.to_string()).collect());

    // stats holder
    let mut stat = ColumnStats::new(&col_type, &name);
    println!("stat: {}", stat);

    // read
    rdr.iter()
        .skip(rdr.next_called)
        .for_each(|r| stat.parse_excel_row(r));

    // refine result
    stat.cal_unique_and_mean();
    result_data.insert_records(stat.stat.iter().map(|s| s.get_fields_values()));
    // print
    // stat.print();

    println!("Total rows: {}", stat.rows);

    Ok(Some(result_data))
}
