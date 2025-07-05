use super::{
    column_type::{ColumnType, ColumnTypes},
    row_split::CsvRowSplitter,
    util,
};
use ahash::HashSet;
use calamine::Data;
use get_fields::GetFields;
use rayon::prelude::*;
use std::fmt::Display;
use tabled::{builder::Builder, settings::Style, Table};

#[derive(Debug)]
pub struct ColumnStats {
    max_col: usize,
    cols: Vec<usize>,
    pub stat: Vec<CStat>,
    pub rows: usize,
}

#[derive(Debug, GetFields)]
pub struct CStat {
    col_index: usize,
    col_type: ColumnType,
    name: String,
    min: f64,
    max: f64,
    min_string: String,
    max_string: String,
    mean: f64,
    unique: usize,
    null: usize,
    total: f64,
    unique_hashset: HashSet<String>,
}

impl ColumnStats {
    pub fn new(col_type: &ColumnTypes, col_name: &[String]) -> Self {
        let mut s = ColumnStats {
            max_col: 0,
            cols: vec![],
            stat: vec![],
            rows: 0,
        };
        col_type
            .iter()
            .for_each(|c| s.push(c.col_index, c.col_type.clone(), &col_name[c.col_index]));

        s
    }

    fn push(&mut self, col_index: usize, col_type: ColumnType, name: &str) {
        let stat = CStat {
            col_index,
            col_type,
            name: name.to_owned(),
            min: f64::MAX,
            max: f64::MIN,
            min_string: String::new(),
            max_string: String::new(),
            mean: 0.0,
            total: 0.0,
            unique: 0,
            null: 0,
            unique_hashset: HashSet::default(),
        };
        self.cols.push(col_index);
        self.stat.push(stat);

        if col_index > self.max_col {
            self.max_col = col_index
        }
    }

    pub fn parse_line_by_fields(&mut self, v: &[&str]) {
        if self.max_col >= v.len() {
            println!("[info] ignore a bad line: {v:?}");
            return;
        }

        self.cols
            .iter()
            .zip(&mut self.stat)
            .for_each(|(&i, c)| c.parse(v[i]));

        self.rows += 1;
    }

    pub fn parse_line(&mut self, line: &str, sep: char, quote: char) {
        let v = CsvRowSplitter::new(line, sep, quote).collect::<Vec<_>>();
        self.parse_line_by_fields(&v);
    }

    pub fn parse_excel_row(&mut self, v: &[Data]) {
        if self.max_col >= v.len() {
            println!("[info] ignore a bad line: {v:?}");
            return;
        }

        self.cols.iter().zip(&mut self.stat).for_each(|(&i, c)| {
            let t = &v[i];
            match t {
                Data::String(v) => c.parse(v),
                _ => c.parse(&t.to_string()),
            };
        });

        self.rows += 1;
    }

    pub fn cal_unique_and_mean(&mut self) {
        self.stat.iter_mut().for_each(|s| {
            s.unique = s.unique_hashset.len();

            match s.col_type {
                ColumnType::Float | ColumnType::Int => {
                    let n = self.rows - s.null;
                    if n != 0 {
                        s.mean = s.total / n as f64;
                    }
                }
                _ => {}
            }
        })
    }

    fn iter(&self) -> impl Iterator<Item = &CStat> {
        self.stat.iter()
    }

    pub fn merge(&mut self, other: ColumnStats) {
        self.rows += other.rows;

        // parallel update
        other
            .stat
            .into_par_iter()
            .zip(&mut self.stat)
            .for_each(|(o, c)| c.merge(o));
    }

    fn print_table_vertical(&self) -> Table {
        let mut builder = Builder::default();

        // header
        let r = ["col", "type", "min", "max", "mean", "unique", "null"];
        builder.push_record(r);

        // columns
        self.iter().for_each(|c| {
            let mut r = vec![];
            r.push(c.name.to_owned());
            r.push(format!("{}", c.col_type));
            r.push(c.min_fmt());
            r.push(c.max_fmt());
            r.push(c.mean_fmt());
            r.push(c.unique_fmt());
            r.push(c.null.to_string());
            builder.push_record(r);
        });

        // build
        let mut table = builder.build();

        // style
        table.with(Style::sharp());

        table
    }

    pub fn print(&self) {
        let table = self.print_table_vertical();
        println!("{table}");
    }
}

