use super::{cli_result::CliResult, writer::Writer};
use crate::utils::util::werr;
use rayon::prelude::*;
use std::process;

pub struct SortColumns(Vec<SortColumn>);

pub struct SortColumn {
    col: usize,
    ascending: bool,
    pub numeric: bool,
}

impl SortColumns {
    pub fn from(cols: &str) -> Self {
        let mut r = SortColumns(vec![]);

        cols.split(',').for_each(|i| {
            let mut j = i.replace(' ', "");
            if j.is_empty() {
                return;
            }
            (0..2).for_each(|_| {
                if j.ends_with(['n', 'N', 'd', 'D']) {
                    j.pop();
                }
            });

            let col = j.parse::<usize>().unwrap_or_else(|_| {
                werr!(
                    "{}",
                    "Error: column syntax should be something like:
-c 0: first column ascending,
-c 0D: first column descending, 
-c 0DN: first column as numeric values,
-c 0,2N: multiple columns.

The command supports sorting by at most two columns."
                );
                process::exit(1)
            });
            r.0.push(SortColumn {
                col,
                ascending: !i.contains(['d', 'D']),
                numeric: i.contains(['n', 'N']),
            });
        });

        if r.0.is_empty() {
            werr!("{}", "Error: no column is specified.");
            process::exit(1)
        }

        if r.0.len() > 2 {
            werr!(
                "{}",
                "Error: sort by more than two columns is not supported."
            );
            process::exit(1)
        }

        r
    }

    fn col_at(&self, n: usize) -> usize {
        self.0[n].col
    }

    fn ascending_at(&self, n: usize) -> bool {
        self.0[n].ascending
    }

    fn numeric_at(&self, n: usize) -> bool {
        self.0[n].numeric
    }

    pub fn sort_and_write(&self, lines: &Vec<String>, sep: &str, wtr: &mut Writer) -> CliResult {
        match self.0.len() {
            1 => match self.numeric_at(0) {
                true => self.sort_numeric_column(lines, sep, wtr),
                false => self.sort_str_column(lines, sep, wtr),
            },
            2 => match (self.numeric_at(0), self.numeric_at(1)) {
                (true, true) => self.sort_numeric_numeric_columns(lines, sep, wtr),
                (true, false) => self.sort_numeric_str_columns(lines, sep, wtr),
                (false, true) => self.sort_str_numeric_columns(lines, sep, wtr),
                (false, false) => self.sort_str_str_columns(lines, sep, wtr),
            },
            _ => {}
        }

        Ok(())
    }

    fn sort_str_column(&self, lines: &Vec<String>, sep: &str, wtr: &mut Writer) {
        let c = self.col_at(0);
        let mut r = lines
            .par_iter()
            .map(|i| (i, i.split(sep).nth(c).unwrap_or_default()))
            .collect::<Vec<_>>();
        match self.ascending_at(0) {
            true => r.sort_by(|&a, &b| a.1.cmp(b.1)),
            false => r.sort_by(|&a, &b| b.1.cmp(a.1)),
        }

        r.iter().for_each(|(l, _)| wtr.write_line_unchecked(l));
    }

    fn sort_numeric_column(&self, lines: &Vec<String>, sep: &str, wtr: &mut Writer) {
        let c = self.col_at(0);
        let mut r = lines
            .par_iter()
            .map(|i| {
                let f = i.split(sep).nth(c).unwrap_or_default();
                (i, f.parse::<f64>().unwrap_or_default())
            })
            .collect::<Vec<_>>();
        match self.ascending_at(0) {
            true => r.sort_by(|&a, &b| a.1.partial_cmp(&b.1).unwrap()),
            false => r.sort_by(|&a, &b| b.1.partial_cmp(&a.1).unwrap()),
        };

        r.iter().for_each(|(l, _)| wtr.write_line_unchecked(l));
    }

