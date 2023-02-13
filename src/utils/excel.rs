use calamine::DataType;
use std::fmt::Write;

pub fn datatype_vec_to_string_vec(data: &[DataType]) -> Vec<String> {
    data.iter().map(|i| i.to_string()).collect()
}

pub fn datatype_vec_to_string(data: &[DataType]) -> String {
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

pub fn write_datatype_to_string(s: &mut String, d: &DataType) {
    let _ = match d {
        DataType::String(v) => write!(s, "{}", v),
        DataType::Float(v) => write!(s, "{}", v),
        DataType::Int(v) => write!(s, "{}", v),
        DataType::Bool(v) => write!(s, "{}", v),
        DataType::DateTime(v) => write!(s, "{}", v),
        DataType::Error(v) => write!(s, "{}", v),
        DataType::Empty => Ok(()),
    };
}


