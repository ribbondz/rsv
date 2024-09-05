use crate::{
    args::Excel2csv,
    utils::{cli_result::CliResult, reader::ExcelReader, util::werr_exit, writer::Writer},
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
            wtr.write_excel_line(r, &sep)?;
        }

        println!("Saved to file: {}", out.display());

        Ok(())
    }
}
