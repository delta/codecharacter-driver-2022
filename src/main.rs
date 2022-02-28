use std::sync::Arc;

use cc_driver::{
    cpp, create_error_response, create_executing_response, 
    fifo::Fifo,
    game_dir::GameDir,
    java,
    mq::{consumer, Publisher},
    py,
    request::{GameRequest, Language},
    response::{ GameStatus},
    simulator,
};
use log::{error, info, LevelFilter};
use log4rs::{
    append::{
        console::{ConsoleAppender, Target},
        file::FileAppender,
    },
    config::{Appender, Config, Root},
    filter::threshold::ThresholdFilter,
};

fn handler(game_request: GameRequest) -> GameStatus {
    info!(
        "Starting execution for {} with language {:?}",
        game_request.game_id, game_request.language
    );
    let game_dir_handle = GameDir::new(&game_request.game_id);

    if game_dir_handle.is_none() {
        return create_error_response(
            &game_request,
            cc_driver::error::SimulatorError::UnidentifiedError(
                "Failed to create game directory".to_owned(),
            ),
        );
    }

    let game_dir_handle = game_dir_handle.unwrap();

    let (to_copy_dir, player_code_file) = match game_request.language {
        cc_driver::request::Language::CPP => (
            "player_code/cpp",
            format!("{}/run.cpp", game_dir_handle.get_path()),
        ),
        cc_driver::request::Language::PYTHON => (
            "player_code/python",
            format!("{}/run.py", game_dir_handle.get_path()),
        ),
        cc_driver::request::Language::JAVA => (
            "player_code/java",
            format!("{}/Run.java", game_dir_handle.get_path()),
        ),
    };

    match cc_driver::utils::make_copy(
        to_copy_dir,
        game_dir_handle.get_path(),
        &player_code_file,
        &game_request,
    ) {
        Some(resp) => {
            return resp;
        }
        _ => {}
    }

    let p1_in = format!("{}/p1_in", game_dir_handle.get_path()).to_owned();
    let p2_in = format!("{}/p2_in", game_dir_handle.get_path()).to_owned();

    let pipe1 = Fifo::new(p1_in.to_owned());
    let pipe2 = Fifo::new(p2_in.to_owned());

    match (pipe1, pipe2) {
        (Ok(mut p1), Ok(mut p2)) => {
            let (p1_stdin, p2_stdout) = p1.get_ends().unwrap();
            let (p2_stdin, p1_stdout) = p2.get_ends().unwrap();

            cc_driver::utils::send_initial_input(vec![&p1_stdout, &p2_stdout], &game_request);

            let player_process = match game_request.language {
                Language::CPP => cpp::Runner::new(format!("{}", game_dir_handle.get_path()))
                    .run(p1_stdin, p1_stdout),
                Language::PYTHON => py::Runner::new(format!("{}", game_dir_handle.get_path()))
                    .run(p1_stdin, p1_stdout),
                Language::JAVA => java::Runner::new(format!("{}", game_dir_handle.get_path()))
                    .run(p1_stdin, p1_stdout),
            };
            let player_pid;

            match player_process {
                Ok(pid) => {
                    player_pid = pid;
                }
                Err(err) => {
                    return create_error_response(&game_request, err);
                }
            };

            let sim_process = simulator::Simulator {}.run(p2_stdin, p2_stdout);
            let sim_pid;
            match sim_process {
                Ok(pid) => {
                    sim_pid = pid;
                }
                Err(err) => {
                    return create_error_response(&game_request, err);
                }
            };

            let player_process_out = cc_driver::handle_process(player_pid);
            if let Err(err) = player_process_out {
                // error in publish means we crash
                error!("Error from player.");
                return create_error_response(&game_request, err);
            }
            let player_process_out = player_process_out.unwrap();

            let sim_process_out = cc_driver::handle_process(sim_pid);
            if let Err(err) = sim_process_out {
                error!("Error from simulator.");
                return create_error_response(&game_request, err);
            }
            let sim_process_out = sim_process_out.unwrap();

            info!("Successfully executed for game {}", game_request.game_id);
            let response =
                cc_driver::create_final_response(game_request, player_process_out, sim_process_out);

            return response;
        }

        (Err(e), _) | (_, Err(e)) => {
            return create_error_response(&game_request, e);
        }
    }
}

fn worker_fn(msg_receiver: crossbeam_channel::Receiver<GameRequest>, publisher: Arc<Publisher>) {
    loop {
        match msg_receiver.recv() {
            Ok(req) => {
                publisher.publish(create_executing_response(&req)).unwrap();
                let response = handler(req);
                publisher.publish(response).unwrap();
            }
            Err(_) => {
                break;
            }
        }
    }
}

fn main() {
    let level = log::LevelFilter::Info;
    let file_path = "driver.log";

    let stderr = ConsoleAppender::builder().target(Target::Stderr).build();

    let logfile = FileAppender::builder().build(file_path).unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(level)))
                .build("stderr", Box::new(stderr)),
        )
        .build(
            Root::builder()
                .appender("logfile")
                .appender("stderr")
                .build(LevelFilter::Info),
        )
        .unwrap();

    let _handle = log4rs::init_config(config).unwrap();

    let res = consumer(
        "amqp://guest:guest@localhost".to_owned(),
        "gameRequestQueue".to_owned(),
        "gameStatusUpdateQueue".to_owned(),
        worker_fn,
    );
    match res {
        Ok(_) => {}
        Err(e) => {
            println!("{}", e);
        }
    }
}
