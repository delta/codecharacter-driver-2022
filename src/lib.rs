use std::{collections::HashMap, io::Read, os::unix::prelude::ExitStatusExt, process::Child};

use error::SimulatorError;
use log::error;
use response::{GameResult, GameStatusEnum};
pub mod cpp;
pub mod error;
pub mod fifo;
pub mod game_dir;
pub mod java;
pub mod mq;
pub mod py;
pub mod request;
pub mod response;
pub mod simulator;
pub mod utils;

// maximum size for log will be around 200KBs, everything after that is ignored
const MAXLOGSIZE: usize = 200000;
const SIGKILL: i32 = 9;
const COMPILATION_TIME_LIMIT: &'static str = "5";
const RUNTIME_TIME_LIMIT: &'static str = "10";
const COMPILATION_MEMORY_LIMIT: &'static str = "300m";
const RUNTIME_MEMORY_LIMIT: &'static str = "100m";

pub fn handle_process(
    proc: Child,
    is_player_process: bool,
    make_err: fn(String) -> SimulatorError,
) -> Result<String, SimulatorError> {
    match proc.wait_with_output() {
        Ok(out) => {
            let logs_extraction_result: Result<String, std::io::Error> = if is_player_process {
                let mut logs = String::new();
                out.stderr
                    .take(MAXLOGSIZE as u64)
                    .read_to_string(&mut logs)
                    .map(|_| logs)
            } else {
                String::from_utf8(out.stderr)
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("{}", e)))
            };
            if out.status.success() {
                match logs_extraction_result {
                    Err(e) => Err(SimulatorError::UnidentifiedError(
                        format!("Error during log extraction: {}", e).to_owned(),
                    )),
                    Ok(logs) => Ok(logs),
                }
            } else {
                if let Some(sig) = out.status.signal() {
                    if sig == SIGKILL {
                        return Err(SimulatorError::TimeOutError("Process took longer than the specified time to execute, so it was killed".to_string()));
                    }
                }

                match logs_extraction_result {
                    Err(e) => Err(SimulatorError::UnidentifiedError(
                        format!(
                            "Program exited with non zero exit code followed by error during log extraction: {}",
                            e
                        )
                        .to_owned(),
                    )),
                    Ok(logs) => Err(make_err(format!(
                        "Program exited with non zero exit code: {} ",
                        logs
                    ))),
                }
            }
        }
        Err(err) => Err(SimulatorError::UnidentifiedError(format!(
            "Waiting on Child Failed: {}",
            err
        ))),
    }
}

fn get_turnwise_logs(player_log: String) -> HashMap<usize, Vec<String>> {
    let mut turnwise_logs = HashMap::new();

    let mut processing = false;
    let mut cur_turn_no = 0;
    let mut cur_turn_logs = vec![];

    for ln in player_log.lines() {
        let ln = ln.trim();
        if !processing && ln.starts_with("TURN ") {
            processing = true;
            match ln
                .strip_prefix("TURN ")
                .and_then(|x| usize::from_str_radix(x, 10).ok())
            {
                Some(num) => cur_turn_no = num,
                None => {
                    processing = false;
                }
            }
            continue;
        }
        if processing && ln.starts_with("ENDLOG") {
            processing = false;
            turnwise_logs.insert(cur_turn_no, cur_turn_logs);
            cur_turn_logs = vec![];
            continue;
        }
        if processing {
            cur_turn_logs.push(ln.to_owned());
        }
    }
    turnwise_logs
}

pub fn create_final_response(
    game_request: request::GameRequest,
    player_log: String,
    simulator_log: String,
) -> response::GameStatus {
    let turnwise_logs = get_turnwise_logs(player_log);

    let mut final_logs = String::new();

    let mut coins_left = game_request.parameters.no_of_coins;
    let mut destruction_percentage = 0.0;

    for ln in simulator_log.lines() {
        //
        let ln = ln.trim();
        final_logs.push_str(ln);
        final_logs.push('\n');

        if ln.starts_with("TURN") {
            match ln
                .strip_prefix("TURN, ")
                .and_then(|x| usize::from_str_radix(x, 10).ok())
            {
                Some(num) => {
                    if turnwise_logs.contains_key(&num) {
                        for log in turnwise_logs.get(&num).unwrap().iter() {
                            final_logs.push_str(&format!("PRINT, {}\n", log));
                        }
                    }
                }
                None => {}
            }
            continue;
        }

        if ln.starts_with("DESTRUCTION") {
            match ln
                .strip_prefix("DESTRUCTION, ")
                .and_then(|s| s.strip_suffix("%"))
                .and_then(|x| x.parse::<f64>().ok())
            {
                Some(x) => {
                    destruction_percentage = x;
                }
                None => {}
            }
            continue;
        }
        if ln.starts_with("COINS") {
            match ln
                .strip_prefix("COINS, ")
                .and_then(|x| usize::from_str_radix(x, 10).ok())
            {
                Some(x) => coins_left = x as u32,
                None => {}
            }
        }
    }

    response::GameStatus {
        game_id: game_request.game_id,
        game_status: GameStatusEnum::EXECUTED,
        game_result: Some(GameResult {
            destruction_percentage,
            coins_used: (game_request.parameters.no_of_coins - coins_left) as u64,
            has_errors: false,
            log: final_logs,
        }),
    }
}

