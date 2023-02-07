use crate::utils::cli_result::CliResult;
use crate::utils::column::Columns;
use crate::utils::file;
use crate::utils::filename::new_file;
use crate::utils::util::print_frequency_table;
use dashmap::DashMap;
use rayon::prelude::*;
use std::io::{stdin, BufRead};

pub fn run(
    no_header: bool,
    sep: &str,
    cols: &str,
    ascending: bool,
    top_n: i32,
    export: bool,
) -> CliResult {
    // cols
    let col = Columns::new(cols);

    // open file and header
    let mut rdr = stdin().lock().lines();
    let names: Vec<String> = if no_header {
        col.artificial_cols_with_appended_n()
    } else {
        let  Some(r) = rdr.next() else {
            return Ok(())
        };
        let r = r?;
        let r = r.split(sep).collect::<Vec<_>>();
        if col.max() >= r.len() {
            println!("[info] ignore a bad line # {r:?}!");
            col.artificial_cols_with_appended_n()
        } else {
            col.select_owned_vector_and_append_n(&r)
        }
    };

    let freq = DashMap::new();
    let mut n = 0;
    let buffer = 1000;
    let mut lines = Vec::with_capacity(buffer);

    for r in rdr {
        n += 1;
        lines.push(r?);
        if n >= buffer {
            task_handle(lines, &freq, &col, sep);
            lines = Vec::with_capacity(buffer);
            n = 0;
        }
    }

    if !lines.is_empty() {
        task_handle(lines, &freq, &col, sep);
    }

    let mut freq = freq.into_iter().collect::<Vec<(_, _)>>();
    if ascending {
        freq.sort_by(|a, b| a.1.cmp(&b.1));
    } else {
        freq.sort_by(|a, b| b.1.cmp(&a.1));
    }

    // apply head n
    if top_n > 0 {
        freq = freq.into_iter().take(top_n as usize).collect()
    }

    // export or print
    if export {
        let out = new_file("frequency.csv");
        file::write_frequency_to_csv(&out, &names, freq);
        println!("Saved to file: {}", out.display());
    } else {
        print_frequency_table(&names, freq)
    }

    Ok(())
}

fn task_handle(lines: Vec<String>, freq: &DashMap<String, usize>, col: &Columns, sep: &str) {
    lines.par_iter().for_each(|r| {
        let r = r.split(sep).collect::<Vec<_>>();
        if col.max() >= r.len() {
            println!("[info] ignore a bad line # {r:?}!");
        } else {
            let r = col.select_owned_string(&r);
            *freq.entry(r).or_insert(0) += 1;
        }
    });
}
