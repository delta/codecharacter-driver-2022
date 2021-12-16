#![allow(dead_code, unused_imports)]
use std::fs::{remove_file, File, OpenOptions};
use std::io::{Error, Read};
use std::os::unix::prelude::{AsRawFd, OpenOptionsExt};
use std::process::{exit, Child, Command, ExitStatus, Stdio};

use nix::fcntl::{self, FcntlArg, OFlag};
use nix::libc::O_NONBLOCK;
use nix::sys::stat;
use nix::unistd::mkfifo;

pub mod cpp;
pub mod py;
pub mod simulator;

const P1_IN: &str = "/tmp/p1_in";
const P2_IN: &str = "/tmp/p2_in";

fn open_fifo(path: &str) {
    match mkfifo(
        path,
        stat::Mode::S_IRWXU | stat::Mode::S_IRWXG | stat::Mode::S_IRWXO,
    ) {
        Ok(_) | Err(nix::errno::Errno::EEXIST) => {}
        Err(err) => {
            // TODO: better way to handle this
            panic!("Error creating fifo: {}", err);
        }
    }
}

pub fn setup_pipes() -> (File, File, File, File) {
    open_fifo(P1_IN);
    open_fifo(P2_IN);

    let p1_stdin = OpenOptions::new()
        .custom_flags(O_NONBLOCK)
        .read(true)
        .open(P1_IN)
        .unwrap();
    let p2_stdin = OpenOptions::new()
        .custom_flags(O_NONBLOCK)
        .read(true)
        .open(P2_IN)
        .unwrap();
    let p1_stdout = OpenOptions::new().write(true).open(P2_IN).unwrap();
    let p2_stdout = OpenOptions::new().write(true).open(P1_IN).unwrap();
    let p1_stdin_fd = p1_stdin.as_raw_fd();
    let p2_stdin_fd = p2_stdin.as_raw_fd();
    make_blocking(p1_stdin_fd);
    make_blocking(p2_stdin_fd);

    (p1_stdin, p1_stdout, p2_stdin, p2_stdout)
}

pub fn handle_process(mut proc: Child) -> ExitStatus {
    match proc.wait() {
        Ok(e) => {
            println!("{}", e.to_string());
            e
        }
        Err(_) => {
            //TODO: error handling
            unimplemented!()
        }
    }
}

fn make_blocking(fd: i32) {
    let mut flags = OFlag::from_bits_truncate(fcntl::fcntl(fd, FcntlArg::F_GETFL).unwrap());
    flags.remove(OFlag::O_NONBLOCK);
    fcntl::fcntl(fd, FcntlArg::F_SETFL(flags)).unwrap();
}

pub fn cleanup() {
    remove_file(P2_IN).unwrap();
    remove_file(P1_IN).unwrap();
}
