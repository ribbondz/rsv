use crate::utils::cli_result::CliResult;
use crate::utils::column::Columns;
use crate::utils::column_stats::ColumnStats;
use crate::utils::column_type::{ColumnType, ColumnTypes};
use crate::utils::filename::new_file;
use crate::utils::util::is_null;

use rayon::prelude::*;
use std::fs::File;
use std::io::{self, BufRead, BufWriter, Write};

pub fn run(sep: &str, no_header: bool, cols: &str, export: bool) -> CliResult {
    // read
    let rows = io::stdin()
        .lock()
        .lines()
        .filter_map(|r| r.ok())
        .collect::<Vec<_>>();

    // too few rows
    if rows.len() <= 1 - no_header as usize {
        return Ok(());
    }

    // cols
    let cols = Columns::new(cols);

    // header
    let names = match no_header {
        true => cols.artificial_n_cols(rows[0].split(sep).count()),
        false => rows[0].split(sep).map(|i| i.to_owned()).collect::<Vec<_>>(),
    };
    let names = match cols.all {
        true => names,
        false => cols
            .iter()
            .map(|&i| names[i].to_owned())
            .collect::<Vec<_>>(),
    };

    // split rows and select
    let row_with_selected_cols = rows
        .par_iter()
        .skip(1 - no_header as usize)
        .map(|r| match cols.all {
            true => r.split(sep).map(|i| i.to_owned()).collect::<Vec<_>>(),
            false => {
                let fields = r.split(sep).collect::<Vec<_>>();
                cols.iter()
                    .map(|&i| fields[i].to_owned())
                    .collect::<Vec<_>>()
            }
        })
        .collect::<Vec<_>>();

    // column type
    let typ = ColumnTypes::from(col_type(&row_with_selected_cols))?;

    // stats holder
    let mut stat = ColumnStats::new(&typ, &names);
    row_with_selected_cols.iter().for_each(|r| {
        r.par_iter()
            .zip(&mut stat.stat)
            .for_each(|(v, c)| c.parse(v))
    });

    stat.rows += row_with_selected_cols.len();
    stat.cal_unique_and_mean();

    if export {
        let out = new_file("stats.csv");
        let mut wtr = BufWriter::new(File::create(&out)?);
        wtr.write_all(stat.to_string().as_bytes())?;
        println!("Saved to file: {}", out.display());
    } else {
        stat.print();
        println!("Total rows: {}", stat.rows);
    }

    Ok(())
}

fn col_type(v: &[Vec<String>]) -> Vec<ColumnType> {
    assert!(!v.is_empty());

    (0..v[0].len())
        .into_par_iter()
        .map(|n| parse_col_type_at(n, v))
        .collect::<Vec<_>>()
}

fn parse_col_type_at(n: usize, v: &[Vec<String>]) -> ColumnType {
    let mut ctype = ColumnType::Null;
    for r in v {
        if ctype.is_string() {
            break;
        }
        let f = &r[n];
        if is_null(f) {
            continue;
        }
        ctype.update(f);
    }

    ctype
}
