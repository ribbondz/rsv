use crate::args::Size;
use crate::utils::return_result::{CliResultData, ResultData};
use std::fs::File;

impl Size {
    #[allow(dead_code)]
    pub fn csv_run_lib(&self) -> CliResultData {
        let mut out = ResultData::new();

        let file = File::open(self.path())?;
        let filesize_bytes = file.metadata()?.len() as f64;

        let filesize_kb = filesize_bytes / 1024.0;
        let filesize_mb = filesize_bytes / (1024.0 * 1024.0);
        let filesize_gb = filesize_bytes / (1024.0 * 1024.0 * 1024.0);

        let size = if filesize_gb >= 1.0 {
            format!("{:.2} GB", filesize_gb)
        } else if filesize_mb >= 1.0 {
            format!("File Size: {:.2} MB", filesize_mb)
        } else {
            format!("File Size: {:.2} KB", filesize_kb)
        };

        out.insert_header(vec!["size".to_string()]);
        out.insert_record(vec![size]);

        Ok(Some(out))
    }
}
