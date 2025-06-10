use crate::args::Headers;
use crate::utils::return_result::{CliResultData, ResultData};
use std::fs::File;
use std::io::{BufRead, BufReader};

impl Headers {
    #[allow(dead_code)]
    pub fn csv_run_lib(&self) -> CliResultData {
        let mut out = ResultData::new();

        // open file and header
        let mut rdr = BufReader::new(File::open(self.path())?).lines();

        out.insert_header(vec!["column_name".to_string()]);
        if let Some(r) = rdr.next() {
            self.split(&r?)
                .for_each(|v| out.insert_record(vec![v.to_string()]));
        };

        Ok(Some(out))
    }
}
