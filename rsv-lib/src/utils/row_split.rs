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
    iter: Peekable<CharIndices<'a>>,
    sep: char,
    quote: char,
}

impl<'a> CsvRowSplitter<'a> {
    pub fn new(row: &'a str, sep: char, quote: char) -> CsvRowSplitter<'a> {
        CsvRowSplitter {
            row: row,
            iter: row.char_indices().peekable(),
            sep: sep,
            quote: quote,
        }
    }

    fn next_char_is_none_or_sep(&mut self) -> bool {
        match self.iter.peek() {
            None => true,
            Some((_, v)) => v == &self.sep,
        }
    }

    fn get_field(&self, i: usize, j: usize) -> &'a str {
        unsafe { self.row.get_unchecked(i..j) }
    }

    pub fn collect_owned(self) -> Vec<String> {
        self.map(String::from).collect()
    }
}

impl<'a> Iterator for CsvRowSplitter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        // First char of field
        // 1. for the first field, first char is not separator
        // 2. for other fields, first char is separator
        let Some((mut start_index, mut first_char)) = self.iter.next() else {
            return None;
        };

        // For first-field-empty string like ",1,2,3"
        if start_index == 0 && first_char == self.sep {
            return Some("");
        }

        // Field may start with a separator that should be escaped
        // Parsing chain: "1,2,,3" => "1" -> ",2" -> "," -> ",3"
        if first_char == self.sep {
            match self.iter.peek() {
                None => return Some(""),
                Some(&(index, c)) => {
                    // empty field
                    if c == self.sep {
                        return Some("");
                    }

                    // true field start
                    self.iter.next();
                    start_index = index;
                    first_char = c;
                }
            }
        }

        // Case 1: The field is quoted
        if first_char == self.quote {
            while let Some((i, c)) = self.iter.next() {
                if c == self.quote {
                    if self.next_char_is_none_or_sep() {
                        return Some(self.get_field(start_index + 1, i));
                    }
                } else if c == '\\' {
                    self.iter.next();
                }
            }
        }

        // Case 2: The field is not quoted
        while let Some(&(i, c)) = self.iter.peek() {
            if c == self.sep {
                return Some(self.get_field(start_index, i));
            }
            self.iter.next();
        }

        Some(self.get_field(start_index, self.row.len()))
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

        let r = "1,2,,3";
        let o = CsvRowSplitter::new(&r, ',', '"').collect::<Vec<_>>();
        assert_eq!(o, vec!["1", "2", "", "3"]);

        let r = r#"1,2,3,"""#;
        let o = CsvRowSplitter::new(&r, ',', '"').collect::<Vec<_>>();
        assert_eq!(o, vec!["1", "2", "3", ""]);

        let r = r#""",1,2,3,"",4"#;
        let o = CsvRowSplitter::new(&r, ',', '"').collect::<Vec<_>>();
        assert_eq!(o, vec!["", "1", "2", "3", "", "4"]);

        let r = r#",1,2,3,"",4"#;
        let o = CsvRowSplitter::new(&r, ',', '"').collect::<Vec<_>>();
        assert_eq!(o, vec!["", "1", "2", "3", "", "4"]);

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
        assert_eq!(o, vec!["first", "second", "third,fourth", "fifth"]);

        let r = r#"first,second,"third,fourth","fifth""#;
        let o = CsvRowSplitter::new(&r, ',', '"').collect::<Vec<_>>();
        assert_eq!(o, vec!["first", "second", "third,fourth", "fifth"]);

        let r = r#""third,fourth","fifth""#;
        let o = CsvRowSplitter::new(&r, ',', '"').collect::<Vec<_>>();
        assert_eq!(o, vec!["third,fourth", "fifth"]);

        // double-quote in field,, escaped by a preceding \
        let r = r#"third\",fourth,"fifth""#;
        let o = CsvRowSplitter::new(&r, ',', '"').collect::<Vec<_>>();
        assert_eq!(o, vec![r#"third\""#, "fourth", "fifth"]);

        let r = r#""Charles ""Pretty Boy"" Floyd","1 Short St, Smallville""#;
        let o = CsvRowSplitter::new(&r, ',', '"').collect::<Vec<_>>();
        assert_eq!(
            o,
            vec![r#"Charles ""Pretty Boy"" Floyd"#, "1 Short St, Smallville"]
        );
    }
}
