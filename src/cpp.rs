use std::{
    fs::File,
    process::{Child, Command, Stdio},
};

use crate::error::SimulatorError;

pub struct Runner {
    file_name: String,
    out_name: String,
}
impl Runner {
    pub fn new(file_name: String, out_name: String) -> Self {
        Runner {
            file_name,
            out_name,
        }
    }
    pub fn run(&self, stdin: File, stdout: File) -> Result<Child, SimulatorError> {
        let compile = Command::new("clang++")
            .args([
                "-fsanitize=address",
                "-O2",
                "-o",
                &self.out_name,
                &self.file_name,
            ])
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
        Command::new("./".to_owned() + &self.out_name.clone())
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
