use super::date_format_infer::DateSmartParser;
use super::{cli_result::CliResult, filename::new_file, reader::ExcelReader, writer::Writer};
use crate::utils::{
    column::Columns,
    column_type::{ColumnType, ColumnTypes},
    row_split::CsvRowSplitter,
};
use rust_xlsxwriter::*;
use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write, stdin},
    path::{Path, PathBuf},
};

pub fn is_file_suffix(f: &str) -> bool {
    f == "csv" || f == "txt" || f == "tsv" || f == "xlsx" || f == "xls"
}

pub fn is_valid_plain_text(f: &str) -> bool {
    f.ends_with("csv") || f.ends_with("txt") || f.ends_with("tsv")
}

pub fn is_valid_excel(f: &str) -> bool {
    f.ends_with("xlsx") || f.ends_with("xls")
}

pub fn csv_or_io_to_csv(path: Option<&Path>, out: &str) -> CliResult {
    // out path
    let out = out_filename(out);

    // rdr and wtr
    let mut rdr = match path {
        Some(f) => Box::new(BufReader::new(File::open(f)?)) as Box<dyn BufRead>,
        None => Box::new(BufReader::new(stdin())) as Box<dyn BufRead>,
    };
    let mut wtr = BufWriter::new(File::create(&out)?);

    // copy
    let mut buf = vec![];
    while let Ok(bytes) = rdr.read_until(b'\n', &mut buf) {
        if bytes == 0 {
            break;
        }
        wtr.write_all(&buf[..bytes])?;
        buf.clear();
    }

    println!("Saved to file: {}", out.display());

    Ok(())
}

pub fn excel_to_csv(path: &Path, sheet: usize, sep: &str, out: &str) -> CliResult {
    // out path
    let out = out_filename(out);

    // rdr and wtr
    let range = ExcelReader::new(path, sheet)?;
    let mut wtr = Writer::new(Path::new(&out))?;

    // excel2csv
    let sep_bytes = sep.as_bytes();
    for r in range.iter() {
        wtr.write_excel_line(r, sep_bytes)?;
    }

    println!("Saved to file: {}", out.display());

    Ok(())
}

pub fn csv_to_excel(
    path: &Path,
    sep: char,
    quote: char,
    out: &str,
    no_header: bool,
    text_columns: &Vec<usize>,
    date_columns: &Vec<usize>,
    date_formats: &Vec<String>,
) -> CliResult {
    // rdr and wtr
    let rdr = BufReader::new(File::open(path)?);
    let mut workbook = Workbook::new();
    let mut sheet = workbook.add_worksheet();

    // column type
    let cols = Columns::new("").total_col_of(path, sep, quote).parse();
    let ctypes = match ColumnTypes::guess_from_csv(
        path,
        sep,
        quote,
        no_header,
        &cols,
        text_columns,
        date_columns,
    )? {
        Some(v) => v,
        None => return Ok(()),
    };
    ctypes.update_excel_column_width(&mut sheet)?;
    let ctypes = Some(ctypes);

    // copy
    let mut iter = rdr.lines().enumerate();
    if !no_header {
        if let Some((_, r)) = iter.next() {
            let r = r?;
            sheet.write_row(0, 0, CsvRowSplitter::new(&r, sep, quote))?;
        };
    }

    let parser = DateSmartParser::new();
    for (n, r) in iter {
        let r = r?;
        let l = CsvRowSplitter::new(&r, sep, quote).collect::<Vec<_>>();
        write_excel_line(
            &mut sheet,
            n,
            &l,
            ctypes.as_ref(),
            date_columns,
            date_formats,
            &parser,
        )?;
    }

    // out path
    let out = out_filename(out);
    workbook.save(&out)?;

    println!("Saved to file: {}", out.display());

    Ok(())
}

pub fn io_to_excel(
    sep: char,
    quote: char,
    no_header: bool,
    out: &str,
    text_columns: &Vec<usize>,
    date_columns: &Vec<usize>,
    date_formats: &Vec<String>,
) -> CliResult {
    // rdr
    let lines = stdin()
        .lock()
        .lines()
        .filter_map(|i| i.ok())
        .collect::<Vec<_>>();
    let lines = lines
        .iter()
        .map(|i| CsvRowSplitter::new(i, sep, quote).collect())
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
            &lines[(1 - no_header as usize)..],
            &cols,
            text_columns,
            date_columns,
        );
        ctypes.update_excel_column_width(&mut sheet)?;
        Some(ctypes)
    } else {
        None
    };

    let smart_parser = DateSmartParser::new();
    for (n, r) in lines.iter().enumerate() {
        write_excel_line(
            &mut sheet,
            n,
            r,
            ctypes.as_ref(),
            date_columns,
            date_formats,
            &smart_parser,
        )?;
    }

    // out path
    let out = out_filename(out);
    workbook.save(&out)?;

    println!("Saved to file: {}", out.display());

    Ok(())
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

fn out_filename(out: &str) -> PathBuf {
    let f = if is_file_suffix(out) {
        format!("export.{out}")
    } else {
        out.to_owned()
    };

    new_file(&f)
}

fn write_excel_line(
    sheet: &mut Worksheet,
    row: usize,
    line: &[&str],
    ctypes: Option<&ColumnTypes>,
    date_columns: &Vec<usize>,
    date_format: &Vec<String>,
    parser: &DateSmartParser,
) -> CliResult {
    let row = row as u32;
    let fmt = Format::new().set_num_format("yyyy-mm-dd hh:mm:ss");
    if ctypes.is_some() {
        for ((c, &v), t) in line.iter().enumerate().zip(ctypes.unwrap().iter()) {
            let col = c as u16;
            match t.col_type {
                ColumnType::Float | ColumnType::Int => match v.parse::<f64>() {
                    Ok(v) => sheet.write(row, col, v)?,
                    Err(_) => sheet.write(row, col, v)?,
                },
                ColumnType::Date => {
                    let assigned_fmt = if let Some(i) = date_columns.iter().position(|&r| r == c) {
                        date_format.get(i)
                    } else {
                        None
                    };
                    match parser.smart_parse(v, assigned_fmt) {
                        Some(dt) => sheet.write_datetime_with_format(row, col, &dt, &fmt)?,
                        None => sheet.write(row, col, v)?,
                    }
                }
                _ => sheet.write(row, col, v)?,
            };
        }
    } else {
        for (col, &v) in line.iter().enumerate() {
            sheet.write(row, col as u16, v)?;
        }
    }

    Ok(())
}
