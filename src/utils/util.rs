use chrono;

pub fn datetime_str() -> String {
    let t = chrono::offset::Local::now();
    t.format("%Y%m%d%H%M%S").to_string()
}

pub fn is_null(s: &str) -> bool {
    s.is_empty() || s == "NA" || s == "Na" || s == "na" || s == "NULL" || s == "Null" || s == "null"
}
