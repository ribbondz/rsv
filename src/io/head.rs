use crate::{
    args::Head,
    utils::{cli_result::CliResult, filename::new_file, writer::Writer},
};
use std::io::{stdin, BufRead};

impl Head {
    pub fn io_run(&self) -> CliResult {
        let out = new_file("sorted.csv");
        let mut wtr = Writer::file_or_stdout(self.export, &out)?;

        // show head n
        // open file and header
        stdin()
            .lock()
            .lines()
            .take(self.n + 1 - self.no_header as usize)
            .for_each(|r| {
                if let Ok(r) = r {
                    wtr.write_str_unchecked(&r);
                }
            });

        if self.export {
            println!("Saved to file: {}", out.display())
        }

        Ok(())
    }
}
