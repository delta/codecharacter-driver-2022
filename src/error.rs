#[derive(Debug)]
pub enum SimulatorError {
    CompilationError(String),
    RuntimeError(String),
    UnidentifiedError(String),
    FifoCreationError(String),
}

pub fn handle_err(err: SimulatorError) {
    eprintln!("{:?}", err);
    // unimplemented!()
}
