use std::{
    fs::File,
    process::{Child, Command, Stdio},
};

use crate::{error::SimulatorError, handle_process};

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
                "--signal=KILL",
                "5",
                "docker",
                "run",
                "--memory=100m",
                "--memory-swap=100m",
                "--cpus=1.5",
                "--rm",
                "-v",
                format!(
                    "{}/Run.java:/player_code/Run.java",
                    self.current_dir.as_str()
                )
                .as_str(),
                "-v",
                format!("{}/run.jar:/player_code/run.jar", self.current_dir.as_str()).as_str(),
                "ghcr.io/delta/codecharacter-java-compiler:latest",
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

        let _ = handle_process(compile, true, |x| SimulatorError::CompilationError(x))?;

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
                "-v",
                format!("{}/run.jar:/run.jar", self.current_dir.as_str()).as_str(),
                "ghcr.io/delta/codecharacter-java-runner:latest",
            ])
            .current_dir(&self.current_dir.to_owned())
            .stdin(stdin)
            .stdout(stdout)
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|err| {
                SimulatorError::UnidentifiedError(format!(
                    "Couldnt spawn the java runner process: {}",
                    err
                ))
            })
    }
}
