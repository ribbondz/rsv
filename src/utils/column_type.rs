use super::{cli_result::CliResult, column::Columns, reader::ExcelReader, util::is_null};
use crate::utils::column;
use calamine::{Data, DataType};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use std::{
    error::Error,
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};
use xlsxwriter::Worksheet;

#[derive(Debug)]
pub struct ColumnTypes(Vec<CType>);

#[derive(Debug)]
pub struct CType {
    pub col_index: usize,
    pub col_type: ColumnType,
    pub max_length: usize, // for excel export
}

#[derive(Debug, Clone, PartialEq)]
pub enum ColumnType {
    Int,
    Float,
    String,
    Null,
}

impl ColumnTypes {
    fn push(&mut self, col_index: usize, col_type: ColumnType, max_length: usize) {
        self.0.push(CType {
            col_index,
            col_type,
            max_length,
        })
    }

    pub fn iter(&self) -> impl Iterator<Item = &CType> {
        self.0.iter()
    }

    // parallel guess based on columns
    pub fn guess_from_csv(
        path: &Path,
        sep: &str,
        no_header: bool,
        cols: &column::Columns,
    ) -> Result<Option<Self>, Box<dyn Error>> {
        // reader
        let rdr = BufReader::new(File::open(path)?).lines();
        let lines = rdr
            .skip(1 - no_header as usize)
            .take(5000)
            .filter_map(|i| i.ok())
            .collect::<Vec<_>>();

        if lines.is_empty() {
            return Ok(None);
        }

        // split
        let lines = lines
            .iter()
            .map(|r| r.split(sep).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let guess = cols
            .col_vec_or_length_of(lines[0].len())
            .into_par_iter()
            .map(|n| (n, parse_col_type_at(n, &lines), max_length_at(n, &lines)))
            .collect::<Vec<_>>()
            .iter()
            .fold(ColumnTypes(vec![]), |mut a, b| {
                a.push(b.0, b.1.clone(), b.2);
                a
            });

        Ok(Some(guess))
    }

    // sequential guess given that excel is usually small
    pub fn guess_from_excel(range: &ExcelReader, no_header: bool, cols: &Columns) -> Option<Self> {
        let lines = range
            .iter()
            .skip(1 - no_header as usize)
            .take(5000)
            .collect::<Vec<_>>();

        if lines.is_empty() {
            return None;
        }

        let mut guess = ColumnTypes(vec![]);
        for c in cols.col_vec_or_length_of(lines[0].len()) {
            // max_length is meaningless for excel, so set default to 0
            guess.push(c, parse_excel_col_type_at(c, &lines), 0)
        }

        Some(guess)
    }

    // sequential guess given that io is usually small
    pub fn guess_from_io(v: &[Vec<&str>], cols: &Columns) -> Self {
        let v = if v.len() < 5000 { v } else { &v[..5000] };

        let mut guess = ColumnTypes(vec![]);
        for c in cols.col_vec_or_length_of(v[0].len()) {
            guess.push(c, parse_col_type_at(c, v), max_length_at(c, v))
        }

        guess
    }

    pub fn update_excel_column_width(&self, sheet: &mut Worksheet) -> CliResult {
        for c in self.iter() {
            sheet.set_column(
                c.col_index as u16,
                c.col_index as u16,
                c.excel_col_width(),
                None,
            )?;
        }

        Ok(())
    }
}

fn parse_col_type_at(n: usize, v: &[Vec<&str>]) -> ColumnType {
    let mut ctype = ColumnType::Null;
    for r in v {
        if ctype.is_string() {
            break;
        }
        let f = r[n];
        if is_null(f) {
            continue;
        }
        ctype.update(f);
    }

    ctype
}

fn parse_excel_col_type_at(n: usize, v: &[&[Data]]) -> ColumnType {
    let mut ctype = ColumnType::Null;
    for &r in v {
        if ctype.is_string() {
            break;
        }
        ctype.update_by_excel_cell(&r[n]);
    }

    ctype
}

fn max_length_at(n: usize, v: &[Vec<&str>]) -> usize {
    v.iter().map(|r| r[n].len()).max().unwrap_or(0)
}

impl Display for ColumnType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ColumnType::Float => f.write_str("float")?,
            ColumnType::Int => f.write_str("int")?,
            ColumnType::String => f.write_str("string")?,
            ColumnType::Null => f.write_str("null")?,
        }

        Ok(())
    }
}

impl ColumnType {
    pub fn is_string(&self) -> bool {
        self == &ColumnType::String
    }

    pub fn is_number(&self) -> bool {
        self == &ColumnType::Int || self == &ColumnType::Float
    }

    pub fn update(&mut self, f: &str) {
        match self {
            ColumnType::Null => {
                *self = if f.parse::<i64>().is_ok() {
                    ColumnType::Int
                } else if f.parse::<f64>().is_ok() {
                    ColumnType::Float
                } else {
                    ColumnType::String
                }
            }
            ColumnType::Int => {
                if f.parse::<i64>().is_err() {
                    *self = if f.parse::<f64>().is_ok() {
                        ColumnType::Float
                    } else {
                        ColumnType::String
                    }
                }
            }
            ColumnType::Float => {
                if f.parse::<f64>().is_err() {
                    *self = ColumnType::String
                }
            }
            _ => {}
        }
    }

    pub fn update_by_excel_cell(&mut self, f: &Data) {
        match self {
            ColumnType::Null => {
                *self = if f.is_int() {
                    ColumnType::Int
                } else if f.is_float() {
                    ColumnType::Float
                } else {
                    ColumnType::String
                };
            }
            ColumnType::Int => {
                if !f.is_int() {
                    *self = if f.is_float() {
                        ColumnType::Float
                    } else {
                        ColumnType::String
                    }
                };
            }
            ColumnType::Float => {
                if !f.is_float() {
                    *self = ColumnType::String;
                }
            }
            _ => {}
        }
    }
}

impl CType {
    pub fn excel_col_width(&self) -> f64 {
        let w = self.max_length as f64;
        // set min-width and max-width
        w.clamp(6.0, 60.0)
    }
}
