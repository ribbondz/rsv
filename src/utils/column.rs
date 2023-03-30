use super::excel::write_datatype_to_string;
use super::util::werr;
use calamine::DataType;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process;

#[derive(Debug)]
pub struct Columns<'a> {
    path: Option<&'a Path>,
    sep: &'a str,
    pub cols: Vec<usize>,
    pub max: usize,
    pub select_all: bool,
    raw: &'a str,
    total: Option<usize>,
    pub parsed: bool,
}

fn parse_col_usize(col: &str) -> usize {
    col.parse().unwrap_or_else(|_| {
        werr!(
            "{}",
            "Column syntax error: can be something like 0,1,2,5 or 0-2,5 or -1 or -3--1."
        );
        process::exit(1);
    })
}

fn parse_i32(col: &str) -> i32 {
    col.parse().unwrap_or_else(|_| {
        werr!(
            "{}",
            "Column syntax error: can be something like 0,1,2,5 or 0-2,5 or -1 or -3--1."
        );
        process::exit(1);
    })
}

fn split_pat_at<'a>(source: &'a str, pat: &'a str, n: usize) -> (&'a str, &'a str) {
    let (i, _) = source.match_indices(pat).nth(n).unwrap();
    let (first, second) = source.split_at(i);
    (first, &second[pat.len()..])
}

impl<'a> Columns<'a> {
    pub fn new(raw: &str) -> Columns {
        Columns {
            path: None,
            sep: "",
            cols: vec![],
            max: 0,
            select_all: true,
            raw,
            total: None,
            parsed: false,
        }
    }

    pub fn total_col(mut self, total: usize) -> Self {
        self.total = Some(total);
        self
    }

    pub fn total_col_of(mut self, path: &'a Path, sep: &'a str) -> Self {
        self.path = Some(path);
        self.sep = sep;
        self
    }

    pub fn parse(mut self) -> Self {
        self.parsed = true;

        if self.raw.is_empty() {
            return self;
        }

        self.raw.split(',').for_each(|i| {
            if !i.trim().is_empty() {
                self.parse_col(i)
            }
        });
        self.update_status();

        self
    }

    fn parse_col(&mut self, col: &str) {
        match (col.starts_with('-'), col.matches('-').count()) {
            // positive
            (true, 1) => {
                let c = { self.true_col(col) };
                self.push(c)
            }
            (true, _) => {
                let (first, second) = split_pat_at(col, "-", 1);
                let min = { self.true_col(first) };
                let max = { self.true_col(second) };
                self.push_range(min, max);
            }
            (false, 0) => {
                let c = { self.true_col(col) };
                self.push(c)
            }
            (false, _) => {
                let (first, second) = split_pat_at(col, "-", 0);
                let min = { self.true_col(first) };
                let max = { self.true_col(second) };
                self.push_range(min, max);
            }
        };
    }

    fn true_col(&mut self, col: &str) -> usize {
        if col.starts_with('-') {
            if self.total.is_none() {
                let mut first_line = String::new();
                let f = File::open(self.path.unwrap()).expect("unable to open file.");
                BufReader::new(f)
                    .read_line(&mut first_line)
                    .expect("read error.");
                self.total = Some(first_line.split(self.sep).count());
            }
            let i = (self.total.unwrap() as i32) + parse_i32(col);
            if i < 0 {
                werr!("Column {} does not exist.", col);
                process::exit(1);
            }
            i as usize
        } else {
            parse_col_usize(col)
        }
    }

    fn push(&mut self, col: usize) {
        if !self.cols.contains(&col) {
            self.cols.push(col);
        }
    }

    fn push_range(&mut self, min: usize, max: usize) {
        if min > max {
            werr!("Min column is bigger than max column.");
            process::exit(1);
        }
        for i in min..=max {
            self.push(i)
        }
    }

    fn update_status(&mut self) {
        self.max = *self.cols.iter().max().unwrap();
        self.select_all = self.cols.is_empty();
    }

    pub fn iter(&self) -> impl Iterator<Item = &usize> {
        self.cols.iter()
    }

    pub fn artificial_cols_with_appended_n(&self) -> Vec<String> {
        self.iter()
            .map(|&i| format!("col{i}"))
            .chain(std::iter::once("n".to_owned()))
            .collect::<Vec<_>>()
    }

    pub fn artificial_n_cols(&self, n: usize) -> Vec<String> {
        (0..n).map(|i| format!("col{i}")).collect::<Vec<_>>()
    }

    pub fn select_owned_string(&self, all: &[&str]) -> String {
        self.iter().map(|&i| all[i]).collect::<Vec<_>>().join(",")
    }

    pub fn select_owned_string_from_excel_datatype(&self, all: &[DataType]) -> String {
        let mut o = String::new();
        let mut col = self.cols.iter().peekable();
        while let Some(&i) = col.next() {
            write_datatype_to_string(&mut o, &all[i]);
            if col.peek().is_some() {
                o.push(',');
            }
        }

        o
    }

    pub fn select_owned_vec_from_excel_datatype(&self, all: &[DataType]) -> Vec<String> {
        self.cols
            .iter()
            .map(|&i| all[i].to_string())
            .chain(std::iter::once("n".to_owned()))
            .collect::<Vec<_>>()
    }

    pub fn select_owned_vector_and_append_n(&self, all: &[&str]) -> Vec<String> {
        self.cols
            .iter()
            .map(|&i| all[i].to_owned())
            .chain(std::iter::once("n".to_owned()))
            .collect::<Vec<_>>()
    }

    pub fn col_vec_or_length_of(&self, n: usize) -> Vec<usize> {
        match self.select_all {
            true => (0..n).collect::<Vec<_>>(),
            false => self.cols.clone(),
        }
    }
}
