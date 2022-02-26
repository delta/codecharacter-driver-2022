use std::{
    fs::File,
    process::{Child, Command, Stdio},
};

use crate::error::SimulatorError;

pub struct Runner {
    current_dir: String,
}
impl Runner {
    pub fn new(current_dir: String) -> Self {
        Runner { current_dir }
    }
    pub fn run(&self, stdin: File, stdout: File) -> Result<Child, SimulatorError> {
        let compile = Command::new("timeout".to_owned())
            .args([
                "5",
                "docker",
                "run",
                "--memory=100m",
                "--cpus=2",
                "--rm",
                "-v",
                format!("{}/run.cpp:/player_code/run.cpp", self.current_dir.as_str()).as_str(),
                "-v",
                format!("{}/run:/player_code/run", self.current_dir.as_str()).as_str(),
                "ghcr.io/delta/codecharacter-cpp-compiler:latest",
            ])
            .current_dir(&self.current_dir.to_owned())
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|err| {
                SimulatorError::UnidentifiedError(format!(
                    "Couldnt spawn compilation command: {}",
                    err
                ))
            })?;
        let compile_output = compile.wait_with_output().map_err(|err| {
            SimulatorError::UnidentifiedError(format!(
                "Waiting on compilation process failed: {}",
                err
            ))
        })?;
        if !compile_output.status.success() {
            return Err(SimulatorError::CompilationError(
                String::from_utf8(compile_output.stderr)
                    .unwrap()
                    .trim()
                    .to_owned(),
            ));
        }
        Command::new("timeout".to_owned())
            .args([
                "10",
                "docker",
                "run",
                "--memory=100m",
                "--cpus=1",
                "--rm",
                "-i",
                "-v",
                format!("{}/run:/player_code", self.current_dir.as_str()).as_str(),
                "ghcr.io/delta/codecharacter-cpp-runner:latest",
            ])
            .current_dir(&self.current_dir.to_owned())
            .stdin(stdin)
            .stdout(stdout)
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|err| {
                SimulatorError::UnidentifiedError(format!(
                    "Couldnt spawn the C++ runner process: {}",
                    err
                ))
            })
    }
}
