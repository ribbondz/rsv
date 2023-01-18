use std::process;

use calamine::DataType;

use crate::utils::util::werr;

struct FilterItem {
    col: usize,
    values: Vec<String>,
}

pub struct Filter {
    items: Vec<FilterItem>,
}

impl Filter {
    pub fn is_empty(&self) -> bool {
        self.items.len() == 0
    }

    pub fn new(raw: &str) -> Self {
        let mut f = Filter { items: vec![] };

        if raw.is_empty() {
            return f;
        }

        raw.split('&').for_each(|one| Filter::parse(&mut f, one));

        f
    }

    fn parse(f: &mut Filter, one: &str) {
        let v = one.split('=').collect::<Vec<_>>();

        if v.len() != 2 {
            werr!("Error: Filter syntax can only be 0=a,b,c or 0=a,b&2=c.");
            process::exit(1);
        }

        let col: usize = v[0].parse().unwrap_or_else(|_| {
            werr!("Error: column should be an integer bigger than or equal to 0.");
            process::exit(1)
        });
        let values = v[1].split(',').map(|i| i.to_owned()).collect::<Vec<_>>();

        f.append(col, values);
    }

    pub fn append(&mut self, col: usize, values: Vec<String>) {
        self.items.push(FilterItem { col, values })
    }

    // todo
    pub fn record_is_valid(&self, row: &[&str]) -> bool {
        self.items
            .iter()
            .all(|item| item.values.iter().any(|i| i.as_str() == row[item.col]))
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

        self.items
            .iter()
            .all(|item| item.values.iter().any(|i| i.as_str() == row[item.col]))
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
