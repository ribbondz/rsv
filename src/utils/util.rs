use chrono;

pub fn datetime_str() -> String {
    let t = chrono::offset::Local::now();
    t.format("%Y%m%d%H%M%S").to_string()
}
