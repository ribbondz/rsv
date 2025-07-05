use crate::args::Size;
use rsv_lib::utils::cli_result::CliResult;
use std::io::{stdin, Read};

impl Size {
    pub fn io_run(&self) -> CliResult {
        let mut buffer = [0; 1024]; // A buffer to read into
        let mut total_bytes_read = 0;

        loop {
            let bytes_read = stdin().read(&mut buffer)?;
            if bytes_read == 0 {
                // End of input
                break;
            }
            total_bytes_read += bytes_read;
        }

        let filesize_kb = total_bytes_read as f64 / 1024.0;
        let filesize_mb = total_bytes_read as f64 / (1024.0 * 1024.0);
        let filesize_gb = total_bytes_read as f64 / (1024.0 * 1024.0 * 1024.0);

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
