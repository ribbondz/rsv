use crate::utils::util::werr_exit;

pub type CliResult = Result<(), Box<dyn std::error::Error>>;

pub trait E {
    fn handle_err(&self) {}
}

impl E for CliResult {
    fn handle_err(&self) {
        match self {
            Ok(()) => {}
            Err(msg) => {
                werr_exit!("Error: {}", msg);
            }
        }
    }
}
