use super::{column::Columns, excel_reader::ExcelReader, util::is_null};
use crate::utils::column;
use calamine::DataType;
use rayon::prelude::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use std::{
    error::Error,
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

#[derive(Debug)]
pub struct ColumnTypes(Vec<CType>);

#[derive(Debug)]
pub struct CType {
    pub col_index: usize,
    pub col_type: ColumnType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ColumnType {
    Int,
    Float,
    String,
    Null,
}

impl ColumnTypes {
    fn push(&mut self, col_index: usize, col_type: ColumnType) {
        self.0.push(CType {
            col_index,
            col_type,
        })
    }

    pub fn iter(&self) -> impl Iterator<Item = &CType> {
        self.0.iter()
    }

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
            .par_iter()
            .map(|r| r.split(sep).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let cols_vec = match cols.all {
            true => (0..lines[0].len()).collect::<Vec<_>>(),
            false => cols.iter().copied().collect::<Vec<_>>(),
        };
        let types = cols_vec
            .into_par_iter()
            .map(|i| (i, parse_col_type_at(i, &lines)))
            .collect::<Vec<_>>();
        let guess = types.iter().fold(ColumnTypes(vec![]), |mut a, b| {
            a.push(b.0, b.1.clone());
            a
        });

        Ok(Some(guess))
    }

    pub fn from_excel(range: &ExcelReader, no_header: bool, cols: &Columns) -> Option<Self> {
        let lines = range
            .iter()
            .skip(1 - no_header as usize)
            .collect::<Vec<_>>();

        if lines.is_empty() {
            return None;
        }

        let cols_vec = match cols.all {
            true => (0..lines[0].len()).collect::<Vec<_>>(),
            false => cols.iter().copied().collect::<Vec<_>>(),
        };
        let types = cols_vec
            .into_par_iter()
            .map(|i| (i, parse_excel_col_type_at(i, &lines)))
            .collect::<Vec<_>>();

        let guess = types.iter().fold(ColumnTypes(vec![]), |mut a, b| {
            a.push(b.0, b.1.clone());
            a
        });

        Some(guess)
    }

    pub fn guess_from_io(v: &[Vec<&str>], cols: &Columns) -> Self {
        let cols_vec = match cols.all {
            true => (0..v[0].len()).collect::<Vec<_>>(),
            false => cols.iter().copied().collect::<Vec<_>>(),
        };
        let types = cols_vec
            .into_par_iter()
            .map(|i| (i, parse_col_type_at(i, v)))
            .collect::<Vec<_>>();

        types.iter().fold(ColumnTypes(vec![]), |mut a, b| {
            a.push(b.0, b.1.clone());
            a
        })
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

fn parse_excel_col_type_at(n: usize, v: &[&[DataType]]) -> ColumnType {
    let mut ctype = ColumnType::Null;
    for &r in v {
        if ctype.is_string() {
            break;
        }
        ctype.update_by_excel_cell(&r[n]);
    }

    ctype
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

    pub fn update_by_excel_cell(&mut self, f: &DataType) {
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
