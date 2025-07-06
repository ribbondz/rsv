use crate::utils::filename::full_path;
use crate::utils::return_result::{CliResultData, ResultData};
use std::fs::File;

pub fn file_size(file: &str) -> CliResultData {
    let file = full_path(file);

    let file = File::open(file)?;
    let filesize_bytes = file.metadata()?.len() as f64;

    let out = ResultData::from(
        vec!["size".to_string()],
        vec![vec![filesize_bytes.to_string()]],
    );

    Ok(Some(out))
}
