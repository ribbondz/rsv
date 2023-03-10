use super::excel::write_datatype_to_string;
use super::util::werr;
use calamine::DataType;
use std::process;

#[derive(Debug)]
pub struct Columns {
    pub cols: Vec<usize>,
    pub all_cols: Vec<usize>,
    max_col: usize,
    pub all: bool,
}

fn parse_col_usize(col: &str) -> usize {
    col.parse().unwrap_or_else(|_| {
        werr!("{}", "Column syntax error: can only be 0,1,2,5 or 0-2,5.");
        process::exit(1);
    })
}

impl Columns {
    pub fn new(raw: &str) -> Self {
        let mut cols = Columns {
            cols: vec![],
            all_cols: vec![],
            max_col: 0,
            all: true,
        };

        if raw.is_empty() {
            return cols;
        }

        raw.split(',').for_each(|i| cols.parse(i));
        cols.update_status();

        cols
    }

    fn parse(&mut self, col: &str) {
        if col.contains('-') {
            let v = col.split('-').collect::<Vec<_>>();
            let min = parse_col_usize(v[0]);
            let max = parse_col_usize(v[1]);
            for i in min..=max {
                self.push_col(i)
            }
        } else {
            self.push_col(parse_col_usize(col));
        }
    }

    fn push_col(&mut self, col: usize) {
        if !self.cols.contains(&col) {
            self.cols.push(col);
        }
    }

    fn update_status(&mut self) {
        self.max_col = *self.cols.iter().max().unwrap();
        self.all = self.cols.is_empty();
    }

    pub fn iter(&self) -> impl Iterator<Item = &usize> {
        self.cols.iter()
    }

    pub fn max(&self) -> usize {
        self.max_col
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

    pub fn select_owned_vector_and_append_n(&self, all: &[&str]) -> Vec<String> {
        self.cols
            .iter()
            .map(|&i| all[i].to_owned())
            .chain(std::iter::once("n".to_owned()))
            .collect::<Vec<_>>()
    }

    pub fn select_owned_vector_and_append_n2(&self, all: Vec<String>) -> Vec<String> {
        all.into_iter()
            .enumerate()
            .filter_map(|(u, i)| self.cols.contains(&u).then_some(i))
            .chain(std::iter::once("n".to_owned()))
            .collect::<Vec<_>>()
    }

    pub fn col_vec_or_length_of(&self, n: usize) -> Vec<usize> {
        match self.all {
            true => (0..n).collect::<Vec<_>>(),
            false => self.cols.clone(),
        }
    }
}
