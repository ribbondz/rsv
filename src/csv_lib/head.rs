use crate::args::Head;
use crate::utils::return_result::{CliResultData, ResultData};
use std::fs::File;
use std::io::{BufRead, BufReader};

impl Head {
    #[allow(dead_code)]
    pub fn csv_run_lib(&self) -> CliResultData {
        let mut out = ResultData::new();
        let path = self.path();

        // show head n
        let mut lines = BufReader::new(File::open(path)?)
            .lines()
            .take(self.n + 1 - self.no_header as usize);

        // Process header
        if let Some(header) = lines.next() {
            if let Ok(h) = header {
                let h = self.split_row_to_owned_vec(&h);
                out.insert_header(h);
            }
        }

        lines.for_each(|r| {
            if let Ok(r) = r {
                let r = self.split_row_to_owned_vec(&r);
                out.insert_record(r);
            }
        });

        Ok(Some(out))
    }
}