impl Clone for ColumnStats {
    fn clone(&self) -> Self {
        let mut o = ColumnStats {
            max_col: self.max_col,
            stat: vec![],
            cols: vec![],
            rows: 0,
        };

        self.iter()
            .for_each(|c| o.push(c.col_index, c.col_type.clone(), &c.name));

        o
    }
}

impl Display for ColumnStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let t = self.print_table_vertical().to_string();
        f.write_str(&t)?;

        Ok(())
    }
}

impl CStat {
    pub fn get_fields_values(&self) -> Vec<String> {
        vec![
            self.col_index.to_string(),
            self.col_type.to_string(),
            self.name.clone(),
            self.min_fmt(),
            self.max_fmt(),
            self.min_string.clone(),
            self.max_string.clone(),
            self.mean_fmt(),
            self.unique_fmt(),
            self.null.to_string(),
            self.total.to_string(),
            String::new(),
        ]
    }

    pub fn parse(&mut self, f: &str) {
        if util::is_null(f) {
            self.null += 1;
            return;
        }
        match self.col_type {
            ColumnType::Int => {
                if let Ok(v) = f.parse::<i64>() {
                    self.update_number_stat(v as f64)
                } else if let Ok(v) = f.parse::<f64>() {
                    self.set_as_float();
                    self.update_number_stat(v)
                } else {
                    self.set_as_string();
                    self.update_string_stat(f)
                }
            }
            ColumnType::Float => {
                if let Ok(v) = f.parse::<f64>() {
                    self.update_number_stat(v)
                } else {
                    self.set_as_string();
                    self.update_string_stat(f)
                }
            }
            ColumnType::String => self.update_string_stat(f),
            ColumnType::Null => {}
        }
        // ignore unique for FLOAT type
        if !self.is_float() {
            self.insert_unique(f);
        }
    }

    fn insert_unique(&mut self, v: &str) {
        // quicker compared with no check
        if !self.unique_hashset.contains(v) {
            self.unique_hashset.insert(v.to_owned());
        }
    }

    fn set_as_float(&mut self) {
        self.col_type = ColumnType::Float
    }

    fn set_as_string(&mut self) {
        self.col_type = ColumnType::String
    }

    fn is_int(&self) -> bool {
        self.col_type == ColumnType::Int
    }

    fn is_float(&self) -> bool {
        self.col_type == ColumnType::Float
    }

    fn is_string(&self) -> bool {
        self.col_type == ColumnType::String
    }

    fn update_number_stat(&mut self, v: f64) {
        if v > self.max {
            self.max = v
        }
        if v < self.min {
            self.min = v
        }
        self.total += v;
    }

    fn update_string_stat(&mut self, v: &str) {
        if self.min_string.is_empty() || v < &self.min_string {
            self.min_string = v.to_owned();
        }
        if v > &self.max_string {
            self.max_string = v.to_owned();
        }
    }

    fn merge(&mut self, o: CStat) {
        if self.col_type != o.col_type {
            self.col_type = o.col_type
        }
        if o.min < self.min {
            self.min = o.min;
        }
        if o.max > self.max {
            self.max = o.max
        }
        if self.min_string.is_empty() || o.min_string < self.min_string {
            self.min_string = o.min_string
        }
        if o.max_string > self.max_string {
            self.max_string = o.max_string
        }
        self.null += o.null;
        self.total += o.total;
        self.unique_hashset.extend(o.unique_hashset)
    }

    fn mean_fmt(&self) -> String {
        if self.is_string() {
            "-".to_owned()
        } else {
            format!("{:.2}", self.mean)
        }
    }

    fn min_fmt(&self) -> String {
        if self.is_string() {
            self.min_string.to_owned()
        } else if self.is_int() {
            format!("{:.0}", if self.min == f64::MAX { 0.0 } else { self.min })
        } else {
            format!("{:.2}", if self.min == f64::MAX { 0.0 } else { self.min })
        }
    }

    fn max_fmt(&self) -> String {
        if self.is_string() {
            self.max_string.to_owned()
        } else if self.is_int() {
            format!("{:.0}", if self.max == f64::MIN { 0.0 } else { self.max })
        } else {
            format!("{:.2}", if self.max == f64::MIN { 0.0 } else { self.max })
        }
    }

    fn unique_fmt(&self) -> String {
        if self.is_float() {
            "-".to_owned()
        } else {
            self.unique.to_string()
        }
    }
}
