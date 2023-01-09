use std::fmt::Display;

use ahash::HashSet;
use tabled::{builder::Builder, Style, Table};

use super::{
    column_type::{ColumnType, ColumnTypes},
    util,
};

#[derive(Debug)]
pub struct ColumnStats {
    max_col: usize,
    cols: Vec<usize>,
    stat: Vec<CStat>,
    pub rows: usize,
}

#[derive(Debug)]
struct CStat {
    col_index: usize,
    col_type: ColumnType,
    name: String,
    min: f64,
    max: f64,
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
            .for_each(|c| s.push(c.col_index, c.col_type, &col_name[c.col_index]));

        s
    }

    fn push(&mut self, col_index: usize, col_type: ColumnType, name: &str) {
        let stat = CStat {
            col_index,
            col_type,
            name: name.to_owned(),
            min: f64::MAX,
            max: f64::MIN,
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

    pub fn parse_line(&mut self, line: &str, sep: &str) {
        let v = line.split(sep).collect::<Vec<_>>();

        if self.max_col >= v.len() {
            println!("ignore a bad line: {:?}", v);
            return;
        }

        let v = self.cols.iter().map(|&i| v[i]).collect::<Vec<_>>();
        for (i, f) in v.iter().enumerate() {
            self.parse_col(i, f)
        }

        self.rows += 1;
    }

    fn parse_col(&mut self, i: usize, f: &str) {
        let c = &mut self.stat[i];

        if util::is_null(f) {
            c.null += 1;
            return;
        }

        if c.col_type != ColumnType::Float {
            c.insert_unique(f);
        }

        match c.col_type {
            ColumnType::Float => match f.parse::<f64>() {
                Ok(v) => {
                    if v > c.max {
                        c.max = v
                    }
                    if v < c.min {
                        c.min = v
                    }
                    c.total += v;
                }
                // fallback to string
                Err(_) => c.col_type = ColumnType::String,
            },
            ColumnType::Int => match f.parse::<i64>() {
                Ok(v) => {
                    let v = v as f64;
                    if v > c.max {
                        c.max = v
                    }
                    if v < c.min {
                        c.min = v
                    }
                    c.total += v
                }
                // fallback to string
                Err(_) => match f.parse::<f64>() {
                    Ok(v) => {
                        c.total += v;
                        c.col_type = ColumnType::Float
                    }
                    Err(_) => c.col_type = ColumnType::String,
                },
            },
            _ => {}
        }
    }

    pub fn finalize_stats(&mut self) {
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

    fn into_iter(self) -> impl Iterator<Item = CStat> {
        self.stat.into_iter()
    }

    pub fn merge(&mut self, other: ColumnStats) {
        self.rows += other.rows;

        other.into_iter().zip(&mut self.stat).for_each(|(o, c)| {
            if c.col_type != o.col_type {
                c.col_type = o.col_type
            }

            if o.min < c.min {
                c.min = o.min;
            }

            if o.max > c.max {
                c.max = o.max
            }

            c.null += o.null;
            c.total += o.total;

            o.unique_hashset.iter().for_each(|i| c.insert_unique(i));
        })
    }

    fn print_table_vertical(&self) -> Table {
        let mut builder = Builder::default();

        // header
        let r = vec!["col", "type", "min", "max", "mean", "unique", "null"];
        builder.set_columns(r);

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
            builder.add_record(r);
        });

        // build
        let mut table = builder.build();

        // style
        table.with(Style::sharp());

        table
    }

    fn _print_table_horizontal(&self) -> Table {
        let mut builder = Builder::default();

        // header
        let mut r = vec!["".to_owned()];
        self.iter().for_each(|c| r.push(c.name.to_owned()));
        builder.set_columns(r);

        // type
        let mut r = vec!["type".to_owned()];
        self.iter().for_each(|c| r.push(c.col_type.to_string()));
        builder.add_record(r);

        // min
        let mut r = vec!["min".to_owned()];
        self.iter().for_each(|c| {
            let v = if c.col_type == ColumnType::String {
                "-".to_owned()
            } else {
                c.min.to_string()
            };
            r.push(v)
        });
        builder.add_record(r);

        // max
        let mut r = vec!["max".to_owned()];
        self.iter().for_each(|c| {
            let v = if c.col_type == ColumnType::String {
                "-".to_owned()
            } else {
                c.max.to_string()
            };
            r.push(v)
        });
        builder.add_record(r);

        // mean
        let mut r = vec!["mean".to_owned()];
        self.iter().for_each(|c| {
            let v = if c.col_type == ColumnType::String {
                "-".to_owned()
            } else {
                format!("{:.2}", c.mean)
            };
            r.push(v)
        });
        builder.add_record(r);

        // unique
        let mut r = vec!["unique".to_owned()];
        self.iter().for_each(|c| {
            let v = if c.col_type == ColumnType::Float {
                "-".to_owned()
            } else {
                c.unique.to_string()
            };
            r.push(v)
        });
        builder.add_record(r);

        // null
        let mut r = vec!["null".to_owned()];
        self.iter().for_each(|c| r.push(c.null.to_string()));
        builder.add_record(r);

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
            .for_each(|c| o.push(c.col_index, c.col_type, &c.name));

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
    fn insert_unique(&mut self, v: &str) {
        // quicker compared with no check
        if !self.unique_hashset.contains(v) {
            self.unique_hashset.insert(v.to_owned());
        }
    }

    fn is_string(&self) -> bool {
        self.col_type == ColumnType::String
    }

    fn is_float(&self) -> bool {
        self.col_type == ColumnType::Float
    }

    fn is_int(&self) -> bool {
        self.col_type == ColumnType::Int
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
            "-".to_owned()
        } else if self.is_int() {
            format!("{:.0}", if self.min == f64::MAX { 0.0 } else { self.min })
        } else {
            format!("{:.2}", if self.min == f64::MAX { 0.0 } else { self.min })
        }
    }

    fn max_fmt(&self) -> String {
        if self.is_string() {
            "-".to_owned()
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
