use crate::utils::cli_result::CliResult;
use crate::utils::constants::COMMA;
use crate::utils::excel::datatype_vec_to_string_vec;
use crate::utils::filename::new_path;
use crate::utils::regex::Re;
use crate::utils::util::werr_exit;
use crate::utils::writer::Writer;
use calamine::{open_workbook_auto, Reader, Sheets};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

struct Args {
    sheet: usize,
    no_header: bool,
    wtr: Writer,
    re: Re,
    matched: usize,
    workbook: Sheets<BufReader<File>>,
}

pub fn run(path: &Path, sheet: &str, pattern: &str, no_header: bool, export: bool) -> CliResult {
    // wtr and rdr
    let out = new_path(path, "-searched").with_extension("csv");
    let wtr = Writer::file_or_stdout(export, &out)?;

    // regex search
    let mut args = Args {
        sheet: 0,
        no_header,
        wtr,
        re: Re::new(pattern)?,
        matched: 0,
        workbook: open_workbook_auto(path)?,
    };

    if sheet == "all" {
        args.search_all()?
    } else {
        args.parse_sheet(sheet);
        args.search_one()?
    };

    if export {
        println!("Matched rows: {}", args.matched);
        println!("Saved to file: {}", out.display());
    }

    Ok(())
}

impl Args {
    fn parse_sheet(&mut self, sheet: &str) {
        let Ok(v) = sheet.parse::<usize>() else {
            werr_exit!("{} is not a valid int.", sheet);
        };

        self.sheet = v;
    }

    fn search_one(&mut self) -> Result<(), Box<dyn Error>> {
        self.search(self.sheet as usize);
        Ok(())
    }

    fn search_all(&mut self) -> Result<(), Box<dyn Error>> {
        let sheets = self.workbook.sheet_names().to_owned();

        for (i, sheet) in sheets.iter().enumerate() {
            write!(self.wtr.0, "[{}]\n", sheet)?;
            self.search(i);
            write!(self.wtr.0, "{}\n", "")?;
        }

        Ok(())
    }

    fn search(&mut self, sheet: usize) {
        let Ok(range) = self.workbook.worksheet_range_at(sheet).unwrap_or_else(|| {
            werr_exit!("{}-th sheet does not exist.", sheet);
        }) else {
            return;
        };

        let mut rows = range.rows();

        // header
        if !self.no_header {
            let Some(r) = rows.next() else {
                return;
            };
            self.wtr.write_excel_line_unchecked(r, COMMA)
        };

        // read file
        rows.for_each(|r| {
            let r = datatype_vec_to_string_vec(r);
            if r.iter().any(|i| self.re.is_match(i)) {
                // pipeline could be closed,
                // e.g., when rsv head take enough items
                self.wtr.write_line_by_field_unchecked(&r, None);
                self.matched += 1;
            }
        });
    }
}
