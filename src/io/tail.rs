use crate::{
    args::Tail,
    utils::{cli_result::CliResult, filename::new_file, reader::IoReader, writer::Writer},
};

impl Tail {
    pub fn io_run(&self) -> CliResult {
        let out = new_file("tail.csv");
        let mut wtr = Writer::file_or_stdout(self.export, &out)?;

        // lines
        let lines = IoReader::new().lines();

        // header
        if !self.no_header && !lines.is_empty() {
            wtr.write_str_unchecked(&lines[0]);
        }

        // show tail n
        lines
            .iter()
            .skip(1 - self.no_header as usize)
            .rev()
            .take(self.n)
            .rev()
            .for_each(|r| wtr.write_str_unchecked(r));

        if self.export {
            println!("Saved to file: {}", out.display())
        }

        Ok(())
    }
}
