use super::{excel_reader::ExcelReader, file, util::is_null};
use crate::utils::column;
use calamine::DataType;
use std::{
    error::Error,
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    process,
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

    pub fn from(cols: Vec<ColumnType>) -> Result<Self, Box<dyn Error>> {
        let mut guess = ColumnTypes(vec![]);

        cols.into_iter()
            .enumerate()
            .for_each(|(i, t)| guess.push(i, t));

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

    pub fn iter(&self) -> impl Iterator<Item = &CType> {
        self.0.iter()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn guess_from_csv(
        path: &Path,
        sep: &str,
        no_header: bool,
        cols: &column::Columns,
    ) -> Result<Option<Self>, Box<dyn Error>> {
        let mut guess = ColumnTypes(vec![]);

        // reader
        let mut rdr = BufReader::new(File::open(path)?).lines();

        // take first row to analyze the number of column
        let first_row = if no_header {
            match file::first_row(path)? {
                Some(v) => v,
                None => return Ok(None),
            }
        } else {
            match rdr.next() {
                Some(r) => r?,
                None => return Ok(None),
            }
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
            let l = l?;
            let v = l.split(sep).collect::<Vec<_>>();
            // ignore bad line
            if cols.max() >= v.len() {
                continue;
            }
            // parse
            (0..guess.len()).for_each(|n| guess.parse_csv_row(n, v[guess.col_index_at(n)]))
        }

        Ok(Some(guess))
    }

    fn parse_csv_row(&mut self, n: usize, v: &str) {
        let ctype = &mut self.0[n].col_type;
        if ctype.is_string() || is_null(v) {
            return;
        }
        ctype.update(v);
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
        let ctype = &mut self.0[n].col_type;
        if ctype.is_string() || v.is_empty() {
            return;
        }
        ctype.update_by_excel_cell(v);
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

impl ColumnType {
    pub fn is_string(&self) -> bool {
        self == &ColumnType::String
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
