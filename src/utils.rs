use std::{
    fs::File,
    io::{BufWriter, Write},
};

use crate::{error, mq::Publisher, request::GameRequest};

// https://stackoverflow.com/questions/26958489/how-to-copy-a-folder-recursively-in-rust
pub fn copy_dir_all(
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

pub fn send_initial_input(fifos: Vec<&File>, game_request: &GameRequest) {
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

pub fn make_copy(
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
                error::SimulatorError::UnidentifiedError(format!(
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
                error::SimulatorError::UnidentifiedError(format!(
                    "Failed to copy player code: {}",
                    e
                )),
            ))
            .unwrap();
        return false;
    }
    return true;
}
