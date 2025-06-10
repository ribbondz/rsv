// CSV row split module that supports:
// 1. double-quoted field
// 2. comma in a double-quoted field
// 3. double-quotes in a field escaped by a backslash \
// 4. double-quotes in a field escaped by a preceding double-quotes as discussed in
// https://stackoverflow.com/questions/17808511/how-to-properly-escape-a-double-quote-in-csv

// worked for examples:
// v1,v2,v3
// "v1","v2","v3"
// "v1",v2,v3
// "Charles \"Pretty Boy\" Floyd","1 Short St, Smallville"
// "Charles ""Pretty Boy"" Floyd","1 Short St, Smallville"

use std::{iter::Peekable, str::CharIndices};

#[derive(Debug)]
pub struct CsvRowSplitter<'a> {
    row: &'a str,
    char_indices: Peekable<CharIndices<'a>>,
    sep: char,
    quote: char,
    parse_done: bool,
    field_start_index: usize,
    field_is_quoted: bool,
    field_has_separator: bool,
    cur_in_quoted_field: bool,
    cur_is_field_start: bool,
}

impl<'a> CsvRowSplitter<'a> {
    pub fn new(row: &'a str, sep: char, quote: char) -> CsvRowSplitter<'a> {
        CsvRowSplitter {
            row: row,
            char_indices: row.char_indices().peekable(),
            sep: sep,
            quote: quote,
            parse_done: false,
            field_start_index: 0,
            field_is_quoted: false,
            field_has_separator: false, // whether a field has a CSV sep within it
            cur_in_quoted_field: false,
            cur_is_field_start: true, // whether current position is the start of a field
        }
    }

    fn field_start_set(&mut self, start_index: usize) {
        self.field_start_index = start_index;
        self.field_is_quoted = false;
        self.field_has_separator = false;
        self.cur_in_quoted_field = false;
        self.cur_is_field_start = true;
    }

    fn get_field(&self, end_index: usize) -> &'a str {
        let field_shift = self.field_is_quoted as usize - self.field_has_separator as usize;
        let i = self.field_start_index + field_shift;
        let j = end_index - field_shift;
        unsafe { self.row.get_unchecked(i..j) }
    }

    fn next_char_is_none_or_sep(&mut self) -> bool {
        match self.char_indices.peek() {
            None => true,
            Some((_, v)) => v == &self.sep,
        }
    }
}

impl<'a> Iterator for CsvRowSplitter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.parse_done {
            return None;
        }

        loop {
            let Some((index, c)) = self.char_indices.next() else {
                // obtain last field
                self.parse_done = true;
                let f = self.get_field(self.row.len());
                return Some(f);
            };

            if c == '\\' {
                // skip \ escape, e.g., v1,v2\",v3 is parsed into ["v1", "v2\"", "v3"]
                self.char_indices.next();
            } else if c == self.sep {
                if self.cur_in_quoted_field {
                    self.field_has_separator = true;
                } else {
                    let f = self.get_field(index);
                    self.field_start_set(index + 1);
                    return Some(f);
                }
            } else if c == self.quote {
                if self.cur_is_field_start {
                    self.field_is_quoted = true;
                    self.cur_in_quoted_field = true;
                } else if self.next_char_is_none_or_sep() {
                    self.cur_in_quoted_field = false;
                } else {
                    // skip double-quotes escape, e.g., v1,v2"",v3 is parsed into ["v1", "v2""", "v3"]
                    self.char_indices.next();
                }
            }

            self.cur_is_field_start = false;
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_csv_row_split() {
        let r = "我们abc,def,12";
        let o = CsvRowSplitter::new(&r, ',', '"').collect::<Vec<_>>();
        assert_eq!(o, vec!["我们abc", "def", "12"]);

        let r = "1,2,3,";
        let o = CsvRowSplitter::new(&r, ',', '"').collect::<Vec<_>>();
        assert_eq!(o, vec!["1", "2", "3", ""]);

        let r = r#"1,2,3,"""#;
        let o = CsvRowSplitter::new(&r, ',', '"').collect::<Vec<_>>();
        assert_eq!(o, vec!["1", "2", "3", ""]);

        let r = r#"1,2,3,"",4"#;
        let o = CsvRowSplitter::new(&r, ',', '"').collect::<Vec<_>>();
        assert_eq!(o, vec!["1", "2", "3", "", "4"]);

        let r = r#"1,2,3,"","4""#;
        let o = CsvRowSplitter::new(&r, ',', '"').collect::<Vec<_>>();
        assert_eq!(o, vec!["1", "2", "3", "", "4"]);

        // quoted field
        let r = r#""1",2,3,"#;
        let o = CsvRowSplitter::new(&r, ',', '"').collect::<Vec<_>>();
        assert_eq!(o, vec!["1", "2", "3", ""]);

        // comma in quoted field
        let r = r#"first,second,"third,fourth",fifth"#;
        let o = CsvRowSplitter::new(&r, ',', '"').collect::<Vec<_>>();
        assert_eq!(o, vec!["first", "second", r#""third,fourth""#, "fifth"]);

        let r = r#"first,second,"third,fourth","fifth""#;
        let o = CsvRowSplitter::new(&r, ',', '"').collect::<Vec<_>>();
        assert_eq!(o, vec!["first", "second", r#""third,fourth""#, "fifth"]);

        let r = r#""third,fourth","fifth""#;
        let o = CsvRowSplitter::new(&r, ',', '"').collect::<Vec<_>>();
        assert_eq!(o, vec![r#""third,fourth""#, "fifth"]);

        // double-quote in field,, escaped by a preceding \
        let r = r#"third\",fourth,"fifth""#;
        let o = CsvRowSplitter::new(&r, ',', '"').collect::<Vec<_>>();
        assert_eq!(o, vec![r#"third\""#, "fourth", "fifth"]);

        let r = r#""Charles ""Pretty Boy"" Floyd","1 Short St, Smallville""#;
        let o = CsvRowSplitter::new(&r, ',', '"').collect::<Vec<_>>();
        assert_eq!(
            o,
            vec![
                r#"Charles ""Pretty Boy"" Floyd"#,
                r#""1 Short St, Smallville""#
            ]
        );
    }
}
