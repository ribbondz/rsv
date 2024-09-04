use std::str::Chars;

#[derive(Debug)]
pub struct CsvRow<'a> {
    row: &'a str,
}

#[derive(Debug)]
pub struct CsvRowSplit<'a> {
    row: &'a str,
    chars: Chars<'a>,
    sep: char,
    quote: char,
    done: bool,
    start: usize,
    current_index: usize,
    in_quoted_field: bool,
    is_second_quote: bool,
}

impl<'a> CsvRow<'a> {
    pub fn new(row: &'a str) -> Self {
        CsvRow { row }
    }

    pub fn split(self, sep: char, quote: char) -> CsvRowSplit<'a> {
        CsvRowSplit {
            row: self.row,
            chars: self.row.chars(),
            sep,
            quote,
            done: false,
            start: 0,
            current_index: 0,
            in_quoted_field: false,
            is_second_quote: true,
        }
    }
}

impl<'a> Iterator for CsvRowSplit<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        loop {
            if let Some(c) = self.chars.next() {
                self.current_index += 1;

                if c == self.sep && !self.in_quoted_field {
                    let s = self.start;
                    self.start = self.current_index;
                    return Some(unsafe { self.row.get_unchecked(s..self.current_index - 1) });
                } else if c == self.quote {
                    self.is_second_quote = !self.is_second_quote;
                    self.in_quoted_field = !self.is_second_quote;
                }
            } else {
                self.done = true;
                return Some(unsafe { self.row.get_unchecked(self.start..) });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_csv_row_split() {
        let r1 = "1,2,3,";
        let o1 = CsvRow::new(&r1).split(',', '"').collect::<Vec<_>>();
        assert_eq!(o1, vec!["1", "2", "3", ""]);

        let r2 = "first,second,\"third,fourth\", fifth";
        let o2 = CsvRow::new(&r2).split(',', '"').collect::<Vec<_>>();
        assert_eq!(o2, vec!["first", "second", "\"third,fourth\"", " fifth"]);
    }
}
