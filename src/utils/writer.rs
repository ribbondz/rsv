use super::{cli_result::CliResult, constants::TERMINATOR};
use calamine::Data;
use chrono::Timelike;
use std::{
    fs::{File, OpenOptions},
    io::{stdout, BufWriter, Error, Write},
    path::Path,
    process,
};

pub struct Writer(pub Box<dyn Write>);

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

    pub fn stdout() -> Result<Self, Error> {
        let wtr = Box::new(stdout()) as Box<dyn Write>;
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
            self.write_str(row)?;
        }
        Ok(())
    }

    // pub fn write_header_unchecked(&mut self, row: &str) {
    //     if self.write_header(row).is_err() {
    //         process::exit(0)
    //     }
    // }

    pub fn write_str<T: AsRef<str>>(&mut self, row: T) -> CliResult {
        self.0.write_all(row.as_ref().as_bytes())?;
        self.0.write_all(TERMINATOR)?;
        Ok(())
    }

    pub fn write_str_unchecked<T: AsRef<str>>(&mut self, row: T) {
        if self.write_str(row).is_err() {
            process::exit(0)
        }
    }

    pub fn write_strings<T: AsRef<str>>(&mut self, lines: &[T]) -> CliResult {
        for l in lines {
            self.write_str(l)?
        }
        Ok(())
    }

    pub fn write_strings_unchecked<T: AsRef<str>>(&mut self, lines: &[T]) {
        if self.write_strings(lines).is_err() {
            process::exit(0)
        }
    }

    pub fn write_fields<T: AsRef<str>>(&mut self, line: &[T], sep: Option<&[u8]>) -> CliResult {
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

    pub fn write_fields_unchecked<T: AsRef<str>>(&mut self, line: &[T], sep: Option<&[u8]>) {
        if self.write_fields(line, sep).is_err() {
            process::exit(0)
        }
    }

    pub fn write_selected_fields<T: AsRef<str>>(
        &mut self,
        line: &[T],
        cols: &[usize],
        sep: Option<&[u8]>,
    ) -> CliResult {
        let mut l = cols.iter().peekable();
        while let Some(&i) = l.next() {
            self.0.write_all(line[i].as_ref().as_bytes())?;
            self.0.write_all(if l.peek().is_none() {
                TERMINATOR
            } else {
                sep.unwrap_or(b",")
            })?;
        }

        Ok(())
    }

    pub fn write_selected_fields_unchecked<T: AsRef<str>>(
        &mut self,
        line: &[T],
        cols: &[usize],
        sep: Option<&[u8]>,
    ) {
        if self.write_selected_fields(line, cols, sep).is_err() {
            process::exit(0)
        }
    }

    pub fn write_fields_of_lines_unchecked<T: AsRef<str>>(
        &mut self,
        lines: &Vec<Vec<T>>,
        sep: Option<&[u8]>,
    ) {
        for line in lines {
            if self.write_fields(line, sep).is_err() {
                process::exit(0)
            }
        }
    }

    pub fn write_excel_line(&mut self, line: &[Data], sep: &[u8]) -> CliResult {
        let mut l = line.iter().peekable();
        while let Some(f) = l.next() {
            match f {
                Data::DateTime(v) => {
                    if let Some(a) = v.as_datetime() {
                        if a.hour() == 0 && a.minute() == 0 && a.second() == 0 {
                            write!(&mut self.0, "{}", a.format("%Y-%m-%d"))?
                        } else {
                            write!(&mut self.0, "{}", a.format("%Y-%m-%d %H:%M:%S"))?
                        }
                    };
                }
                _ => write!(&mut self.0, "{}", f)?,
            }

            if l.peek().is_some() {
                self.0.write_all(sep)?;
            } else {
                self.0.write_all(TERMINATOR)?;
            }
        }

        Ok(())
    }

    pub fn write_excel_line_unchecked(&mut self, line: &[Data], sep: &[u8]) {
        if self.write_excel_line(line, sep).is_err() {
            process::exit(0)
        }
    }

    pub fn write_excel_selected_fields(
        &mut self,
        line: &[Data],
        cols: &[usize],
        sep: &[u8],
    ) -> CliResult {
        let mut l = cols.iter().peekable();
        while let Some(&i) = l.next() {
            write!(&mut self.0, "{}", line[i])?;
            if l.peek().is_some() {
                self.0.write_all(sep)?;
            } else {
                self.0.write_all(TERMINATOR)?;
            }
        }

        Ok(())
    }

    pub fn write_excel_selected_fields_unchecked(
        &mut self,
        line: &[Data],
        cols: &[usize],
        sep: &[u8],
    ) {
        if self.write_excel_selected_fields(line, cols, sep).is_err() {
            process::exit(0)
        }
    }

    pub fn write_excel_lines(&mut self, lines: &[Vec<Data>], sep: &[u8]) -> CliResult {
        for l in lines {
            self.write_excel_line(l, sep)?;
        }
        Ok(())
    }

    pub fn write_excel_lines_by_ref(&mut self, lines: &[&Vec<Data>], sep: &[u8]) -> CliResult {
        for &l in lines {
            self.write_excel_line(l, sep)?;
        }
        Ok(())
    }
}
