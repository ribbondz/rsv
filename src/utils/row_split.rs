use std::str::CharIndices;

#[derive(Debug)]
pub struct CsvRow<'a> {
    row: &'a str,
}

#[derive(Debug)]
pub struct CsvRowSplit<'a> {
    row: &'a str,
    char_indices: CharIndices<'a>,
    sep: char,
    quote: char,
    done: bool,
    field_start: usize,
    field_end_shift: usize,
    in_quoted_field: bool,
    has_separator: bool,
}

impl<'a> CsvRow<'a> {
    pub fn new(row: &'a str) -> Self {
        CsvRow { row }
    }

    pub fn split(self, sep: char, quote: char) -> CsvRowSplit<'a> {
        CsvRowSplit {
            row: self.row,
            char_indices: self.row.char_indices(),
            sep,
            quote,
            done: false,
            field_start: 0,
            field_end_shift: 0,
            in_quoted_field: false,
            has_separator: false, // whether a field has a CSV sep in it
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
            if let Some((index, c)) = self.char_indices.next() {
                if c == '\\' {
                    self.char_indices.next();
                } else if c == self.sep {
                    if self.in_quoted_field {
                        self.has_separator = true;
                    } else {
                        let has_sep = self.has_separator as usize;
                        let i = self.field_start - has_sep;
                        let j = index - self.field_end_shift + has_sep;
                        let f = unsafe { self.row.get_unchecked(i..j) };

                        self.field_start = index + 1;
                        self.field_end_shift = 0;
                        self.has_separator = false;
                        return Some(f);
                    }
                } else if c == self.quote {
                    let i = self.in_quoted_field as usize;
                    self.field_start += i;
                    self.field_end_shift += i;

                    self.in_quoted_field = !self.in_quoted_field;
                }
            } else {
                self.done = true;
                let has_sep = self.has_separator as usize;
                let i = self.field_start - has_sep;
                let j = self.row.len() - self.field_end_shift + has_sep;
                let f = unsafe { self.row.get_unchecked(i..j) };
                return Some(f);
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
        let r = "1,2,3,";
        let o = CsvRow::new(&r).split(',', '"').collect::<Vec<_>>();
        assert_eq!(o, vec!["1", "2", "3", ""]);

        let r = "\"1\",2,3,";
        let o = CsvRow::new(&r).split(',', '"').collect::<Vec<_>>();
        assert_eq!(o, vec!["1", "2", "3", ""]);

        let r = "first,second,\"third,fourth\",fifth";
        let o = CsvRow::new(&r).split(',', '"').collect::<Vec<_>>();
        assert_eq!(o, vec!["first", "second", "\"third,fourth\"", "fifth"]);

        let r = "first,second,\"third,fourth\",\"fifth\"";
        let o = CsvRow::new(&r).split(',', '"').collect::<Vec<_>>();
        assert_eq!(o, vec!["first", "second", "\"third,fourth\"", "fifth"]);

        let r = "\"third,fourth\",\"fifth\"";
        let o = CsvRow::new(&r).split(',', '"').collect::<Vec<_>>();
        assert_eq!(o, vec!["\"third,fourth\"", "fifth"]);

        let r = "我们abc,def,12";
        let o = CsvRow::new(&r).split(',', '"').collect::<Vec<_>>();
        assert_eq!(o, vec!["我们abc", "def", "12"]);

        // double-quote in field
        let r = "\"third\\\",fourth\",\"fifth\"";
        let o = CsvRow::new(&r).split(',', '"').collect::<Vec<_>>();
        assert_eq!(o, vec!["\"third\\\",fourth\"", "fifth"]);
    }
}