pub fn create_executing_response(game_request: &request::GameRequest) -> response::GameStatus {
    response::GameStatus {
        game_id: game_request.game_id.to_string(),
        game_status: GameStatusEnum::EXECUTING,
        game_result: None,
    }
}

pub fn create_error_response(
    game_request: &request::GameRequest,
    err: SimulatorError,
) -> response::GameStatus {
    error!("Error in execution: {:?}", err);
    let (err_type, error) = match err {
        SimulatorError::RuntimeError(e) => ("Runtime Error!".to_owned(), e),
        SimulatorError::CompilationError(e) => ("Compilation Error!".to_owned(), e),
        SimulatorError::FifoCreationError(e) => ("Process Communication Error!".to_owned(), e),
        SimulatorError::UnidentifiedError(e) => {
            ("Unidentified Error. Contact the POCs!".to_owned(), e)
        }
        SimulatorError::TimeOutError(e) => ("Timeout Error!".to_owned(), e),
    };

    let error = error
        .lines()
        .into_iter()
        .map(|x| format!("ERRORS, {}", x))
        .collect::<Vec<String>>()
        .join("\n");

    response::GameStatus {
        game_id: game_request.game_id.clone(),
        game_status: response::GameStatusEnum::EXECUTE_ERROR,
        game_result: Some(response::GameResult {
            destruction_percentage: 0.0,
            coins_used: 0,
            has_errors: true,
            log: format!(
                "ERRORS, ERROR TYPE: {}\nERRORS, ERROR LOG:\n{}\n",
                err_type, error
            ),
        }),
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        create_final_response, get_turnwise_logs,
        request::{GameParameters, GameRequest, Language},
        response::{GameResult, GameStatus, GameStatusEnum},
    };

    #[test]
    fn turnwise_logs_test() {
        let logs = r#"
            TURN 1
            Bug is here
            No it's here
            ENDLOG
            Nothing
            TURN 100
            Nope, it's been here the whole time
            ENDLOG
            Useless
            "#;
        let mut expected_result = vec![
            (
                1 as usize,
                vec!["Bug is here".to_owned(), "No it's here".to_owned()],
            ),
            (
                100 as usize,
                vec!["Nope, it's been here the whole time".to_owned()],
            ),
        ];
        expected_result.sort();

        let mut turnwise_logs = get_turnwise_logs(logs.to_owned())
            .into_iter()
            .collect::<Vec<(usize, Vec<String>)>>();
        turnwise_logs.sort();

        assert_eq!(turnwise_logs, expected_result);
    }

    #[test]
    fn create_final_response_test() {
        let player_logs = r#"
            TURN 1
            Bug is here
            No it's here
            ENDLOG
            Nothing
            TURN 100
            Nope, it's been here the whole time
            ENDLOG
            Useless
            "#;
        let simulator_logs = r#"TURN, 1
            COINS, 100
            DESTRUCTION, 20.0%
            TURN, 3
            COINS, 100
            DESTRUCTION, 20.0%
            TURN, 100
            DESTRUCTION, 75.0%
            COINS, 10"#;
        let dummy_game_request = GameRequest {
            game_id: "1".to_owned(),
            parameters: GameParameters {
                attackers: vec![],
                defenders: vec![],
                no_of_turns: 500,
                no_of_coins: 500,
            },
            language: Language::CPP,
            source_code: "".to_owned(),
            map: vec![vec![]],
        };

        let tot_coins = dummy_game_request.parameters.no_of_coins;
        let result = create_final_response(
            dummy_game_request,
            player_logs.to_owned(),
            simulator_logs.to_owned(),
        );

        let expected_game_status = GameStatus {
            game_id: "1".to_owned(),
            game_status: GameStatusEnum::EXECUTED,
            game_result: Some(GameResult {
                destruction_percentage: 75.0,
                coins_used: (tot_coins - 10) as u64,
                has_errors: false,
                log: "TURN, 1\nPRINT, Bug is here\nPRINT, No it's here\nCOINS, 100\nDESTRUCTION, 20.0%\nTURN, 3\nCOINS, 100\nDESTRUCTION, 20.0%\nTURN, 100\nPRINT, Nope, it's been here the whole time\nDESTRUCTION, 75.0%\nCOINS, 10\n".to_owned()
            }),
        };

        assert_eq!(expected_game_status, result);
    }
}
