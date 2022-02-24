use std::fs::File;

use std::process::{Command, Stdio};

use crate::error::SimulatorError;
pub struct Simulator {
    main_command: &'static str,
    args: Vec<&'static str>,
}

impl Simulator {
    pub fn new(main_command: &'static str, args: Vec<&'static str>) -> Self {
        Simulator { main_command, args }
    }
    pub fn run(&self, stdin: File, stdout: File) -> Result<std::process::Child, SimulatorError> {
        Command::new("timeout".to_owned())
            .args(["3", self.main_command])
            .args(&self.args)
            .stdin(stdin)
            .stdout(stdout)
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|err| {
                SimulatorError::UnidentifiedError(format!(
                    "Couldn't spawn simulator process: {}",
                    err
                ))
            })
    }
}
