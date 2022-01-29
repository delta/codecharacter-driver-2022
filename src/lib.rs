#![allow(dead_code, unused_imports)]
use std::fs::{remove_file, File, OpenOptions};
use std::io::{Error, Read};
use std::os::unix::prelude::{AsRawFd, OpenOptionsExt};
use std::process::{exit, Child, Command, ExitStatus, Stdio};

use error::SimulatorError;
use nix::fcntl::{self, FcntlArg, OFlag};
use nix::libc::O_NONBLOCK;
use nix::sys::stat;
use nix::unistd::mkfifo;

pub mod cpp;
pub mod error;
pub mod py;
pub mod simulator;

fn open_fifo(path: &str) -> Result<(), SimulatorError> {
    match mkfifo(
        path,
        stat::Mode::S_IRWXU | stat::Mode::S_IRWXG | stat::Mode::S_IRWXO,
    ) {
        Ok(_) | Err(nix::errno::Errno::EEXIST) => Ok(()),
        Err(e) => Err(SimulatorError::FifoCreationError(format!("{}", e))),
    }
}

pub fn setup_pipes(
    process1_inp: &str,
    process2_inp: &str,
) -> Result<(File, File, File, File), SimulatorError> {
    open_fifo(process1_inp)?;
    open_fifo(process2_inp)?;

    let p1_stdin = OpenOptions::new()
        .custom_flags(O_NONBLOCK)
        .read(true)
        .open(process1_inp)
        .map_err(|e| SimulatorError::FifoCreationError(format!("{}", e)))?;
    let p2_stdin = OpenOptions::new()
        .custom_flags(O_NONBLOCK)
        .read(true)
        .open(process2_inp)
        .map_err(|e| SimulatorError::FifoCreationError(format!("{}", e)))?;
    let p1_stdout = OpenOptions::new()
        .write(true)
        .open(process2_inp)
        .map_err(|e| SimulatorError::FifoCreationError(format!("{}", e)))?;
    let p2_stdout = OpenOptions::new()
        .write(true)
        .open(process1_inp)
        .map_err(|e| SimulatorError::FifoCreationError(format!("{}", e)))?;
    let p1_stdin_fd = p1_stdin.as_raw_fd();
    let p2_stdin_fd = p2_stdin.as_raw_fd();
    make_blocking(p1_stdin_fd)?;
    make_blocking(p2_stdin_fd)?;

    Ok((p1_stdin, p1_stdout, p2_stdin, p2_stdout))
}

pub fn handle_process(mut proc: Child) -> Result<(), SimulatorError> {
    // let result = proc.wait_with_output();
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

fn make_blocking(fd: i32) -> Result<(), SimulatorError> {
    let mut flags = OFlag::from_bits_truncate(fcntl::fcntl(fd, FcntlArg::F_GETFL).unwrap());
    flags.remove(OFlag::O_NONBLOCK);
    fcntl::fcntl(fd, FcntlArg::F_SETFL(flags))
        .map_err(|e| SimulatorError::FifoCreationError(format!("{}", e)))?;
    return Ok(());
}

pub fn cleanup(process1_inp: &str, process2_inp: &str) {
    remove_file(process2_inp).unwrap();
    remove_file(process1_inp).unwrap();
}
