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
}
