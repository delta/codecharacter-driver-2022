use std::{
    fs::File,
    process::{Command, Stdio},
};

use crate::{error::SimulatorError, RUNTIME_MEMORY_LIMIT, RUNTIME_TIME_LIMIT};

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
                "--signal=KILL",
                RUNTIME_TIME_LIMIT,
                "docker",
                "run",
                &format!("--memory={}", RUNTIME_MEMORY_LIMIT),
                &format!("--memory-swap={}", RUNTIME_MEMORY_LIMIT),
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
