use crate::args::Unique;
use rsv_lib::utils::cli_result::CliResult;
use rsv_lib::utils::column::Columns;
use rsv_lib::utils::filename::new_file;
use rsv_lib::utils::reader::IoReader;
use rsv_lib::utils::writer::Writer;

impl Unique {
    pub fn io_run(&self) -> CliResult {
        let all_cols = self.cols == "-1";

        // wtr and rdr
        let out = new_file("drop_duplicates.csv");
        let mut wtr = Writer::file_or_stdout(self.export, &out)?;
        let lines = IoReader::new().no_header(self.no_header).lines();

        if lines.is_empty() {
            return Ok(());
        }

        // cols
        let cols = if all_cols {
            None
        } else {
            let n = self.row_field_count(&lines[0]);
            Some(Columns::new(&self.cols).total_col(n).parse())
        };

        // header
        if !self.no_header {
            wtr.write_str_unchecked(&lines[0]);
        }

        let lines = if self.no_header {
            &lines[..]
        } else {
            &lines[1..]
        };

        // read
        match (self.keep_last, all_cols) {
            (true, true) => keep_last_and_all_cols(lines, &mut wtr)?,
            (true, false) => keep_last_and_partial_cols(self, lines, &mut wtr, cols.unwrap())?,
            (false, true) => keep_first_and_all_cols(lines, &mut wtr)?,
            (false, false) => keep_first_and_partial_cols(self, lines, &mut wtr, cols.unwrap())?,
        };

        if self.export {
            println!("\nSaved to file: {}", out.display())
        }

        Ok(())
    }
}

fn keep_first_and_all_cols(rdr: &[String], wtr: &mut Writer) -> CliResult {
    let mut unique_holder = ahash::HashSet::default();
    for r in rdr {
        if !unique_holder.contains(r) {
            wtr.write_str_unchecked(r);
            unique_holder.insert(r);
        }
    }

    Ok(())
}

fn keep_first_and_partial_cols(
    args: &Unique,
    rdr: &[String],
    wtr: &mut Writer,
    cols: Columns,
) -> CliResult {
    let mut unique_holder = ahash::HashSet::default();
    for r in rdr {
        let segs = args.split_row_to_vec(r);
        let p = cols.select_owned_string(&segs);
        if !unique_holder.contains(&p) {
            wtr.write_str_unchecked(r);
            unique_holder.insert(p);
        }
    }

    Ok(())
}

fn keep_last_and_all_cols(rdr: &[String], wtr: &mut Writer) -> CliResult {
    let mut unique_n = ahash::HashMap::default();

    // first scan to locate record location
    for r in rdr {
        *unique_n.entry(r).or_insert(0) += 1;
    }

    // second scan
    for r in rdr {
        if unique_n[&r] == 1 {
            wtr.write_str_unchecked(r);
        } else {
            *unique_n.entry(r).or_insert(0) -= 1;
        }
    }

    Ok(())
}

fn keep_last_and_partial_cols(
    args: &Unique,
    rdr: &[String],
    wtr: &mut Writer,
    cols: Columns,
) -> CliResult {
    let mut unique_n = ahash::HashMap::default();

    // first scan to locate record location
    for r in rdr {
        let segs = args.split_row_to_vec(r);
        let p = cols.select_owned_string(&segs);
        *unique_n.entry(p).or_insert(0) += 1;
    }

    // second scan
    for r in rdr {
        let segs = args.split_row_to_vec(r);
        let p = cols.select_owned_string(&segs);
        if unique_n[&p] == 1 {
            wtr.write_str_unchecked(r);
        } else {
            *unique_n.entry(p).or_insert(0) -= 1;
        }
    }

    Ok(())
}
