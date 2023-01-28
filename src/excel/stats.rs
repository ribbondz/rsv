use crate::utils::cli_result::CliResult;
use crate::utils::column::Columns;
use crate::utils::column_stats::ColumnStats;
use crate::utils::column_type::ColumnTypes;
use crate::utils::excel_reader::ExcelReader;
use crate::utils::filename::new_path;
use rayon::prelude::*;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

pub fn run(path: &Path, sheet: usize, no_header: bool, cols: &str, export: bool) -> CliResult {
    // read file
    let range = ExcelReader::new(path, sheet)?;
    let lines = range.iter().collect::<Vec<_>>();

    // too few lines
    if lines.len() <= 1 - no_header as usize {
        return Ok(());
    }

    // Column type
    let cols = Columns::new(cols);
    let col_type = ColumnTypes::guess_from_excel(&range, no_header, &cols).unwrap();

    // header
    let name = match (no_header, cols.all) {
        (true, _) => cols.artificial_n_cols(lines[0].len()),
        (false, true) => lines[0].iter().map(|i| i.to_string()).collect::<Vec<_>>(),
        (false, false) => cols.iter().map(|&i| lines[0][i].to_string()).collect(),
    };

    let lines = &lines[(1 - no_header as usize)..];

    // stats holder
    let mut stat = ColumnStats::new(&col_type, &name);
    let segs = lines.chunks(1000).collect::<Vec<_>>();
    let r = segs
        .into_par_iter()
        .map(|chunk| {
            let mut s = stat.clone();
            for &r in chunk {
                s.parse_excel_row(r);
            }
            s
        })
        .collect::<Vec<_>>();
    r.into_iter().fold(&mut stat, |s, b| {
        s.merge(b);
        s
    });

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
