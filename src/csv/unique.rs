use crate::args::Unique;
use crate::utils::cli_result::CliResult;
use crate::utils::column::Columns;
use crate::utils::filename::new_path;
use crate::utils::writer::Writer;
use ahash::HashMapExt;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::Path;

impl Unique {
    pub fn csv_run(&self) -> CliResult {
        let path = &self.path();
        let all_cols = self.cols == "-1";

        // cols
        let cols = if all_cols {
            None
        } else {
            Some(
                Columns::new(&self.cols)
                    .total_col_of(path, self.sep, self.quote)
                    .parse(),
            )
        };

        // wtr and rdr
        let out = new_path(path, "-drop-duplicates");
        let mut wtr = Writer::file_or_stdout(self.export, &out)?;
        let mut rdr = BufReader::new(File::open(path)?).lines();

        // header
        if !self.no_header {
            let Some(r) = rdr.next() else { return Ok(()) };
            wtr.write_str_unchecked(&r?)
        }

        // read
        match (self.keep_last, all_cols) {
            (true, true) => keep_last_and_all_cols(&mut rdr, &mut wtr, path, self.no_header)?,
            (true, false) => {
                keep_last_and_partial_cols(self, &mut rdr, &mut wtr, cols.unwrap(), path)?
            }
            (false, true) => keep_first_and_all_cols(&mut rdr, &mut wtr)?,
            (false, false) => keep_first_and_partial_cols(&mut rdr, &mut wtr, cols.unwrap(), self)?,
        }

        if self.export {
            println!("\nSaved to file: {}", out.display())
        }

        Ok(())
    }
}

fn keep_first_and_all_cols(rdr: &mut Lines<BufReader<File>>, wtr: &mut Writer) -> CliResult {
    let mut unique_holder = ahash::HashSet::default();
    for r in rdr {
        let r = r?;
        if !unique_holder.contains(&r) {
            wtr.write_str_unchecked(&r);
            unique_holder.insert(r);
        }
    }

    Ok(())
}

fn keep_first_and_partial_cols(
    rdr: &mut Lines<BufReader<File>>,
    wtr: &mut Writer,
    cols: Columns,
    args: &Unique,
) -> CliResult {
    let mut unique_holder = ahash::HashSet::default();
    for r in rdr {
        let r = r?;
        let segs = args.split_row_to_vec(&r);
        let p = cols.select_owned_string(&segs);
        if !unique_holder.contains(&p) {
            wtr.write_str_unchecked(&r);
            unique_holder.insert(p);
        }
    }

    Ok(())
}

fn keep_last_and_all_cols(
    rdr: &mut Lines<BufReader<File>>,
    wtr: &mut Writer,
    path: &Path,
    no_header: bool,
) -> CliResult {
    let mut unique_n = ahash::HashMap::default();

    // first scan to locate record location
    let rdr2 = BufReader::new(File::open(path)?).lines();
    for r in rdr2.skip(1 - (no_header as usize)) {
        let r = r?;
        *unique_n.entry(r).or_insert(0) += 1;
    }

    // second scan
    for r in rdr {
        let r = r?;
        if unique_n[&r] == 1 {
            wtr.write_str_unchecked(&r);
        } else {
            *unique_n.entry(r).or_insert(0) -= 1;
        }
    }

    Ok(())
}

fn keep_last_and_partial_cols(
    args: &Unique,
    rdr: &mut Lines<BufReader<File>>,
    wtr: &mut Writer,
    cols: Columns,
    path: &Path,
) -> CliResult {
    let mut unique_n = ahash::HashMap::new();

    // first scan to locate record location
    let rdr2 = BufReader::new(File::open(path)?).lines();
    for r in rdr2.skip(1 - (args.no_header as usize)) {
        let r = r?;
        let segs = args.split_row_to_vec(&r);
        let p = cols.select_owned_string(&segs);
        *unique_n.entry(p).or_insert(0) += 1;
    }

    // second scan
    for r in rdr {
        let r = r?;
        let segs = args.split_row_to_vec(&r);
        let p = cols.select_owned_string(&segs);
        if unique_n[&p] == 1 {
            wtr.write_str_unchecked(&r);
        } else {
            *unique_n.entry(p).or_insert(0) -= 1;
        }
    }

    Ok(())
}
