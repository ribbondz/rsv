use crate::utils::cli_result::CliResult;
use crate::utils::column::Columns;
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

struct Args<'a> {
    sheet: usize,
    cols_raw: &'a str,
    cols: Columns<'a>,
    filter_raw: &'a str,
    filter: Columns<'a>,
    no_header: bool,
    wtr: Writer,
    re: Re,
    matched: usize,
    workbook: Sheets<BufReader<File>>,
}

pub fn run(
    path: &Path,
    filter: &str,
    cols: &str,
    sheet: &str,
    pattern: &str,
    no_header: bool,
    export: bool,
) -> CliResult {
    // wtr and rdr
    let out = new_path(path, "-searched").with_extension("csv");
    let wtr = Writer::file_or_stdout(export, &out)?;

    // regex search
    let mut args = Args {
        sheet: 0,
        cols_raw: cols,
        cols: Columns::new(cols),
        filter_raw: filter,
        filter: Columns::new(filter),
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

impl<'a> Args<'a> {
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

        let n = range.get_size().1;
        self.cols = Columns::new(self.cols_raw).total_col(n).parse();
        self.filter = Columns::new(self.filter_raw).total_col(n).parse();

        let mut rows = range.rows();

        // header
        if !self.no_header {
            let Some(r) = rows.next() else {
                return;
            };
            if self.cols.select_all {
                self.wtr.write_excel_line_unchecked(r, COMMA);
            } else {
                self.wtr
                    .write_excel_line_by_selected_fields_unchecked(&r, &self.cols.cols, COMMA);
            }
        };

        // read file
        rows.for_each(|r| {
            let r = datatype_vec_to_string_vec(r);
            match (self.cols.select_all, self.filter.select_all) {
                (true, true) => {
                    if r.iter().any(|i| self.re.is_match(i)) {
                        self.wtr.write_line_by_field_unchecked(&r, None);
                        self.matched += 1;
                    }
                }
                (true, false) => {
                    if self.filter.iter().any(|&i| self.re.is_match(&r[i])) {
                        self.wtr.write_line_by_field_unchecked(&r, None);
                        self.matched += 1;
                    }
                }
                (false, true) => {
                    if r.iter().any(|i| self.re.is_match(i)) {
                        self.wtr
                            .write_line_by_selected_field_unchecked(&r, &self.cols.cols, None);
                        self.matched += 1;
                    }
                }
                (false, false) => {
                    if self.filter.iter().any(|&i| self.re.is_match(&r[i])) {
                        self.wtr
                            .write_line_by_selected_field_unchecked(&r, &self.cols.cols, None);
                        self.matched += 1;
                    }
                }
            }
        })
    }
}
