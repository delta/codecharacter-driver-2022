use std::{
    fs::File,
    io::{Error, Read},
    process::{exit, Child, Command, Stdio},
};

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
    pub fn run(&self, stdin: File, stdout: File) -> Result<Child, Error> {
        let mut compile = Command::new("clang++")
            .args([
                "-fsanitize=address",
                "-O2",
                "-o",
                &self.out_name,
                &self.file_name,
            ])
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .spawn()?;
        let compile_status = compile.wait()?;
        if !compile_status.success() {
            let mut e = String::new();
            compile.stderr.unwrap().read_to_string(&mut e).unwrap();
            eprintln!("{}", e);
            //TODO, change to graceful handling
            unimplemented!()
        }
        Command::new("./".to_owned() + &self.out_name.clone())
            .stdin(stdin)
            .stdout(stdout)
            .stderr(Stdio::piped())
            .spawn()
    }
}
