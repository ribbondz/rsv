pub type CliResultData = Result<Option<ResultData>, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub struct ResultData {
    pub header: Vec<String>,
    pub data: Vec<Vec<String>>,
}

impl ResultData {
    pub fn new() -> ResultData {
        ResultData {
            header: vec![],
            data: vec![],
        }
    }

    pub fn from(header: Vec<String>, data: Vec<Vec<String>>) -> ResultData {
        ResultData { header, data }
    }

    pub fn from_header(header: Vec<String>) -> ResultData {
        ResultData {
            header,
            data: vec![],
        }
    }

    pub fn insert_header(&mut self, header: Vec<String>) {
        self.header = header;
    }

    pub fn insert_record(&mut self, record: Vec<String>) {
        self.data.push(record);
    }

    pub fn insert_records(&mut self, records: impl Iterator<Item=Vec<String>>) {
        self.data.extend(records);
    }
}
