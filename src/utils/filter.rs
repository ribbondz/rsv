use crate::utils::util::werr;
use calamine::DataType;
use regex::Regex;
use std::process;

enum Op {
    Equal,
    NotEqual,
    Gt,
    Ge,
    Lt,
    Le,
}

struct FilterItem {
    col: usize,
    is_numeric: bool,
    op: Op,
    f64_value: f64,
    str_value: String,
    f64_values: Vec<f64>,
    str_values: Vec<String>,
}

pub struct Filter(Vec<FilterItem>);

impl Filter {
    pub fn is_empty(&self) -> bool {
        self.0.len() == 0
    }

    pub fn new(raw: &str) -> Self {
        let mut f = Filter(vec![]);

        if raw.is_empty() {
            return f;
        }

        raw.split('&')
            .filter(|i| !i.is_empty())
            .for_each(|one| f.parse(one));

        f
    }

    fn parse(&mut self, one: &str) {
        let one = one.replace(' ', "");
        // the matching order is important
        let re = Regex::new("!=|>=|<=|=|>|<").unwrap();
        let v = re.split(&one).collect::<Vec<_>>();

        if v.len() != 2 {
            werr!("Error: Filter syntax is wrong, run <rsv select -h> for help.");
            process::exit(1);
        }

        // parse column
        let mut col = v[0].to_owned();
        let is_numeric = col.ends_with(['n', 'N']);
        if is_numeric {
            col.pop();
        }
        let col = col.parse::<usize>().unwrap_or_else(|_| {
            werr!("Error: <{}> is not a column.", col);
            process::exit(1)
        });

        let mut item = FilterItem {
            col,
            is_numeric,
            op: Op::NotEqual,
            f64_value: 0.0,
            f64_values: vec![],
            str_value: "".to_owned(),
            str_values: vec![],
        };

        // parse filter, the matching order is important
        match is_numeric {
            true => {
                if one.contains("!=") {
                    item.op = Op::NotEqual;
                    item.f64_values = str_to_f64_vec(v[1]);
                } else if one.contains(">=") {
                    item.op = Op::Ge;
                    item.f64_value = str_to_f64(v[1]);
                } else if one.contains("<=") {
                    item.op = Op::Le;
                    item.f64_value = str_to_f64(v[1]);
                } else if one.contains('=') {
                    item.op = Op::Equal;
                    item.f64_values = str_to_f64_vec(v[1]);
                } else if one.contains('>') {
                    item.op = Op::Gt;
                    item.f64_value = str_to_f64(v[1]);
                } else if one.contains('<') {
                    item.op = Op::Lt;
                    item.f64_value = str_to_f64(v[1]);
                }
            }
            false => {
                if one.contains("!=") {
                    item.op = Op::NotEqual;
                    item.str_values = v[1].split(',').map(|i| i.to_owned()).collect::<Vec<_>>();
                } else if one.contains(">=") {
                    item.op = Op::Ge;
                    item.str_value = v[1].to_owned();
                } else if one.contains("<=") {
                    item.op = Op::Le;
                    item.str_value = v[1].to_owned();
                } else if one.contains('=') {
                    item.op = Op::Equal;
                    item.str_values = v[1].split(',').map(|i| i.to_owned()).collect::<Vec<_>>();
                } else if one.contains('>') {
                    item.op = Op::Gt;
                    item.str_value = v[1].to_owned();
                } else if one.contains('<') {
                    item.op = Op::Lt;
                    item.str_value = v[1].to_owned()
                }
            }
        }

        self.0.push(item);
    }

    // todo
    pub fn record_is_valid<T: AsRef<str>>(&self, row: &[T]) -> bool {
        self.0.iter().all(|item| item.record_is_valid(row))
    }

    pub fn record_valid_map<'a>(
        &self,
        row: &'a str,
        sep: &str,
    ) -> Option<(Option<&'a str>, Option<Vec<&'a str>>)> {
        if self.is_empty() {
            return Some((Some(row), None));
        }

        let v = row.split(sep).collect::<Vec<_>>();
        if self.record_is_valid(&v) {
            Some((Some(row), Some(v)))
        } else {
            None
        }
    }

    pub fn excel_record_is_valid(&self, row: &[String]) -> bool {
        if self.is_empty() {
            return true;
        }
        self.0.iter().all(|item| item.record_is_valid(row))
    }

    pub fn excel_record_valid_map(&self, row: &[DataType]) -> Option<Vec<String>> {
        let row = row
            .iter()
            .map(|i| i.to_string().trim().to_owned())
            .collect::<Vec<_>>();
        if self.excel_record_is_valid(&row) {
            Some(row)
        } else {
            None
        }
    }
}

fn str_to_f64(s: &str) -> f64 {
    s.parse::<f64>().unwrap_or_else(|_| {
        werr!(
            "Error: <{}> is not a valid number, run <rsv select -h> for help.",
            s
        );
        process::exit(1)
    })
}

fn str_to_f64_vec(s: &str) -> Vec<f64> {
    s.split(',')
        .map(|i| {
            i.parse::<f64>().unwrap_or_else(|_| {
                werr!(
                    "Error: <{}> is not a number, run <rsv select -h> for help.",
                    i
                );
                process::exit(1);
            })
        })
        .collect()
}

impl FilterItem {
    fn record_is_valid<T: AsRef<str>>(&self, row: &[T]) -> bool {
        match (&self.op, self.is_numeric) {
            (Op::Equal, true) => match row[self.col].as_ref().parse::<f64>() {
                Ok(v) => self.f64_values.contains(&v),
                Err(_) => false,
            },
            (Op::NotEqual, true) => match row[self.col].as_ref().parse::<f64>() {
                Ok(v) => !self.f64_values.contains(&v),
                Err(_) => true,
            },
            (Op::Ge, true) => match row[self.col].as_ref().parse::<f64>() {
                Ok(v) => v >= self.f64_value,
                Err(_) => false,
            },
            (Op::Gt, true) => match row[self.col].as_ref().parse::<f64>() {
                Ok(v) => v > self.f64_value,
                Err(_) => false,
            },
            (Op::Le, true) => match row[self.col].as_ref().parse::<f64>() {
                Ok(v) => v <= self.f64_value,
                Err(_) => false,
            },
            (Op::Lt, true) => match row[self.col].as_ref().parse::<f64>() {
                Ok(v) => v < self.f64_value,
                Err(_) => false,
            },
            (Op::Equal, false) => self
                .str_values
                .iter()
                .any(|i| i.as_str() == row[self.col].as_ref()),
            (Op::NotEqual, false) => !self
                .str_values
                .iter()
                .any(|i| i.as_str() == row[self.col].as_ref()),
            (Op::Ge, false) => row[self.col].as_ref() >= &self.str_value,
            (Op::Gt, false) => row[self.col].as_ref() > &self.str_value,
            (Op::Le, false) => row[self.col].as_ref() <= &self.str_value,
            (Op::Lt, false) => row[self.col].as_ref() < &self.str_value,
        }
    }
}
