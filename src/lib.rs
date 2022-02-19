use std::process::Child;

use error::SimulatorError;
pub mod cpp;
pub mod error;
pub mod fifo;
pub mod mq;
pub mod py;
pub mod request;
pub mod response;
pub mod simulator;

pub fn handle_process(mut proc: Child) -> Result<(), SimulatorError> {
    match proc.wait_with_output() {
        Ok(out) => {
            if out.status.success() {
                Ok(())
            } else {
                Err(SimulatorError::RuntimeError(format!(
                    "Program exited with non zero exit code: {} ",
                    String::from_utf8(out.stderr).unwrap().trim()
                )))
            }
        }
        Err(err) => Err(SimulatorError::UnidentifiedError(format!(
            "Waiting on Child Failed: {}",
            err
        ))),
    }
}
