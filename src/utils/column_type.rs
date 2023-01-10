use crate::utils::column;
use std::{
    error::Error,
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use super::{file, util::is_null};

pub struct ColumnTypes(Vec<CType>);

#[derive(Debug)]
pub struct CType {
    pub col_index: usize,
    pub col_type: ColumnType,
}

#[derive(Debug, Clone, Copy, PartialEq)]
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

    fn col_type_at(&self, n: usize) -> ColumnType {
        self.0[n].col_type
    }

    fn set_col_type(&mut self, n: usize, t: ColumnType) {
        self.0[n].col_type = t
    }

    pub fn iter(&self) -> impl Iterator<Item = &CType> {
        self.0.iter()
    }

    pub fn guess(
        path: &Path,
        filename: &str,
        sep: &str,
        no_header: bool,
        cols: &column::Columns,
    ) -> Result<Self, Box<dyn Error>> {
        let mut guess = ColumnTypes(vec![]);

        // reader
        let mut rdr = BufReader::new(File::open(path)?).lines();

        // take first row to analyze the number of column
        let first_row = if no_header {
            file::first_row(filename)?
        } else {
            rdr.next().unwrap()?
        };
        let first_row = first_row.split(sep).collect::<Vec<_>>();

        // init guess according to column number
        if cols.is_empty() {
            for (i, _) in first_row.iter().enumerate() {
                guess.push(i, ColumnType::Null)
            }
        } else {
            for &i in cols.iter() {
                guess.push(i, ColumnType::Null)
            }
        };

        for l in rdr.take(5000) {
            let l = l.unwrap();

            let mut v = l.split(sep).collect::<Vec<_>>();

            // ignore bad line
            if cols.max() >= v.len() {
                continue;
            }

            // column select
            if !cols.is_empty() {
                v = cols.iter().map(|&i| v[i]).collect();
            }

            // parse
            v.iter().enumerate().for_each(|(i, f)| guess.parse(i, f))
        }

        Ok(guess)
    }

    fn parse(&mut self, n: usize, v: &str) {
        if is_null(v) {
            return;
        }

        match self.col_type_at(n) {
            ColumnType::Null => {
                if v.parse::<i64>().is_ok() {
                    self.set_col_type(n, ColumnType::Int);
                } else if v.parse::<f64>().is_ok() {
                    self.set_col_type(n, ColumnType::Float);
                } else {
                    self.set_col_type(n, ColumnType::String);
                }
            }
            ColumnType::Int => {
                if v.parse::<i64>().is_err() {
                    if v.parse::<f64>().is_ok() {
                        self.set_col_type(n, ColumnType::Float)
                    } else {
                        self.set_col_type(n, ColumnType::String)
                    }
                }
            }
            ColumnType::Float => {
                if v.parse::<f64>().is_err() {
                    self.set_col_type(n, ColumnType::String)
                }
            }
            ColumnType::String => {}
        }
    }
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
