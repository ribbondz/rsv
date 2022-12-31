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

        raw.split("&").for_each(|i| {
            let segs = i.split("=").collect::<Vec<_>>();

            if segs.len() != 2 {
                panic!("Filter syntax error, can only be 0=a,b,c or 0=a,b&2=c.");
            }

            let col: usize = segs[0].parse().unwrap_or_else(|_| {
                panic!("Column should be an interger bigger than or equal to 0.")
            });
            let values = segs[1].split(",").map(|i| i.to_owned()).collect::<Vec<_>>();

            f.append(col, values);
        });

        f
    }

    pub fn append(&mut self, col: usize, values: Vec<String>) {
        self.items.push(FilterItem { col, values })
    }

    pub fn record_is_valid(&self, row: &Vec<&str>) -> bool {
        if self.is_empty() {
            return true;
        }

        self.items
            .iter()
            .all(|item| item.values.iter().any(|i| i.as_str() == row[item.col]))
    }
}
