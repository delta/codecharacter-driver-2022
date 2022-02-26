use std::{
    fs::File,
    process::{Command, Stdio},
};

use crate::error::SimulatorError;

pub struct Runner {
    current_dir: String,
}

impl Runner {
    pub fn new(current_dir: String) -> Self {
        Runner { current_dir }
    }
    pub fn run(&self, stdin: File, stdout: File) -> Result<std::process::Child, SimulatorError> {
        Command::new("timeout".to_owned())
            .args([
                "10",
                "docker",
                "run",
                "--memory=100m",
                "--memory-swap=100m",
                "--cpus=1",
                "--rm",
                "-i",
                "-v",
                format!("{}/run.py:/player_code/run.py", self.current_dir.as_str()).as_str(),
                "ghcr.io/delta/codecharacter-python-runner:latest",
            ])
            .current_dir(&self.current_dir)
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
