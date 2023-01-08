use std::io;
use std::io::Write;
use std::time::Instant;

use super::constants::{GB_F64, KB_F64, MB_F64};

pub struct Progress {
    pub chunks: usize,
    pub bytes: usize,
    pub print_count: usize,
    start_time: Instant,
}

impl Progress {
    pub fn new() -> Self {
        Progress {
            chunks: 0,
            bytes: 0,
            print_count: 0,
            start_time: Instant::now(),
        }
    }

    pub fn add_chunks(&mut self, n: usize) {
        self.chunks += n;
    }

    pub fn add_bytes(&mut self, n: usize) {
        self.bytes += n;
    }

    pub fn info(&self) -> String {
        let bytes = self.bytes as f64;
        if bytes < MB_F64 {
            format!("{:.2}KB", bytes / KB_F64)
        } else if bytes < GB_F64 {
            format!("{:.2}MB", bytes / MB_F64)
        } else {
            format!("{:.2}GB", bytes / GB_F64)
        }
    }

    pub fn elapsed_time_as_string(&self) -> String {
        let t = self.start_time.elapsed().as_secs_f64();
        if t < 60.0 {
            format!("{} seconds", t as usize)
        } else {
            format!("{:.2} minutes", t / 60.0)
        }
    }

    pub fn print(&mut self) {
        // must have the suffix space, otherwise current line cannot be cleaned completely
        print!(
            "\rchunk: {}, total processed: {}, elapsed time: {}         ",
            self.chunks,
            self.info(),
            self.elapsed_time_as_string()
        );
        io::stdout().flush().unwrap();

        self.print_count += 1;
    }

    pub fn clear(&mut self) {
        // must have the suffix space, otherwise current line cannot be cleaned completely
        print!("\r");
        print!("{}", " ".repeat(60));
        print!("\r");
        io::stdout().flush().unwrap();

        self.print_count += 1;
    }

    pub fn _print_multiple_lines(&mut self) {
        // must have the suffix space, otherwise current line cannot be cleaned completely
        println!(
            "chunk: {}, total processed: {}, elapsed time: {}           ",
            self.chunks,
            self.info(),
            self.elapsed_time_as_string()
        );
        io::stdout().flush().unwrap();

        self.print_count += 1;
    }

    pub fn print_elapsed_time(&mut self) {
        println!("elapsed time: {}     ", self.elapsed_time_as_string());
        io::stdout().flush().unwrap();

        self.print_count += 1;
    }
}
