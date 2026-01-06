use crate::utils::util::werr_exit;
use calamine::{Data, Range, Reader, Rows, open_workbook_auto};
use crossbeam_channel::Sender;
use std::error::Error;
use std::io::stdin;
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

    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<Result<String, std::io::Error>> {
        self.0.next()
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
    }
}

pub struct ExcelReader {
    range: Range<Data>,
    pub next_called: usize,
}

pub struct ExcelChunkTask {
    pub lines: Vec<Vec<Data>>,
    pub n: usize,
    pub chunk: usize,
}

impl<'a> ExcelReader {
    pub fn new(path: &Path, sheet: usize) -> Result<Self, Box<dyn Error>> {
        let mut workbook = open_workbook_auto(path)?;

        let range = workbook.worksheet_range_at(sheet).unwrap_or_else(|| {
            werr_exit!("{}-th sheet does not exist.", sheet);
        })?;

        Ok(ExcelReader {
            range,
            next_called: 0,
        })
    }

    pub fn len(&self) -> usize {
        self.range.get_size().0
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn column_n(&self) -> usize {
        self.range.get_size().1
    }

    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<&[Data]> {
        self.next_called += 1;
        self.range.rows().next()
    }

    pub fn iter(&'a self) -> Rows<'a, Data> {
        self.range.rows()
    }

    pub fn send_to_channel_by_chunk(self, tx: Sender<ExcelChunkTask>, size: Option<usize>) {
        let line_buffer_n = size.unwrap_or(1000);
        let mut lines = Vec::with_capacity(line_buffer_n);
        let mut n = 0;
        let mut chunk = 1;
        for l in self.iter().skip(self.next_called) {
            let l = l.to_owned();
            n += 1;
            lines.push(l);
            if n >= line_buffer_n {
                tx.send(ExcelChunkTask { lines, n, chunk }).unwrap();
                n = 0;
                lines = Vec::with_capacity(line_buffer_n);
                chunk += 1;
            }
        }

        if !lines.is_empty() {
            tx.send(ExcelChunkTask { lines, n, chunk }).unwrap();
        }

        drop(tx)
    }
}

pub struct IoReader {
    no_header: bool,
    top_n: Option<usize>,
}

impl Default for IoReader {
    fn default() -> Self {
        Self::new()
    }
}

impl IoReader {
    pub fn new() -> Self {
        IoReader {
            no_header: false,
            top_n: None,
        }
    }

    pub fn no_header(&mut self, no_header: bool) -> &mut Self {
        self.no_header = no_header;
        self
    }

    pub fn top_n(&mut self, top_n: usize) -> &mut Self {
        self.top_n = Some(top_n);
        self
    }

    pub fn lines(&self) -> Vec<String> {
        // open file and header
        let lines = stdin().lock().lines();

        match self.top_n {
            Some(n) => lines
                .take(n + 1 - self.no_header as usize)
                .map_while(Result::ok)
                .collect(),
            None => lines.map_while(Result::ok).collect(),
        }
    }
}
