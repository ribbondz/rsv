pub enum RsvData {
    Str(String),
    Int(i64),
    Float64(f64),
}

pub struct ResultData {
    header: Vec<String>,
    data: Vec<Vec<RsvData>>,
}

impl ResultData {
    pub fn insert_header(&mut self, header: Vec<String>) {
        self.header = header;
    }

    pub fn insert_record(&mut self, record: Vec<RsvData>) {
        self.data.push(record);
    }
}