    fn sort_str_str_columns(&self, lines: &Vec<String>, sep: &str, wtr: &mut Writer) {
        let c1 = self.col_at(0);
        let c2 = self.col_at(1);

        let mut r = lines
            .par_iter()
            .map(|i| {
                let f = i.split(sep).collect::<Vec<_>>();
                (i, f[c1], f[c2])
            })
            .collect::<Vec<_>>();
        match (self.ascending_at(0), self.ascending_at(1)) {
            (true, true) => r.sort_by(|&a, &b| a.1.cmp(b.1).then(a.2.cmp(b.2))),
            (true, false) => r.sort_by(|&a, &b| a.1.cmp(b.1).then(b.2.cmp(a.2))),
            (false, true) => r.sort_by(|&a, &b| b.1.cmp(a.1).then(a.2.cmp(b.2))),
            (false, false) => r.sort_by(|&a, &b| b.1.cmp(a.1).then(b.2.cmp(a.2))),
        }

        r.iter().for_each(|(l, _, _)| wtr.write_line_unchecked(l));
    }

    fn sort_str_numeric_columns(&self, lines: &Vec<String>, sep: &str, wtr: &mut Writer) {
        let c1 = self.col_at(0);
        let c2 = self.col_at(1);

        let mut r = lines
            .par_iter()
            .map(|i| {
                let f = i.split(sep).collect::<Vec<_>>();
                (i, f[c1], f[c2].parse::<f64>().unwrap_or_default())
            })
            .collect::<Vec<_>>();
        match (self.ascending_at(0), self.ascending_at(1)) {
            (true, true) => r.sort_by(|&a, &b| a.1.cmp(b.1).then(a.2.partial_cmp(&b.2).unwrap())),
            (true, false) => r.sort_by(|&a, &b| a.1.cmp(b.1).then(b.2.partial_cmp(&a.2).unwrap())),
            (false, true) => r.sort_by(|&a, &b| b.1.cmp(a.1).then(a.2.partial_cmp(&b.2).unwrap())),
            (false, false) => r.sort_by(|&a, &b| b.1.cmp(a.1).then(b.2.partial_cmp(&a.2).unwrap())),
        }

        r.iter().for_each(|(l, _, _)| wtr.write_line_unchecked(l));
    }

    fn sort_numeric_str_columns(&self, lines: &Vec<String>, sep: &str, wtr: &mut Writer) {
        let c1 = self.col_at(0);
        let c2 = self.col_at(1);

        let mut r = lines
            .par_iter()
            .map(|i| {
                let f = i.split(sep).collect::<Vec<_>>();
                (i, f[c1].parse::<f64>().unwrap_or_default(), f[c2])
            })
            .collect::<Vec<_>>();
        match (self.ascending_at(0), self.ascending_at(1)) {
            (true, true) => r.sort_by(|&a, &b| a.1.partial_cmp(&b.1).unwrap().then(a.2.cmp(b.2))),
            (true, false) => r.sort_by(|&a, &b| a.1.partial_cmp(&b.1).unwrap().then(b.2.cmp(a.2))),
            (false, true) => r.sort_by(|&a, &b| b.1.partial_cmp(&a.1).unwrap().then(a.2.cmp(b.2))),
            (false, false) => r.sort_by(|&a, &b| b.1.partial_cmp(&a.1).unwrap().then(b.2.cmp(a.2))),
        }

        r.iter().for_each(|(l, _, _)| wtr.write_line_unchecked(l));
    }

