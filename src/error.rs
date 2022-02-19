use crate::{request, response};
#[derive(Debug)]
pub enum SimulatorError {
    CompilationError(String),
    RuntimeError(String),
    UnidentifiedError(String),
    FifoCreationError(String),
}

pub fn handle_err(game_request: request::GameRequest, err: SimulatorError) -> response::GameStatus {
    eprintln!("{:?}", err);
    let (err_type, error) = match err {
        SimulatorError::RuntimeError(e) => ("Runtime Error!!".to_owned(), e),
        SimulatorError::CompilationError(e) => ("Compilation Error!!".to_owned(), e),
        SimulatorError::FifoCreationError(e) => ("Process Communication Error!!".to_owned(), e),
        SimulatorError::UnidentifiedError(e) => {
            ("Unidentified Error. Contact the POCs!!!!".to_owned(), e)
        }
    };
    response::GameStatus {
        game_id: game_request.game_id.clone(),
        game_status: response::GameStatusEnum::EXECUTE_ERROR,
        game_result: Some(response::GameResult {
            destruction_percentage: 0.0,
            coins_used: 0,
            has_errors: true,
            log: format!(
                "ERROR TYPE: {}\n\nERROR LOG\n{}\nERROR TYPE : {}\n",
                err_type, error, err_type
            ),
        }),
    }
}
