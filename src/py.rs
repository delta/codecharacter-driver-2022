use std::{
    fs::File,
    io::{Error, Read},
    process::{exit, Child, Command, Stdio},
};

pub struct Runner {
    file_name: String,
}

impl Runner {
    pub fn new(file_name: String) -> Self {
        Runner { file_name }
    }
    pub fn run(&self, stdin: File, stdout: File) -> Result<std::process::Child, Error> {
        Command::new("python3")
            .args([&self.file_name])
            .stdin(stdin)
            .stdout(stdout)
            .spawn()
    }
}
