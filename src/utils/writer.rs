use super::{cli_result::CliResult, constants::TERMINATOR};
use calamine::DataType;
use std::{
    fs::{File, OpenOptions},
    io::{stdout, BufWriter, Error, Write},
    path::Path,
    process,
};

pub struct Writer(Box<dyn Write>);

impl Writer {
    pub fn new(path: &Path) -> Result<Self, Error> {
        let wtr = Box::new(BufWriter::new(File::create(path)?));

        Ok(Writer(wtr))
    }

    pub fn file_or_stdout(export: bool, path: &Path) -> Result<Self, Error> {
        let wtr = match export {
            true => Box::new(BufWriter::new(File::create(path)?)) as Box<dyn Write>,
            false => Box::new(stdout()) as Box<dyn Write>,
        };

        Ok(Writer(wtr))
    }

    pub fn append_to(out: &Path) -> Result<Self, Error> {
        // open file
        let f = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(out)?;

        let wtr = Box::new(BufWriter::new(f));

        Ok(Writer(wtr))
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) -> CliResult {
        self.0.write_all(bytes)?;
        Ok(())
    }

    pub fn write_bytes_unchecked(&mut self, bytes: &[u8]) {
        if self.0.write_all(bytes).is_err() {
            process::exit(0)
        }
    }

    pub fn write_header(&mut self, row: &str) -> CliResult {
        if !row.is_empty() {
            self.write_line(row)?;
        }
        Ok(())
    }

    pub fn write_header_unchecked(&mut self, row: &str) {
        if self.write_header(row).is_err() {
            process::exit(0)
        }
    }

    pub fn write_line<T: AsRef<str>>(&mut self, row: T) -> CliResult {
        self.0.write_all(row.as_ref().as_bytes())?;
        self.0.write_all(TERMINATOR)?;
        Ok(())
    }

    pub fn write_line_unchecked<T: AsRef<str>>(&mut self, row: T) {
        if self.write_line(row).is_err() {
            process::exit(0)
        }
    }

    pub fn write_lines<T: AsRef<str>>(&mut self, lines: &[T]) -> CliResult {
        for l in lines {
            self.write_line(l)?
        }
        Ok(())
    }

    pub fn write_lines_unchecked<T: AsRef<str>>(&mut self, lines: &[T]) {
        if self.write_lines(lines).is_err() {
            process::exit(0)
        }
    }

    pub fn write_line_by_field<T: AsRef<str>>(
        &mut self,
        line: &[T],
        sep: Option<&[u8]>,
    ) -> CliResult {
        let mut l = line.iter().peekable();
        while let Some(f) = l.next() {
            self.0.write_all(f.as_ref().as_bytes())?;
            self.0.write_all(if l.peek().is_none() {
                TERMINATOR
            } else {
                sep.unwrap_or(b",")
            })?;
        }

        Ok(())
    }

    pub fn write_line_by_field_unchecked<T: AsRef<str>>(&mut self, line: &[T], sep: Option<&[u8]>) {
        if self.write_line_by_field(line, sep).is_err() {
            process::exit(0)
        }
    }

    pub fn write_lines_by_field<T: AsRef<str>>(
        &mut self,
        lines: &[Vec<T>],
        sep: Option<&[u8]>,
    ) -> CliResult {
        for l in lines {
            self.write_line_by_field(l, sep)?;
        }

        Ok(())
    }

    pub fn write_lines_by_field_unchecked<T: AsRef<str>>(
        &mut self,
        lines: &[Vec<T>],
        sep: Option<&[u8]>,
    ) {
        if self.write_lines_by_field(lines, sep).is_err() {
            process::exit(0)
        }
    }

    pub fn write_excel_line(&mut self, line: &[DataType]) -> CliResult {
        let mut l = line.iter().peekable();
        while let Some(f) = l.next() {
            self.0.write_all(f.to_string().as_bytes())?;
            if l.peek().is_some() {
                self.0.write_all(b",")?;
            } else {
                self.0.write_all(TERMINATOR)?;
            }
        }

        Ok(())
    }

    pub fn write_excel_line_unchecked(&mut self, line: &[DataType]) {
        if self.write_excel_line(line).is_err() {
            process::exit(0)
        }
    }

    pub fn write_excel_lines(&mut self, lines: &[Vec<DataType>]) -> CliResult {
        for l in lines {
            self.write_excel_line(l)?;
        }
        Ok(())
    }

    pub fn write_excel_lines_by_ref(&mut self, lines: &[&Vec<DataType>]) -> CliResult {
        for &l in lines {
            self.write_excel_line(l)?;
        }
        Ok(())
    }
}
