use std::io::{BufRead, stdin};

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

impl To {
    pub fn io_run(&self) -> CliResult {
        let out = self.out.to_lowercase();

        match out.as_str() {
            v if is_valid_plain_text(v) => csv_or_io_to_csv(None, &out)?,
            v if is_valid_excel(v) => self.io_to_excel(out)?,
            _ => return Err(format!("output file format <{out}> is un-recognized.").into()),
        };

        Ok(())
    }

    pub fn io_to_excel(&self, out: String) -> CliResult {
        // rdr
        let lines = stdin()
            .lock()
            .lines()
            .filter_map(|i| i.ok())
            .collect::<Vec<_>>();
        let lines = lines
            .iter()
            .map(|i| CsvRowSplitter::new(i, self.sep, self.quote).collect())
            .collect::<Vec<_>>();

        if lines.is_empty() {
            return Ok(());
        }

        //  wtr
        let mut workbook = Workbook::new();
        let mut sheet = workbook.add_worksheet();
        let ctypes = if equal_width(&lines) {
            // column type
            let cols = Columns::new("").total_col(lines[0].len()).parse();
            let ctypes = ColumnTypes::guess_from_io(
                &lines[(1 - self.no_header as usize)..],
                &cols,
                &self.text_columns,
                &self.date_columns,
            );
            ctypes.update_excel_column_width(&mut sheet)?;
            Some(ctypes)
        } else {
            None
        };

        let smart_parser = DateSmartParser::new();
        let date_fmt = Format::new().set_num_format("yyyy-mm-dd");
        let datetime_fmt = Format::new().set_num_format("yyyy-mm-dd hh:mm:ss");
        for (n, r) in lines.iter().enumerate() {
            write_excel_line(
                &mut sheet,
                n,
                r,
                ctypes.as_ref(),
                &self.date_columns,
                &self.date_formats,
                self.serial_dates,
                &smart_parser,
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

fn equal_width(lines: &Vec<Vec<&str>>) -> bool {
    let width = lines[0].len();

    for row in lines.iter() {
        if width != row.len() {
            return false;
        }
    }

    true
}
