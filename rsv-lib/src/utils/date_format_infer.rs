use chrono::{DateTime, NaiveDate, NaiveDateTime};
use regex::Regex;

pub struct DateSmartParser {
    date_fmts: Vec<String>,
    datetime_fmts: Vec<String>,
    re_digits: Regex,
    re_short_offset: Regex,
}

impl Default for DateSmartParser {
    fn default() -> Self {
        Self::new()
    }
}

impl DateSmartParser {
    pub fn new() -> Self {
        DateSmartParser {
            date_fmts: vec![
                "%Y-%m-%d".into(),
                "%m-%d-%Y".into(),
                "%m/%d/%Y".into(),
                "%Y/%m/%d".into(),
                "%d %B %Y".into(),
                "%d %b %Y".into(),
                "%b %d %Y".into(),
                "%b %d %y".into(), // "oct 7 70" (Sanitized . and ,)
                "%B %d %y".into(),
            ],
            datetime_fmts: vec![
                "%m/%d/%Y %H:%M:%S".into(), // "04/2/2014 03:00:51"
                "%m/%d/%Y %H:%M".into(),    // "4/8/2014 22:05"
                // Short Years (2-digit)
                "%m/%d/%y %H:%M".into(),    // "4/8/14 22:05"
                "%m/%d/%y %H:%M:%S".into(), // "4/8/14 22:05:50"
                // Chinese / Separated (Sanitized to dashes/colons)
                "%Y-%m-%d %H:%M:%S".into(), // Handles the sanitized Chinese
                "%y%m%d %H:%M:%S".into(),
                // Standard / ISO / RFC
                "%Y-%m-%d %H:%M:%S%.f %z".into(),  // With offset
                "%Y-%m-%d %H:%M:%S%.f".into(),     // No offset
                "%Y-%m-%dT%H:%M:%S%.fZ".into(),    // RFC3339 strict
                "%a %d %b %Y %H:%M:%S GMT".into(), // RFC2822
                // US / Slash formats
                "%m/%d/%Y %I:%M:%S %p".into(), // "8/8/1965 01:00:01 PM"
                "%m/%d/%Y %I:%M %p".into(),    // "8/8/1965 1:00 PM"
                // Month Names (Sanitization removed commas and 'at')
                "%B %d %Y %I:%M:%S %p".into(), // "May 8 2009 5:57:51 PM"
                "%B %d %Y %I:%M %p".into(),    // "September 17 2012 10:09AM"
                "%b %d %Y %H:%M:%S %z".into(), // "May 02 2021 15:51:31 UTC"
                "%B %d %Y %H:%M:%S %z".into(), // Long month version
                "%b %d %Y %I:%M %p %Z".into(), // "May 26 2021 12:49 AM PDT"
            ],
            re_digits: Regex::new(r"^\d+$").unwrap(),
            re_short_offset: Regex::new(r"([+-]\d{2})$").unwrap(),
        }
    }

    pub(crate) fn smart_parse(&self, input: &str, fmt: Option<&String>) -> Option<NaiveDateTime> {
        let input = input.trim();

        // assigned fmt
        if let Some(f) = fmt {
            if let Ok(d) = NaiveDate::parse_from_str(input, f) {
                return Some(d.and_hms_opt(0, 0, 0).unwrap());
            }

            if let Ok(dt) = NaiveDateTime::parse_from_str(input, f) {
                return Some(dt);
            }
        }

        // 1. Handle Unix Timestamps (All digits)
        if self.re_digits.is_match(input) {
            return parse_unix_timestamp(input);
        }

        // 2. Pre-process/Sanitize Input
        // Replace Chinese markers with standard separators to reduce format complexity
        let mut clean = input
            .replace("年", "-")
            .replace("月", "-")
            .replace("日", " ")
            .replace("时", ":")
            .replace("分", ":")
            .replace("秒", "")
            .replace("at ", "") // "September 17, 2012 at 10:09am"
            .replace(",", "") // Remove commas
            .replace(".", "-") // "2014.03.30" -> "2014-03-30"
            .replace("  ", " ");

        // Normalize "am/pm" to uppercase "AM/PM"
        clean = clean.replace("am", "AM").replace("pm", "PM");

        // Fix non-standard Postgres offsets (e.g., "-08" -> "-0800")
        // If string ends with -XX or +XX, append 00
        if self.re_short_offset.is_match(&clean) {
            clean = format!("{}00", clean);
        }

        // 3. Try Pattern Matching
        // Try parsing as Date only (Assume 00:00:00)
        for fmt in &self.date_fmts {
            if let Ok(d) = NaiveDate::parse_from_str(&clean, fmt) {
                return Some(d.and_hms_opt(0, 0, 0).unwrap());
            }
        }

        // Try parsing as NaiveDateTime (No Offset)
        for fmt in &self.datetime_fmts {
            if let Ok(dt) = NaiveDateTime::parse_from_str(&clean, fmt) {
                return Some(dt);
            }
        }

        None
    }
}

fn parse_unix_timestamp(input: &str) -> Option<NaiveDateTime> {
    let parsed_int = input.parse::<i64>().ok()?;
    match input.len() {
        10 => DateTime::from_timestamp(parsed_int, 0), // Seconds
        13 => DateTime::from_timestamp_millis(parsed_int), // Milliseconds
        19 => DateTime::from_timestamp_micros(parsed_int), // Nanoseconds (approx)
        _ => None,
    }
    .map(|dt| dt.naive_utc())
}
