use crate::args::Size;
use rsv_lib::utils::cli_result::CliResult;
use std::fs::File;

impl Size {
    pub fn csv_run(&self) -> CliResult {
        let file = File::open(self.path())?;
        let filesize_bytes = file.metadata()?.len() as f64;

        let filesize_kb = filesize_bytes / 1024.0;
        let filesize_mb = filesize_bytes / (1024.0 * 1024.0);
        let filesize_gb = filesize_bytes / (1024.0 * 1024.0 * 1024.0);

        if filesize_gb >= 1.0 {
            println!("File Size: {:.2} GB", filesize_gb);
        } else if filesize_mb >= 1.0 {
            println!("File Size: {:.2} MB", filesize_mb);
        } else {
            println!("File Size: {:.2} KB", filesize_kb);
        }

        Ok(())
    }
}
