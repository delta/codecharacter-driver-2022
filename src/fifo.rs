use std::fs::{remove_file, File, OpenOptions};
use std::os::unix::prelude::{AsRawFd, OpenOptionsExt};

use crate::error::SimulatorError;
use nix::fcntl::{self, FcntlArg, OFlag};
use nix::libc::O_NONBLOCK;
use nix::sys::stat;
use nix::unistd::mkfifo;
pub struct Fifo {
    _name: String,
    _in: Option<File>,
    _out: Option<File>,
}

impl Fifo {
    fn open_fifo(path: &str) -> Result<(), SimulatorError> {
        match mkfifo(
            path,
            stat::Mode::S_IRWXU | stat::Mode::S_IRWXG | stat::Mode::S_IRWXO,
        ) {
            Ok(_) | Err(nix::errno::Errno::EEXIST) => Ok(()),
            Err(e) => Err(SimulatorError::FifoCreationError(format!("{}", e))),
        }
    }
    fn make_blocking(fd: i32) -> Result<(), SimulatorError> {
        let mut flags = OFlag::from_bits_truncate(fcntl::fcntl(fd, FcntlArg::F_GETFL).unwrap());
        flags.remove(OFlag::O_NONBLOCK);
        fcntl::fcntl(fd, FcntlArg::F_SETFL(flags))
            .map_err(|e| SimulatorError::FifoCreationError(format!("{}", e)))?;
        return Ok(());
    }
    fn setup_pipe(f: &str) -> Result<(File, File), SimulatorError> {
        Fifo::open_fifo(f)?;
        let stdin = OpenOptions::new()
            .custom_flags(O_NONBLOCK)
            .read(true)
            .open(f)
            .map_err(|e| SimulatorError::FifoCreationError(format!("{}", e)))?;
        let stdout = OpenOptions::new()
            .write(true)
            .open(f)
            .map_err(|e| SimulatorError::FifoCreationError(format!("{}", e)))?;
        let stdin_fd = stdin.as_raw_fd();
        Fifo::make_blocking(stdin_fd)?;
        Ok((stdin, stdout))
    }
    pub fn new(filename: String) -> Result<Self, SimulatorError> {
        let (fin, fout) = Fifo::setup_pipe(&filename)?;
        Ok(Self {
            _name: filename,
            _in: Some(fin),
            _out: Some(fout),
        })
    }
    pub fn get_ends(&mut self) -> Option<(File, File)> {
        match (self._in.take(), self._out.take()) {
            (Some(_in), Some(_out)) => Some((_in, _out)),
            _ => None,
        }
    }
}

impl Drop for Fifo {
    fn drop(&mut self) {
        let _ = remove_file(&self._name);
    }
}

#[cfg(test)]
mod fifo_tests {
    use std::{
        io::{Read, Write},
        process::{Command, Stdio},
    };

    use super::*;

    #[test]
    fn communication_test() {
        let mut fifo = Fifo::new("/tmp/p1".to_owned()).unwrap();
        let (mut fin, mut fout) = fifo.get_ends().unwrap();

        let s1 = fout.write(b"Hello World").unwrap();
        fout.flush().unwrap();

        let mut buffer = vec![0; s1];
        let s2 = fin.read(&mut buffer).unwrap();

        let string = String::from_utf8(buffer).unwrap().to_owned();

        assert_eq!(s1, s2);
        assert_eq!(string.clone(), "Hello World".to_owned());

        println!("{}", string);
    }
    #[test]
    fn added_data_to_fifo_before_running_cmd_is_saved() {
        let mut fifo = Fifo::new("/tmp/p1".to_owned()).unwrap();
        let (fin, mut fout) = fifo.get_ends().unwrap();

        let _ = fout.write(b"Hello World").unwrap();
        fout.flush().unwrap();
        drop(fout); // will send eof to the read end

        {
            let python_code = "import sys\nsys.stdout.write(sys.stdin.readline())";
            let mut temp_python_file = File::create("temp_py.py").unwrap();
            temp_python_file.write(python_code.as_bytes()).unwrap();
        }

        match Command::new("python3")
            .args(&["temp_py.py"])
            .stdout(Stdio::piped())
            .stdin(fin)
            .spawn()
            .unwrap()
            .wait_with_output()
        {
            Ok(out) => {
                assert_eq!("Hello World", String::from_utf8(out.stdout).unwrap().trim());
            }
            Err(_) => {
                assert!(false);
            }
        }
        let _ = remove_file("temp_py.py");
    }

    #[test]
    fn get_ends() {
        let fifo = Fifo::new("/tmp/p2".to_owned());
        assert!(fifo.is_ok());
        let mut fifo = fifo.unwrap();
        assert!(fifo.get_ends().is_some());
        assert!(fifo.get_ends().is_none());
    }
}
