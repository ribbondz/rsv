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
    pub chunk: usize,
}

impl ChunkReader {
    pub fn new(path: &Path) -> Result<Self, std::io::Error> {
        let rdr = BufReader::new(File::open(path)?).lines();
        Ok(ChunkReader(rdr))
    }

    pub fn next(&mut self) -> Result<Option<String>, std::io::Error> {
        let v = self.0.next().and_then(|i| i.ok());
        Ok(v)
    }

    pub fn send_to_channel_by_chunks(&mut self, tx: Sender<Task>, line_buffer_n: usize) {
        let mut lines = Vec::with_capacity(line_buffer_n);
        let mut n = 0;
        let mut bytes = 0;
        let mut chunk = 1;

        for l in self.0.by_ref() {
            let l = l.unwrap();
            n += 1;
            bytes += l.len();
            lines.push(l);
            if n >= line_buffer_n {
                tx.send(Task {
                    lines,
                    bytes,
                    chunk,
                })
                .unwrap();
                n = 0;
                bytes = 0;
                lines = Vec::with_capacity(line_buffer_n);
                chunk += 1;
            }
        }

        if !lines.is_empty() {
            tx.send(Task {
                lines,
                bytes,
                chunk,
            })
            .unwrap();
        }

        drop(tx)
    }
}
