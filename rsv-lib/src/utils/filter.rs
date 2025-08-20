use super::{
    math_expr_parser::{AST, CompiledExpr},
    row_split::CsvRowSplitter,
};
use crate::utils::util::werr_exit;
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

#[derive(Debug)]
enum Op {
    Equal,
    NotEqual,
    Gt,
    Ge,
    Lt,
    Le,
}

impl Op {
    fn evaluate(&self, a: f64, b: f64) -> bool {
        match self {
            Op::Equal => a == b,
            Op::NotEqual => a != b,
            Op::Gt => a > b,
            Op::Ge => a >= b,
            Op::Lt => a < b,
            Op::Le => a <= b,
        }
    }
}

struct FilterItem {
    col: usize,
    is_numeric: bool,
    op: Op,
    f64_value: f64,
    str_value: String,
    f64_values: Vec<f64>,
    str_values: Vec<String>,
    is_math_expr: bool,
    ast: Box<CompiledExpr>,
}

pub struct Filter<'a> {
    raw: &'a str,
    total: Option<usize>,
    path: Option<&'a Path>,
    sep: char,
    quote: char,
    filters: Vec<FilterItem>,
    pub parsed: bool,
}

fn parse_col_usize(col: &str) -> usize {
    col.parse().unwrap_or_else(|_| {
        werr_exit!(
            "{}",
            "Column syntax error: can be something like 0 (first column), -1 (last column)."
        );
    })
}

fn parse_i32(col: &str) -> i32 {
    col.parse().unwrap_or_else(|_| {
        werr_exit!(
            "{}",
            "Column syntax error: can be something like 0 (first column), -1 (last column)."
        );
    })
}

impl<'a> Filter<'a> {
    pub fn new(raw: &'a str) -> Self {
        Filter {
            raw,
            total: None,
            path: None,
            sep: ',',
            quote: '"',
            filters: vec![],
            parsed: false,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.filters.is_empty()
    }

    pub fn total_col(mut self, total: usize) -> Self {
        self.total = Some(total);
        self
    }

    pub fn total_col_of(mut self, path: &'a Path, sep: char, quote: char) -> Self {
        self.path = Some(path);
        self.sep = sep;
        self.quote = quote;
        self
    }

    fn true_col(&mut self, col: &str) -> usize {
        if col.starts_with('-') {
            if self.total.is_none() {
                let mut first_line = String::new();
                let f = File::open(self.path.unwrap()).expect("unable to open file.");
                BufReader::new(f)
                    .read_line(&mut first_line)
                    .expect("read error.");
                self.total = Some(CsvRowSplitter::new(&first_line, self.sep, self.quote).count());
            }
            let i = (self.total.unwrap() as i32) + parse_i32(col);
            if i < 0 {
                werr_exit!("Column {} does not exist.", col);
            }
            i as usize
        } else {
            parse_col_usize(col)
        }
    }

    pub fn parse(mut self) -> Self {
        self.parsed = true;

        if self.raw.is_empty() {
            return self;
        }

        self.raw
            .split('&')
            .filter(|&i| !i.is_empty())
            .for_each(|one| self.parse_one(one));

        self
    }

    fn parse_one(&mut self, one: &str) {
        // matching order is important
        let re = Regex::new("!=|>=|<=|=|>|<").unwrap();
        let v = re.split(one).collect::<Vec<_>>();

        if v.len() != 2 {
            werr_exit!("Error: Filter syntax is wrong, run <rsv select -h> for help.");
        }

        // parse column
        let mut col = v[0].to_owned();
        let is_numeric = col.ends_with(['n', 'N']);
        if is_numeric {
            col.pop();
        }
        let col = { self.true_col(&col) };

        // check whether rhs is a math expr
        // @1 or c1 represents first column
        let is_math_expr = v[1].contains(['+', '*', '/', '%', '^', '(', '@', 'c'])
            || v[1].rfind('-').unwrap_or(0) > 0;

        let mut item = FilterItem {
            col,
            is_numeric,
            op: Op::NotEqual,
            f64_value: 0.0,
            f64_values: vec![],
            str_value: String::new(),
            str_values: vec![],
            is_math_expr,
            ast: Box::new(AST::parse("")),
        };

        // parse filter, matching order is important
        match (is_math_expr, is_numeric) {
            (true, _) => {
                item.op = if one.contains("!=") {
                    Op::NotEqual
                } else if one.contains(">=") {
                    Op::Ge
                } else if one.contains("<=") {
                    Op::Le
                } else if one.contains('=') {
                    Op::Equal
                } else if one.contains('>') {
                    Op::Gt
                } else if one.contains('<') {
                    Op::Lt
                } else {
                    Op::NotEqual
                };
                item.ast = Box::new(AST::parse(v[1]));
            }
            (false, true) => {
                if one.contains("!=") {
                    item.op = Op::NotEqual;
                    item.f64_values = parse_f64_vec(v[1]);
                } else if one.contains(">=") {
                    item.op = Op::Ge;
                    item.f64_value = parse_f64(v[1]);
                } else if one.contains("<=") {
                    item.op = Op::Le;
                    item.f64_value = parse_f64(v[1]);
                } else if one.contains('=') {
                    item.op = Op::Equal;
                    item.f64_values = parse_f64_vec(v[1]);
                } else if one.contains('>') {
                    item.op = Op::Gt;
                    item.f64_value = parse_f64(v[1]);
                } else if one.contains('<') {
                    item.op = Op::Lt;
                    item.f64_value = parse_f64(v[1]);
                }
            }
            (false, false) => {
                if one.contains("!=") {
                    item.op = Op::NotEqual;
                    item.str_values = v[1].split(',').map(String::from).collect::<Vec<_>>();
                } else if one.contains(">=") {
                    item.op = Op::Ge;
                    item.str_value = v[1].to_owned();
                } else if one.contains("<=") {
                    item.op = Op::Le;
                    item.str_value = v[1].to_owned();
                } else if one.contains('=') {
                    item.op = Op::Equal;
                    item.str_values = v[1].split(',').map(String::from).collect::<Vec<_>>();
                } else if one.contains('>') {
                    item.op = Op::Gt;
                    item.str_value = v[1].to_owned();
                } else if one.contains('<') {
                    item.op = Op::Lt;
                    item.str_value = v[1].to_owned()
                }
            }
        }

        self.filters.push(item);
    }

    // todo
    pub fn record_is_valid<T: AsRef<str>>(&self, row: &[T]) -> bool {
        self.filters.iter().all(|item| item.record_is_valid(row))
    }

    pub fn record_valid_map<'b>(
        &self,
        row: &'b str,
        sep: char,
        quote: char,
    ) -> Option<(Option<&'b str>, Option<Vec<&'b str>>)> {
        if self.is_empty() {
            return Some((Some(row), None));
        }

        let v = CsvRowSplitter::new(row, sep, quote).collect::<Vec<_>>();
        if self.record_is_valid(&v) {
            Some((Some(row), Some(v)))
        } else {
            None
        }
    }

