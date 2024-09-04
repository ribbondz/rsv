use crate::{
    args::Excel2csv,
    utils::{
        cli_result::CliResult, constants::TERMINATOR, reader::ExcelReader, util::werr_exit,
        writer::Writer,
    },
};

impl Excel2csv {
    pub fn excel_run(&self) -> CliResult {
        if self.filename.is_none() {
            werr_exit!("Please provide a file path.");
        }
        let path = &self.path();

        // new file
        let out = path.with_extension("csv");

        // open files
        let range = ExcelReader::new(path, self.sheet)?;
        let mut wtr = Writer::new(&out)?;

        // const
        let sep = [self.sep as u8];

        // excel2csv
        for r in range.iter() {
            let mut r = r.iter().peekable();
            while let Some(v) = r.next() {
                match v {
                    calamine::Data::String(v) => match v.contains(self.sep) {
                        true => write!(&mut wtr.0, "\"{}\"", v)?,
                        false => wtr.write_bytes(v.trim().as_bytes())?,
                    },
                    _ => write!(&mut wtr.0, "{}", v)?,
                };
                if r.peek().is_some() {
                    wtr.write_bytes(&sep)?;
                } else {
                    wtr.write_bytes(TERMINATOR)?;
                }
            }
        }

        println!("Saved to file: {}", out.display());

        Ok(())
    }
}
