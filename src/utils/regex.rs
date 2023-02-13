use calamine::DataType;
use regex::{Regex, RegexBuilder};
use std::error::Error;

pub struct Re(Regex);

impl Re {
    pub fn new(pattern: &str) -> Result<Self, Box<dyn Error>> {
        let re = RegexBuilder::new(pattern).case_insensitive(true).build()?;

        Ok(Re(re))
    }

    pub fn is_match(&self, v: &str) -> bool {
        self.0.is_match(v)
    }

    pub fn verify_excel_row_map(&self, r: Vec<DataType>) -> Option<Vec<String>> {
        let v = r
            .into_iter()
            .map(|j| match j {
                DataType::String(s) => s,
                _ => j.to_string(),
            })
            .collect::<Vec<_>>();
        match v.iter().any(|i| self.is_match(i)) {
            true => Some(v),
            false => None,
        }
    }
}
