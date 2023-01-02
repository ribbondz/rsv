use crossbeam_channel::Sender;

use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
    path::PathBuf,
};

pub struct ChunkReader {
    rdr: Lines<BufReader<File>>,
}

pub struct Task {
    pub lines: Vec<String>,
    pub bytes: usize,
}

impl ChunkReader {
    pub fn new(path: &PathBuf) -> Result<Self, std::io::Error> {
        let rdr = BufReader::new(File::open(&path)?).lines();
        Ok(ChunkReader { rdr })
    }

    pub fn next(&mut self) -> Result<String, std::io::Error> {
        self.rdr.next().unwrap()
    }

    pub fn send_to_channel_in_line_chunks(&mut self, tx: Sender<Task>, line_buffer_n: usize) {
        let mut lines = Vec::with_capacity(line_buffer_n);
        let mut n = 0;
        let mut bytes = 0;

        for l in self.rdr.by_ref() {
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

        if lines.len() > 0 {
            tx.send(Task { lines, bytes }).unwrap();
        }
    }
}
