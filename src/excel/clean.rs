use crate::{
    args::Clean,
    utils::{cli_result::CliResult, util::werr_exit},
};

impl Clean {
    pub fn excel_run(&self) -> CliResult {
        werr_exit!("Error: rsv clean does not support Excel files.");
    }
}
