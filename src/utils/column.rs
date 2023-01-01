pub struct Columns {
    cols: Vec<usize>,
    max_col: usize,
    pub all: bool,
}

fn parse_col(col: &str) -> usize {
    col.parse().unwrap_or_else(|_| {
        panic!(
            "{}",
            "Column syntax error, can only be 0,1,2 or 0-2, including 2"
        )
    })
}

impl Columns {
    pub fn new(raw: &str) -> Self {
        let mut cols = Columns {
            cols: vec![],
            max_col: 0,
            all: true,
        };

        if raw.is_empty() {
            return cols;
        }

        if raw.contains("-") {
            let segs = raw.split("-").collect::<Vec<_>>();
            let min = parse_col(segs[0]);
            let max = parse_col(segs[1]);
            cols.add_cols((min..=max).collect::<Vec<_>>())
        } else {
            let segs = raw.split(",").map(|i| parse_col(i)).collect::<Vec<_>>();
            cols.add_cols(segs);
        }

        cols.update_status();

        cols
    }

    fn add_cols(&mut self, cols: Vec<usize>) {
        self.cols.extend(cols);
    }

    fn update_status(&mut self) {
        self.max_col = *self.cols.iter().max().unwrap();
        self.all = self.cols.len() == 0;
    }

    pub fn iter(&self) -> impl Iterator<Item = &usize> {
        self.cols.iter()
    }

    pub fn max(&self) -> usize {
        self.max_col
    }

    pub fn _select(&self, all: &Vec<&str>) -> Vec<String> {
        self.cols.iter().map(|&i| all[i].to_owned()).collect()
    }

    pub fn select_and_append_n(&self, all: &Vec<&str>) -> Vec<String> {
        self.cols
            .iter()
            .map(|&i| all[i].to_owned())
            .chain(std::iter::once("n".to_owned()))
            .collect::<Vec<_>>()
    }
}