    pub fn excel_record_is_valid<T: AsRef<str>>(&self, row: &[T]) -> bool {
        if self.is_empty() {
            return true;
        }
        self.filters.iter().all(|item| item.record_is_valid(row))
    }
}

pub fn parse_f64(s: &str) -> f64 {
    s.parse::<f64>().unwrap_or_else(|_| {
        werr_exit!("Error: <{s}> is not a valid number, run <rsv select -h> for help.");
    })
}

fn parse_f64_vec(s: &str) -> Vec<f64> {
    s.split(',')
        .map(|i| {
            i.parse::<f64>().unwrap_or_else(|_| {
                werr_exit!("Error: <{i}> is not a number, run <rsv select -h> for help.")
            })
        })
        .collect()
}

impl FilterItem {
    fn record_is_valid<T: AsRef<str>>(&self, row: &[T]) -> bool {
        match (self.is_math_expr, self.is_numeric, &self.op) {
            (true, _, _) => match row[self.col].as_ref().parse::<f64>() {
                Ok(v) => match self.ast.max_column() {
                    0 => self.op.evaluate(v, self.ast.evaluate(None)),
                    _ => {
                        let f64_vec: Vec<f64> = (0..=self.ast.max_column())
                            .map(|i| match self.ast.contains_column(&i) {
                                true => parse_f64(row[i].as_ref()),
                                false => 0.0,
                            })
                            .collect();
                        self.op.evaluate(v, self.ast.evaluate(Some(&f64_vec)))
                    }
                },
                Err(_) => false,
            },
            (false, true, Op::Equal) => match row[self.col].as_ref().parse::<f64>() {
                Ok(v) => self.f64_values.contains(&v),
                Err(_) => false,
            },
            (false, true, Op::NotEqual) => match row[self.col].as_ref().parse::<f64>() {
                Ok(v) => !self.f64_values.contains(&v),
                Err(_) => true,
            },
            (false, true, _) => match row[self.col].as_ref().parse::<f64>() {
                Ok(v) => self.op.evaluate(v, self.f64_value),
                Err(_) => false,
            },
            (false, false, Op::Equal) => self
                .str_values
                .iter()
                .any(|i| i.as_str() == row[self.col].as_ref()),
            (false, false, Op::NotEqual) => !self
                .str_values
                .iter()
                .any(|i| i.as_str() == row[self.col].as_ref()),
            (false, false, Op::Ge) => row[self.col].as_ref() >= &self.str_value,
            (false, false, Op::Gt) => row[self.col].as_ref() > &self.str_value,
            (false, false, Op::Le) => row[self.col].as_ref() <= &self.str_value,
            (false, false, Op::Lt) => row[self.col].as_ref() < &self.str_value,
        }
    }
}
