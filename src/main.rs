use cc_driver::{cpp, error, fifo::Fifo, py, simulator};
const P1_IN: &str = "/tmp/p1_in";
const P2_IN: &str = "/tmp/p2_in";

fn handler() {
    // This is not final, its just an outline of how it should happen

    let pipe1 = Fifo::new(P1_IN.to_owned());
    let pipe2 = Fifo::new(P2_IN.to_owned());

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
                    error::handle_err(err);
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
                    error::handle_err(err);
                    return;
                }
            };

            if let Err(err) = cc_driver::handle_process(player_pid) {
                error::handle_err(err);
                return;
            }
            if let Err(err) = cc_driver::handle_process(sim_pid) {
                error::handle_err(err);
                return;
            }
            println!("Exited both process successfully");
        }

        (Err(e), _) | (_, Err(e)) => {
            cc_driver::error::handle_err(e);
        }
    }
}

fn main() {
    handler();
}
