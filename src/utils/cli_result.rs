use crate::utils::util::werr;
use std::process;

pub type CliResult = Result<(), Box<dyn std::error::Error>>;

pub trait E {
    fn handle_err(&self) {}
}

impl E for CliResult {
    fn handle_err(&self) {
        match self {
            Ok(()) => {}
            Err(msg) => {
                werr!("Error: {}", msg);
                process::exit(1)
            }
        }
    }
}
