use calamine::Data;
use std::{borrow::Cow, fmt::Write};

pub fn datatype_vec_to_string_vec(data: &[Data]) -> Vec<String> {
    data.iter().map(|i| i.to_string()).collect()
}

pub fn datatype_vec_to_str_vec(data: &[Data]) -> Vec<Cow<str>> {
    data.iter()
        .map(|i| match i {
            Data::String(s) => Cow::Borrowed(s.as_str()),
            _ => Cow::from(i.to_string()),
        })
        .collect()
}

pub fn datatype_vec_to_string(data: &[Data]) -> String {
    let mut o = String::new();
    let mut iter = data.iter().peekable();
    while let Some(i) = iter.next() {
        write_datatype_to_string(&mut o, i);
        if iter.peek().is_some() {
            o.push(',');
        }
    }
    o
}

pub fn write_datatype_to_string(s: &mut String, d: &Data) {
    let _ = match d {
        Data::String(v) => write!(s, "{}", v),
        Data::Float(v) => write!(s, "{}", v),
        Data::Int(v) => write!(s, "{}", v),
        Data::Bool(v) => write!(s, "{}", v),
        Data::DateTime(v) => write!(s, "{}", v),
        Data::DateTimeIso(v) => write!(s, "{}", v),
        Data::DurationIso(v) => write!(s, "{}", v),
        Data::Error(v) => write!(s, "{}", v),
        Data::Empty => Ok(()),
    };
}
