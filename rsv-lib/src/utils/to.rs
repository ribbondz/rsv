use super::{cli_result::CliResult, filename::new_file, reader::ExcelReader, writer::Writer};
use crate::utils::{column::Columns, column_type::ColumnTypes, row_split::CsvRowSplitter};
use std::{
    fs::File,
    io::{stdin, BufRead, BufReader, BufWriter, Write},
    path::{Path, PathBuf},
};
use xlsxwriter::{Workbook, Worksheet};

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

pub fn csv_to_excel(path: &Path, sep: char, quote: char, out: &str, no_header: bool) -> CliResult {
    // out path
    let out = out_filename(out);

    // rdr and wtr
    let rdr = BufReader::new(File::open(path)?);
    let workbook = Workbook::new(out.to_str().unwrap())?;
    let mut sheet = workbook.add_worksheet(None)?;

    // column type
    let cols = Columns::new("").total_col_of(path, sep, quote).parse();
    let ctypes = match ColumnTypes::guess_from_csv(path, sep, quote, no_header, &cols)? {
        Some(v) => v,
        None => return Ok(()),
    };
    ctypes.update_excel_column_width(&mut sheet)?;
    let ctypes = Some(ctypes);

    // copy
    for (n, r) in rdr.lines().enumerate() {
        let r = r?;
        let l = CsvRowSplitter::new(&r, sep, quote).collect::<Vec<_>>();
        write_excel_line(&mut sheet, n, &l, ctypes.as_ref())?;
    }

    println!("Saved to file: {}", out.display());

    Ok(())
}

pub fn io_to_excel(sep: char, quote: char, no_header: bool, out: &str) -> CliResult {
    // out path
    let out = out_filename(out);

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
    let workbook = Workbook::new(out.to_str().unwrap())?;
    let mut sheet = workbook.add_worksheet(None)?;
    let ctypes = if equal_width(&lines) {
        // column type
        let cols = Columns::new("").total_col(lines[0].len()).parse();
        let ctypes = ColumnTypes::guess_from_io(&lines[(1 - no_header as usize)..], &cols);
        ctypes.update_excel_column_width(&mut sheet)?;
        Some(ctypes)
    } else {
        None
    };

    for (n, r) in lines.iter().enumerate() {
        write_excel_line(&mut sheet, n, r, ctypes.as_ref())?;
    }

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
) -> CliResult {
    if ctypes.is_some() {
        for ((col, &v), t) in line.iter().enumerate().zip(ctypes.unwrap().iter()) {
            match t.col_type.is_number() {
                true => match v.parse::<f64>() {
                    Ok(v) => sheet.write_number(row as u32, col as u16, v, None)?,
                    Err(_) => sheet.write_string(row as u32, col as u16, v, None)?,
                },
                false => sheet.write_string(row as u32, col as u16, v, None)?,
            }
        }
    } else {
        for (col, &v) in line.iter().enumerate() {
            sheet.write_string(row as u32, col as u16, v, None)?;
        }
    }

    Ok(())
}
