use std::{error::Error, path::Path};

use calamine::{open_workbook_auto, DataType, Range, Reader, Rows};
use crossbeam_channel::Sender;

pub struct ExcelReader {
    range: Range<DataType>,
    next_called: usize,
}

pub struct ExcelChunkTask {
    pub lines: Vec<Vec<DataType>>,
    pub n: usize,
}

impl<'a> ExcelReader {
    pub fn new(path: &Path, sheet: usize) -> Result<Self, Box<dyn Error>> {
        let mut workbook = open_workbook_auto(path)?;

        let range = workbook
            .worksheet_range_at(sheet)
            .unwrap_or_else(|| panic!("{}-th sheet is not exist.", sheet))?;

        Ok(ExcelReader {
            range,
            next_called: 0,
        })
    }

    pub fn len(&self) -> usize {
        self.range.get_size().0
    }

    pub fn next(&mut self) -> Option<&[DataType]> {
        self.next_called += 1;
        self.range.rows().next()
    }

    pub fn iter(&'a self) -> Rows<'a, DataType> {
        self.range.rows()
    }

    pub fn send_to_channel_in_line_chunks(self, tx: Sender<ExcelChunkTask>) {
        let line_buffer_n = 1000;
        let mut lines = Vec::with_capacity(line_buffer_n);
        let mut n = 0;

        let mut iter = self.iter();
        for _ in 0..self.next_called {
            iter.next();
        }

        for l in iter {
            let l = l.to_owned();
            n += 1;

            lines.push(l);

            if n >= line_buffer_n {
                tx.send(ExcelChunkTask { lines, n }).unwrap();
                n = 0;
                lines = Vec::with_capacity(line_buffer_n);
            }
        }

        if !lines.is_empty() {
            tx.send(ExcelChunkTask { lines, n }).unwrap();
        }

        drop(tx)
    }
}
