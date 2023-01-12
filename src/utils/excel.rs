use calamine::DataType;

pub fn datatype_vec_to_string_vec(data: &[DataType]) -> Vec<String> {
    data.iter().map(|i| i.to_string()).collect()
}
