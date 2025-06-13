use crate::utils::filename::full_path;
use crate::utils::return_result::{CliResultData, ResultData};
use std::fs::File;

pub fn file_size(file: &str) -> CliResultData {
    let file = full_path(file);

    let file = File::open(file)?;
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

    let out = ResultData::from(vec!["size".to_string()], vec![vec![size]]);

    Ok(Some(out))
}
