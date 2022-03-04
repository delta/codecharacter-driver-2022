use std::{
    fs::File,
    process::{Child, Command, Stdio},
};

use crate::{
    error::SimulatorError, handle_process, COMPILATION_MEMORY_LIMIT, COMPILATION_TIME_LIMIT,
    RUNTIME_MEMORY_LIMIT, RUNTIME_TIME_LIMIT,
};

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
                COMPILATION_TIME_LIMIT,
                "docker",
                "run",
                &format!("--memory={}", COMPILATION_MEMORY_LIMIT),
                &format!("--memory-swap={}", COMPILATION_MEMORY_LIMIT),
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

        let _ = handle_process(compile, true, |x| SimulatorError::CompilationError(x))?;

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
