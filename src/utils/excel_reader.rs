use std::{error::Error, path::Path, process};

use calamine::{open_workbook_auto, DataType, Range, Reader, Rows};
use crossbeam_channel::Sender;

use super::util::werr;

pub struct ExcelReader {
    range: Range<DataType>,
    pub next_called: usize,
}

pub struct ExcelChunkTask {
    pub lines: Vec<Vec<DataType>>,
    pub n: usize,
    pub chunk: usize,
}

impl<'a> ExcelReader {
    pub fn new(path: &Path, sheet: usize) -> Result<Self, Box<dyn Error>> {
        let mut workbook = open_workbook_auto(path)?;

        let range = workbook.worksheet_range_at(sheet).unwrap_or_else(|| {
            werr!("{}-th sheet is not exist.", sheet);
            process::exit(1)
        })?;

        Ok(ExcelReader {
            range,
            next_called: 0,
        })
    }

    pub fn len(&self) -> usize {
        self.range.get_size().0
    }

    pub fn column_n(&self) -> usize {
        self.range.get_size().1
    }

    pub fn next(&mut self) -> Option<&[DataType]> {
        self.next_called += 1;
        self.range.rows().next()
    }

    pub fn iter(&'a self) -> Rows<'a, DataType> {
        self.range.rows()
    }

    pub fn send_to_channel_in_line_chunks(self, tx: Sender<ExcelChunkTask>, size: Option<usize>) {
        let line_buffer_n = size.or(Some(1000)).unwrap();
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
