use crossbeam_channel::Sender;
use std::path::Path;
use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
};

pub struct ChunkReader(Lines<BufReader<File>>);

pub struct Task {
    pub lines: Vec<String>,
    pub bytes: usize,
}

impl ChunkReader {
    pub fn new(path: &Path) -> Result<Self, std::io::Error> {
        let rdr = BufReader::new(File::open(path)?).lines();
        Ok(ChunkReader(rdr))
    }

    pub fn next(&mut self) -> Result<String, std::io::Error> {
        self.0.next().unwrap()
    }

    pub fn send_to_channel_in_line_chunks(&mut self, tx: Sender<Task>, line_buffer_n: usize) {
        let mut lines = Vec::with_capacity(line_buffer_n);
        let mut n = 0;
        let mut bytes = 0;

        for l in self.0.by_ref() {
            let l = l.unwrap();

            n += 1;
            bytes += l.len();

            lines.push(l);

            if n >= line_buffer_n {
                tx.send(Task { lines, bytes }).unwrap();
                n = 0;
                bytes = 0;
                lines = Vec::with_capacity(line_buffer_n);
            }
        }

        if !lines.is_empty() {
            tx.send(Task { lines, bytes }).unwrap();
        }

        drop(tx)
    }
}