    fn sort_numeric_numeric_columns(&self, lines: &Vec<String>, sep: &str, wtr: &mut Writer) {
        let c1 = self.col_at(0);
        let c2 = self.col_at(1);

        let mut r = lines
            .par_iter()
            .map(|i| {
                let f = i.split(sep).collect::<Vec<_>>();
                (
                    i,
                    f[c1].parse::<f64>().unwrap_or_default(),
                    f[c2].parse::<f64>().unwrap_or_default(),
                )
            })
            .collect::<Vec<_>>();

        match (self.ascending_at(0), self.ascending_at(1)) {
            (true, true) => r.sort_by(|&a, &b| {
                a.1.partial_cmp(&b.1)
                    .unwrap()
                    .then(a.2.partial_cmp(&b.2).unwrap())
            }),
            (true, false) => r.sort_by(|&a, &b| {
                a.1.partial_cmp(&b.1)
                    .unwrap()
                    .then(b.2.partial_cmp(&a.2).unwrap())
            }),
            (false, true) => r.sort_by(|&a, &b| {
                b.1.partial_cmp(&a.1)
                    .unwrap()
                    .then(a.2.partial_cmp(&b.2).unwrap())
            }),
            (false, false) => r.sort_by(|&a, &b| {
                b.1.partial_cmp(&a.1)
                    .unwrap()
                    .then(b.2.partial_cmp(&a.2).unwrap())
            }),
        }

        r.iter().for_each(|(l, _, _)| wtr.write_line_unchecked(l));
    }

    pub fn sort_excel_and_write(
        &self,
        lines: &mut Vec<Vec<String>>,
        wtr: &mut Writer,
    ) -> CliResult {
        match self.0.len() {
            1 => match self.numeric_at(0) {
                true => self.sort_excel_numeric_column(lines, wtr),
                false => self.sort_excel_str_column(lines, wtr),
            },
            2 => match (self.numeric_at(0), self.numeric_at(1)) {
                (true, true) => self.sort_excel_numeric_numeric_columns(lines, wtr),
                (true, false) => self.sort_excel_numeric_str_columns(lines, wtr),
                (false, true) => self.sort_excel_str_numeric_columns(lines, wtr),
                (false, false) => self.sort_excel_str_str_columns(lines, wtr),
            },
            _ => {}
        }

        Ok(())
    }

    fn sort_excel_str_column(&self, lines: &mut [Vec<String>], wtr: &mut Writer) {
        let c = self.col_at(0);
        match self.ascending_at(0) {
            true => lines.sort_by(|a, b| a[c].cmp(&b[c])),
            false => lines.sort_by(|a, b| b[c].cmp(&a[c])),
        }

        lines
            .iter()
            .for_each(|l| wtr.write_line_by_field_unchecked(l, None));
    }

    fn sort_excel_numeric_column(&self, lines: &mut Vec<Vec<String>>, wtr: &mut Writer) {
        let c = self.col_at(0);
        let mut r = lines
            .par_iter()
            .map(|i| (i, i[c].parse::<f64>().unwrap_or_default()))
            .collect::<Vec<_>>();
        match self.ascending_at(0) {
            true => r.sort_by(|&a, &b| a.1.partial_cmp(&b.1).unwrap()),
            false => r.sort_by(|&a, &b| b.1.partial_cmp(&a.1).unwrap()),
        };

        r.iter()
            .for_each(|(l, _)| wtr.write_line_by_field_unchecked(l, None));
    }

    fn sort_excel_str_str_columns(&self, lines: &mut [Vec<String>], wtr: &mut Writer) {
        let c1 = self.col_at(0);
        let c2 = self.col_at(1);
        match (self.ascending_at(0), self.ascending_at(1)) {
            (true, true) => lines.sort_by(|a, b| a[c1].cmp(&b[c1]).then(a[c2].cmp(&b[c2]))),
            (true, false) => lines.sort_by(|a, b| a[c1].cmp(&b[c1]).then(b[c2].cmp(&a[c2]))),
            (false, true) => lines.sort_by(|a, b| b[c1].cmp(&a[c1]).then(a[c2].cmp(&b[c2]))),
            (false, false) => lines.sort_by(|a, b| b[c1].cmp(&a[c1]).then(b[c2].cmp(&a[c2]))),
        }

        lines
            .iter()
            .for_each(|l| wtr.write_line_by_field_unchecked(l, None));
    }

