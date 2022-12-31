use std::io;
use std::io::Write;
use std::time::Instant;

pub struct Progress {
    pub chunks: usize,
    pub bytes: usize,
    pub print_count: usize,
    start_time: Instant,
}

const KB: f64 = 1024.0;
const MB: f64 = 1024.0 * 1024.0;
const GB: f64 = 1024.0 * MB;

impl Progress {
    pub fn new() -> Self {
        Progress {
            chunks: 0,
            bytes: 0,
            print_count: 0,
            start_time: Instant::now(),
        }
    }

    pub fn add_chuncks(&mut self, n: usize) {
        self.chunks += n;
    }

    pub fn add_bytes(&mut self, n: usize) {
        self.bytes += n;
    }

    pub fn info(&self) -> String {
        let bytes = self.bytes as f64;
        if bytes < MB {
            format!("{:.2}KB", bytes / KB)
        } else if bytes < GB {
            format!("{:.2}MB", bytes / MB)
        } else {
            format!("{:.2}GB", bytes / GB)
        }
    }

    pub fn print(&mut self) {
        let t = self.start_time.elapsed().as_secs_f64() / 60.0;
        // must have the suffix space, otherwise current line cannot be cleaned completely
        print!(
            "\rchunk: {}, total processed: {}, elapsed time: {:.2} minutes        ",
            self.chunks,
            self.info(),
            t
        );
        io::stdout().flush().unwrap();
        self.print_count += 1;
    }
}
