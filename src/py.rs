use std::{
    fs::File,
    process::{Command, Stdio},
};

use crate::error::SimulatorError;

pub struct Runner {
    file_name: String,
}

impl Runner {
    pub fn new(file_name: String) -> Self {
        Runner { file_name }
    }
    pub fn run(&self, stdin: File, stdout: File) -> Result<std::process::Child, SimulatorError> {
        Command::new("python3")
            .args([&self.file_name])
            .stdin(stdin)
            .stdout(stdout)
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|err| {
                SimulatorError::UnidentifiedError(format!(
                    "Couldnt spawn the python runner process: {}",
                    err
                ))
            })
    }
}