    fn sort_excel_str_numeric_columns(&self, lines: &mut Vec<Vec<String>>, wtr: &mut Writer) {
        let c1 = self.col_at(0);
        let c2 = self.col_at(1);

        let mut r = lines
            .par_iter()
            .map(|i| (i, &i[c1], i[c2].parse::<f64>().unwrap_or_default()))
            .collect::<Vec<_>>();
        match (self.ascending_at(0), self.ascending_at(1)) {
            (true, true) => r.sort_by(|&a, &b| a.1.cmp(b.1).then(a.2.partial_cmp(&b.2).unwrap())),
            (true, false) => r.sort_by(|&a, &b| a.1.cmp(b.1).then(b.2.partial_cmp(&a.2).unwrap())),
            (false, true) => r.sort_by(|&a, &b| b.1.cmp(a.1).then(a.2.partial_cmp(&b.2).unwrap())),
            (false, false) => r.sort_by(|&a, &b| b.1.cmp(a.1).then(b.2.partial_cmp(&a.2).unwrap())),
        }

        r.iter()
            .for_each(|(l, _, _)| wtr.write_line_by_field_unchecked(l, None));
    }

    fn sort_excel_numeric_str_columns(&self, lines: &mut [Vec<String>], wtr: &mut Writer) {
        let c1 = self.col_at(0);
        let c2 = self.col_at(1);

        let mut r = lines
            .par_iter()
            .map(|i| (i, i[c1].parse::<f64>().unwrap_or_default(), &(i[c2])))
            .collect::<Vec<_>>();
        match (self.ascending_at(0), self.ascending_at(1)) {
            (true, true) => r.sort_by(|&a, &b| a.1.partial_cmp(&b.1).unwrap().then(a.2.cmp(b.2))),
            (true, false) => r.sort_by(|&a, &b| a.1.partial_cmp(&b.1).unwrap().then(b.2.cmp(a.2))),
            (false, true) => r.sort_by(|&a, &b| b.1.partial_cmp(&a.1).unwrap().then(a.2.cmp(b.2))),
            (false, false) => r.sort_by(|&a, &b| b.1.partial_cmp(&a.1).unwrap().then(b.2.cmp(a.2))),
        }

        r.iter()
            .for_each(|(l, _, _)| wtr.write_line_by_field_unchecked(l, None));
    }

    fn sort_excel_numeric_numeric_columns(&self, lines: &mut [Vec<String>], wtr: &mut Writer) {
        let c1 = self.col_at(0);
        let c2 = self.col_at(1);

        let mut r = lines
            .par_iter()
            .map(|i| {
                (
                    i,
                    i[c1].parse::<f64>().unwrap_or_default(),
                    i[c2].parse::<f64>().unwrap_or_default(),
                )
            })
            .collect::<Vec<_>>();

        match (self.ascending_at(0), self.ascending_at(1)) {
            (true, true) => r.sort_by(|&a, &b| {
                a.1.partial_cmp(&b.1)
                    .unwrap()
                    .then(a.2.partial_cmp(&b.2).unwrap())
            }),
            (true, false) => r.sort_by(|&a, &b| {
                a.1.partial_cmp(&b.1)
                    .unwrap()
                    .then(b.2.partial_cmp(&a.2).unwrap())
            }),
            (false, true) => r.sort_by(|&a, &b| {
                b.1.partial_cmp(&a.1)
                    .unwrap()
                    .then(a.2.partial_cmp(&b.2).unwrap())
            }),
            (false, false) => r.sort_by(|&a, &b| {
                b.1.partial_cmp(&a.1)
                    .unwrap()
                    .then(b.2.partial_cmp(&a.2).unwrap())
            }),
        }

        r.iter()
            .for_each(|(l, _, _)| wtr.write_line_by_field_unchecked(l, None));
    }
}
