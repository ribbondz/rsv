pub type CliResultData = Result<Option<ResultData>, Box<dyn std::error::Error>>;

pub struct ResultData {
    header: Vec<String>,
    data: Vec<Vec<String>>,
}

impl ResultData {
    pub fn new() -> ResultData {
        ResultData {
            header: vec![],
            data: vec![],
        }
    }

    pub fn insert_header(&mut self, header: Vec<String>) {
        self.header = header;
    }

    pub fn insert_record(&mut self, record: Vec<String>) {
        self.data.push(record);
    }
}
