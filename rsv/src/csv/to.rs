use crate::args::To;
use rsv_lib::utils::cli_result::CliResult;
use rsv_lib::utils::column::Columns;
use rsv_lib::utils::column_type::ColumnTypes;
use rsv_lib::utils::date_format_infer::DateSmartParser;
use rsv_lib::utils::row_split::CsvRowSplitter;
use rsv_lib::utils::to::{
    csv_or_io_to_csv, is_valid_excel, is_valid_plain_text, out_filename, write_excel_line,
};
use rust_xlsxwriter::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

impl To {
    pub fn csv_run(&self) -> CliResult {
        let path = &self.path();
        let out = self.out.to_lowercase();

        match out.as_str() {
            v if is_valid_plain_text(v) => csv_or_io_to_csv(Some(path), &out)?,
            v if is_valid_excel(v) => self.csv_to_excel(path, out)?,
            _ => return Err(format!("output file format <{out}> is un-recognized.").into()),
        };

        Ok(())
    }

    pub fn csv_to_excel(&self, path: &PathBuf, out: String) -> CliResult {
        // rdr and wtr
        let rdr = BufReader::new(File::open(path)?);
        let mut workbook = Workbook::new();
        let sheet = workbook.add_worksheet();

        // column type
        let cols = Columns::new("")
            .total_col_of(path, self.sep, self.quote)
            .parse();
        let ctypes = match ColumnTypes::guess_from_csv(
            path,
            self.sep,
            self.quote,
            self.no_header,
            &cols,
            &self.text_columns,
            &self.date_columns,
        )? {
            Some(v) => v,
            None => return Ok(()),
        };
        ctypes.update_excel_column_width(sheet)?;
        let ctypes = Some(ctypes);

        // copy
        let mut iter = rdr.lines().enumerate();
        if !self.no_header
            && let Some((_, r)) = iter.next()
        {
            let r = r?;
            sheet.write_row(0, 0, CsvRowSplitter::new(&r, self.sep, self.quote))?;
        };

        let parser = DateSmartParser::new();
        let date_fmt = Format::new().set_num_format("yyyy-mm-dd");
        let datetime_fmt = Format::new().set_num_format("yyyy-mm-dd hh:mm:ss");
        for (n, r) in iter {
            let r = r?;
            let l = CsvRowSplitter::new(&r, self.sep, self.quote).collect::<Vec<_>>();
            write_excel_line(
                sheet,
                n,
                &l,
                ctypes.as_ref(),
                &self.date_columns,
                &self.date_formats,
                self.serial_dates,
                &parser,
                &date_fmt,
                &datetime_fmt,
            )?;
        }

        // out path
        let out = out_filename(&out);
        workbook.save(&out)?;

        println!("Saved to file: {}", out.display());

        Ok(())
    }
}
