use cc_driver::{
    cpp, error,
    fifo::Fifo,
    game_dir::GameDir,
    mq::{consumer, Publisher},
    request::GameRequest,
    simulator,
};
use std::{
    fs::File,
    io::{prelude::*, BufWriter},
};

// https://stackoverflow.com/questions/26958489/how-to-copy-a-folder-recursively-in-rust
fn copy_dir_all(
    src: impl AsRef<std::path::Path>,
    dst: impl AsRef<std::path::Path>,
) -> std::io::Result<()> {
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            std::fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

fn send_initial_input(fifos: Vec<&File>, game_request: &GameRequest) {
    let game_parameters = &game_request.parameters;
    for fifo in fifos {
        let mut writer = BufWriter::new(fifo);
        writer
            .write_all(
                format!(
                    "{} {}\n",
                    game_parameters.no_of_turns, game_parameters.no_of_coins
                )
                .as_bytes(),
            )
            .unwrap();
        writer
            .write_all(format!("{}\n", game_parameters.attackers.len()).as_bytes())
            .unwrap();
        for attacker in &game_parameters.attackers {
            writer
                .write_all(
                    format!(
                        "{} {} {} {} {}\n",
                        attacker.hp,
                        attacker.range,
                        attacker.attack_power,
                        attacker.speed,
                        attacker.price
                    )
                    .as_bytes(),
                )
                .unwrap();
        }
        writer
            .write_all(format!("{}\n", game_parameters.defenders.len()).as_bytes())
            .unwrap();
        for defender in &game_parameters.defenders {
            writer
                .write_all(
                    format!(
                        "{} {} {} {} {}\n",
                        defender.hp, defender.range, defender.attack_power, 0, defender.price
                    )
                    .as_bytes(),
                )
                .unwrap();
        }
        writer.write_all("64 64\n".as_bytes()).unwrap();
        for row in game_request.map.iter() {
            for cell in row.iter() {
                writer.write_all(format!("{} ", cell).as_bytes()).unwrap();
            }
            writer.write_all("\n".as_bytes()).unwrap();
        }
    }
}

fn make_copy(
    src_dir: &str,
    dest_dir: &str,
    player_code_file: &str,
    game_request: &GameRequest,
    publisher: &mut Publisher,
) -> bool {
    if let Err(e) = copy_dir_all(src_dir, dest_dir) {
        publisher
            .publish(error::handle_err(
                game_request,
                cc_driver::error::SimulatorError::UnidentifiedError(format!(
                    "Failed to copy player code boilerplate: {}",
                    e
                )),
            ))
            .unwrap();
        return false;
    }

    if let Err(e) = std::fs::File::create(player_code_file).and_then(|mut file| {
        file.write_all(game_request.source_code.as_bytes())
            .and_then(|_| file.sync_all())
    }) {
        publisher
            .publish(error::handle_err(
                game_request,
                cc_driver::error::SimulatorError::UnidentifiedError(format!(
                    "Failed to copy player code: {}",
                    e
                )),
            ))
            .unwrap();
        return false;
    }
    return true;
}

fn handler(game_request: GameRequest, publisher: &mut Publisher) {
    // This is not final, its just an outline of how it should happen

    let game_dir_handle = GameDir::new(&game_request.game_id);

    if game_dir_handle.is_none() {
        publisher
            .publish(error::handle_err(
                &game_request,
                cc_driver::error::SimulatorError::UnidentifiedError(
                    "Failed to create game directory".to_owned(),
                ),
            ))
            .unwrap();
        return;
    }

    let game_dir_handle = game_dir_handle.unwrap();

    let (to_copy_dir, player_code_file) = match game_request.language {
        cc_driver::request::Language::CPP => (
            "player_code/cpp",
            format!("{}/run.cpp", game_dir_handle.get_path()),
        ),
        cc_driver::request::Language::PYTHON => (
            "player_code/py",
            format!("{}/run.py", game_dir_handle.get_path()),
        ),
        cc_driver::request::Language::JAVA => (
            "player_code/java",
            format!("{}/Run.java", game_dir_handle.get_path()),
        ),
    };

    if !make_copy(
        to_copy_dir,
        game_dir_handle.get_path(),
        &player_code_file,
        &game_request,
        publisher,
    ) {
        return;
    }

    let p1_in = format!("{}/p1_in", game_dir_handle.get_path()).to_owned();
    let p2_in = format!("{}/p2_in", game_dir_handle.get_path()).to_owned();

    let pipe1 = Fifo::new(p1_in.to_owned());
    let pipe2 = Fifo::new(p2_in.to_owned());

    match (pipe1, pipe2) {
        (Ok(mut p1), Ok(mut p2)) => {
            let (p1_stdin, p2_stdout) = p1.get_ends().unwrap();
            let (p2_stdin, p1_stdout) = p2.get_ends().unwrap();

            send_initial_input(vec![&p1_stdout, &p2_stdout], &game_request);

            let player_process = cpp::Runner::new(format!("{}", game_dir_handle.get_path()))
                .run(p1_stdin, p1_stdout);

            let player_pid;

            match player_process {
                Ok(pid) => {
                    player_pid = pid;
                }
                Err(err) => {
                    // if there's error in publishing, might as well crash
                    publisher
                        .publish(error::handle_err(&game_request, err))
                        .unwrap();
                    return;
                }
            };

            let sim_process = simulator::Simulator::new("python3", vec!["tests/simulator.py"])
                .run(p2_stdin, p2_stdout);
            let sim_pid;
            match sim_process {
                Ok(pid) => {
                    sim_pid = pid;
                }
                Err(err) => {
                    // if there's error in publishing, might as well crash
                    publisher
                        .publish(error::handle_err(&game_request, err))
                        .unwrap();
                    return;
                }
            };

            let player_process_out = cc_driver::handle_process(player_pid);
            if let Err(err) = player_process_out {
                // error in publish means we crash
                eprint!("Error from player: {:?}", err);
                publisher
                    .publish(error::handle_err(&game_request, err))
                    .unwrap();
                return;
            }
            let player_process_out = player_process_out.unwrap();

            let sim_process_out = cc_driver::handle_process(sim_pid);
            if let Err(err) = sim_process_out {
                // error in publish means we crash
                eprint!("Error from simulator: {:?}", err);
                publisher
                    .publish(error::handle_err(&game_request, err))
                    .unwrap();
                return;
            }
            let sim_process_out = sim_process_out.unwrap();

            let response =
                cc_driver::create_final_response(game_request, player_process_out, sim_process_out);

            // crash on failure
            publisher.publish(response).unwrap();
        }

        (Err(e), _) | (_, Err(e)) => {
            error::handle_err(&game_request, e);
            return;
        }
    }
}

fn main() {
    let res = consumer(
        "amqp://guest:guest@localhost".to_owned(),
        "gameRequestQueue".to_owned(),
        "gameStatusUpdateQueue".to_owned(),
        handler,
    );
    match res {
        Ok(_) => {}
        Err(e) => {
            println!("{}", e);
        }
    }
}
