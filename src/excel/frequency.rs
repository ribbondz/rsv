use crate::utils::cli_result::CliResult;
use crate::utils::column::Columns;
use crate::utils::file;
use crate::utils::filename;
use crate::utils::reader::ExcelReader;
use crate::utils::util::print_frequency_table;
use dashmap::DashMap;
use std::path::Path;

pub fn run(
    path: &Path,
    no_header: bool,
    sheet: usize,
    cols: &str,
    ascending: bool,
    n: i32,
    export: bool,
) -> CliResult {
    // open file and header
    let mut rdr = ExcelReader::new(path, sheet)?;

    // cols
    let col = Columns::new(cols).total_col(rdr.column_n()).parse();

    // header
    let names: Vec<String> = if no_header {
        col.artificial_cols_with_appended_n()
    } else {
        let Some(r) = rdr.next() else {
           return Ok(())
        };
        if col.max >= r.len() {
            println!("[info] ignore a bad line # {r:?}!");
            col.artificial_cols_with_appended_n()
        } else {
            col.select_owned_vec_from_excel_datatype(r)
        }
    };

    // read file
    let freq = DashMap::new();
    rdr.iter().skip(rdr.next_called).for_each(|r| {
        if col.max >= r.len() {
            println!("[info] ignore a bad line # {r:?}!");
        } else {
            let r = col.select_owned_string_from_excel_datatype(r);
            *freq.entry(r).or_insert(0) += 1;
        }
    });

    let mut freq = freq.into_iter().collect::<Vec<(_, _)>>();
    if ascending {
        freq.sort_by(|a, b| a.1.cmp(&b.1));
    } else {
        freq.sort_by(|a, b| b.1.cmp(&a.1));
    }

    // apply head n
    if n > 0 {
        freq.truncate(n as usize)
    }

    // export or print
    if export {
        let new_path = filename::new_path(path, "-frequency").with_extension("csv");
        file::write_frequency_to_csv(&new_path, &names, freq);
        println!("\nSaved to file: {}", new_path.display());
    } else {
        print_frequency_table(&names, freq)
    }

    Ok(())
}
