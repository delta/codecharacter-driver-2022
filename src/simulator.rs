use std::fs::File;

use std::process::{Command, Stdio};

use crate::error::SimulatorError;
pub struct Simulator {}

impl Simulator {
    pub fn run(&self, stdin: File, stdout: File) -> Result<std::process::Child, SimulatorError> {
        Command::new("timeout".to_owned())
            .args([
                "--signal=KILL",
                "10",
                "docker",
                "run",
                "--memory=100m",
                "--memory-swap=100m",
                "--cpus=1",
                "--rm",
                "-i",
                "ghcr.io/delta/codecharacter-simulator:latest",
            ])
            .stdin(stdin)
            .stdout(stdout)
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|err| {
                SimulatorError::UnidentifiedError(format!(
                    "Couldnt spawn the simulator process: {}",
                    err
                ))
            })
    }
}
