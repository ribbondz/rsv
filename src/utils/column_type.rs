use calamine::DataType;

use crate::utils::column;
use std::{
    error::Error,
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    process,
};

use super::{excel_reader::ExcelReader, file, util::is_null};

#[derive(Debug)]
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
    pub fn _new(total_col: usize, cols: &column::Columns) -> Result<Self, Box<dyn Error>> {
        let mut guess = ColumnTypes(vec![]);

        // init to null according to column number
        if cols.is_empty() {
            (0..total_col).for_each(|i| guess.push(i, ColumnType::Null))
        } else {
            cols.iter().for_each(|&i| guess.push(i, ColumnType::Null))
        }

        Ok(guess)
    }

    fn push(&mut self, col_index: usize, col_type: ColumnType) {
        self.0.push(CType {
            col_index,
            col_type,
        })
    }

    fn col_index_at(&self, n: usize) -> usize {
        self.0[n].col_index
    }

    fn col_type_at(&self, n: usize) -> ColumnType {
        self.0[n].col_type
    }

    fn set_as_string(&mut self, n: usize) {
        self.0[n].col_type = ColumnType::String
    }

    fn set_as_float(&mut self, n: usize) {
        self.0[n].col_type = ColumnType::Float
    }

    fn set_as_int(&mut self, n: usize) {
        self.0[n].col_type = ColumnType::Int
    }

    pub fn iter(&self) -> impl Iterator<Item = &CType> {
        self.0.iter()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn guess_from_csv(
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
            first_row
                .iter()
                .enumerate()
                .for_each(|(i, _)| guess.push(i, ColumnType::Null))
        } else {
            cols.iter().for_each(|&i| guess.push(i, ColumnType::Null))
        };

        for l in rdr.take(5000) {
            let l = l.unwrap();

            let v = l.split(sep).collect::<Vec<_>>();

            // ignore bad line
            if cols.max() >= v.len() {
                continue;
            }

            // parse
            (0..guess.len()).for_each(|n| guess.parse_csv_row(n, v[guess.col_index_at(n)]))
        }

        Ok(guess)
    }

    fn parse_csv_row(&mut self, n: usize, v: &str) {
        if is_null(v) {
            return;
        }

        match self.col_type_at(n) {
            ColumnType::Null => {
                if v.parse::<i64>().is_ok() {
                    self.set_as_int(n)
                } else if v.parse::<f64>().is_ok() {
                    self.set_as_float(n)
                } else {
                    self.set_as_string(n)
                }
            }
            ColumnType::Int => {
                if v.parse::<i64>().is_err() {
                    if v.parse::<f64>().is_ok() {
                        self.set_as_float(n)
                    } else {
                        self.set_as_string(n)
                    }
                }
            }
            ColumnType::Float => {
                if v.parse::<f64>().is_err() {
                    self.set_as_string(n)
                }
            }
            ColumnType::String => {}
        }
    }

    pub fn guess_from_excel(
        path: &Path,
        sheet: usize,
        no_header: bool,
        cols: &column::Columns,
    ) -> Result<Self, Box<dyn Error>> {
        let mut guess = ColumnTypes(vec![]);

        // reader
        let mut range = ExcelReader::new(path, sheet)?;

        // take first row to analyze the number of column
        let first_row = match range.next() {
            Some(v) => v,
            None => process::exit(0),
        };

        // init guess according to column number
        if cols.is_empty() {
            first_row
                .iter()
                .enumerate()
                .for_each(|(i, _)| guess.push(i, ColumnType::Null))
        } else {
            cols.iter().for_each(|&i| guess.push(i, ColumnType::Null))
        };

        for l in range.iter().skip(1 - no_header as usize).take(5000) {
            // ignore bad line
            if cols.max() >= l.len() {
                continue;
            }
            // parse
            (0..guess.len()).for_each(|n| guess.parse_excel_row(n, &l[guess.col_index_at(n)]))
        }

        Ok(guess)
    }

    fn parse_excel_row(&mut self, n: usize, v: &DataType) {
        if v.is_empty() {
            return;
        }

        match self.col_type_at(n) {
            ColumnType::Null => {
                if v.is_int() {
                    self.set_as_int(n)
                } else if v.is_float() {
                    self.set_as_float(n)
                } else {
                    self.set_as_string(n)
                }
            }
            ColumnType::Int => {
                if !v.is_int() {
                    if v.is_float() {
                        self.set_as_float(n)
                    } else {
                        self.set_as_string(n)
                    }
                }
            }
            ColumnType::Float => {
                if !v.is_float() {
                    self.set_as_string(n)
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
