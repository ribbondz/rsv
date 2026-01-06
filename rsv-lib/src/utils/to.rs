use super::date_format_infer::DateSmartParser;
use super::{cli_result::CliResult, filename::new_file};
use crate::utils::column_type::{ColumnType, ColumnTypes};
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

pub fn out_filename(out: &str) -> PathBuf {
    let f = if is_file_suffix(out) {
        format!("export.{out}")
    } else {
        out.to_owned()
    };

    new_file(&f)
}

#[allow(clippy::too_many_arguments)]
pub fn write_excel_line(
    sheet: &mut Worksheet,
    row: usize,
    line: &[&str],
    ctypes: Option<&ColumnTypes>,
    date_columns: &[usize],
    date_formats: &[String],
    serial_dates: bool,
    parser: &DateSmartParser,
    date_fmt: &Format,
    datetime_fmt: &Format,
) -> CliResult {
    let row = row as u32;
    if let Some(ctypes) = ctypes {
        for ((c, &v), t) in line.iter().enumerate().zip(ctypes.iter()) {
            let col = c as u16;
            match t.col_type {
                ColumnType::Float | ColumnType::Int => match v.parse::<f64>() {
                    Ok(v) => sheet.write(row, col, v)?,
                    Err(_) => sheet.write(row, col, v)?,
                },
                ColumnType::Date => {
                    let assigned_fmt = match date_formats {
                        [] => None,
                        [fmt] => Some(fmt),
                        _ => {
                            if let Some(i) = date_columns.iter().position(|&r| r == c) {
                                date_formats.get(i)
                            } else {
                                None
                            }
                        }
                    };
                    if let Some(dt) = parser.smart_parse(v, assigned_fmt) {
                        if serial_dates {
                            sheet.write_datetime(row, col, dt)?
                        } else {
                            sheet.write_datetime_with_format(
                                row,
                                col,
                                dt,
                                if v.contains(":") {
                                    datetime_fmt
                                } else {
                                    date_fmt
                                },
                            )?
                        }
                    } else {
                        sheet.write(row, col, v)?
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
