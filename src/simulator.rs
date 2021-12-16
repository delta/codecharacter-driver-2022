use std::fs::File;

use std::io::Error;
use std::process::Command;
pub struct Simulator {
    main_command: &'static str,
    args: Vec<&'static str>,
}

impl Simulator {
    pub fn new(main_command: &'static str, args: Vec<&'static str>) -> Self {
        Simulator { main_command, args }
    }
    pub fn run(&self, stdin: File, stdout: File) -> Result<std::process::Child, Error> {
        Command::new(self.main_command)
            .args(&self.args)
            .stdin(stdin)
            .stdout(stdout)
            .spawn()
    }
}
