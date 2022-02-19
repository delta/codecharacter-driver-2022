use cc_driver::{
    cpp, error, fifo::Fifo, mq::Publisher, request::GameRequest, response::GameStatus, simulator,
};

fn handler(game_request: GameRequest, publisher: &mut Publisher) {
    // This is not final, its just an outline of how it should happen
    let p1_in = format!("/tmp/{}/p1_in", &game_request.game_id).to_owned();
    let p2_in = format!("/tmp/{}/p2_in", &game_request.game_id).to_owned();

    let pipe1 = Fifo::new(p1_in.to_owned());
    let pipe2 = Fifo::new(p2_in.to_owned());

    match (pipe1, pipe2) {
        (Ok(mut p1), Ok(mut p2)) => {
            let (p1_stdin, p2_stdout) = p1.get_ends().unwrap();
            let (p2_stdin, p1_stdout) = p2.get_ends().unwrap();
            let player_process =
                cpp::Runner::new("tests/cpp/player.cpp".to_owned(), "player".to_owned())
                    .run(p1_stdin, p1_stdout);

            let player_pid;

            match player_process {
                Ok(pid) => {
                    player_pid = pid;
                }
                Err(err) => {
                    // if there's error in publishing, might as well crash
                    publisher
                        .publish(error::handle_err(game_request, err))
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
                        .publish(error::handle_err(game_request, err))
                        .unwrap();
                    return;
                }
            };

            let player_process_out = cc_driver::handle_process(player_pid);
            if let Err(err) = player_process_out {
                // error in publish means we crash
                publisher
                    .publish(error::handle_err(game_request, err))
                    .unwrap();
                return;
            }
            let player_process_out = player_process_out.unwrap();

            let sim_process_out = cc_driver::handle_process(sim_pid);
            if let Err(err) = sim_process_out {
                // error in publish means we crash
                publisher
                    .publish(error::handle_err(game_request, err))
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
            error::handle_err(game_request, e);
            return;
        }
    }
}

fn main() {
    // handler();
}
