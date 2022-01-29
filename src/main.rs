#![allow(dead_code, unused_imports)]
use std::process::exit;

use cc_driver::*;
const P1_IN: &str = "/tmp/p1_in";
const P2_IN: &str = "/tmp/p2_in";

fn handler() {
    let files = setup_pipes(P1_IN, P2_IN);
    if let Ok((p1_stdin, p1_stdout, p2_stdin, p2_stdout)) = files {
        // let player_process = py::Runner::new("tests/py/player.py".to_owned()).run(p1_stdin, p1_stdout);

        let player_process =
            cpp::Runner::new("tests/cpp/player.cpp".to_owned(), "player".to_owned())
                .run(p1_stdin, p1_stdout);

        let player_pid; //= player_process.map_err(|e| error::handle_err(e)).unwrap();

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
        let sim_pid; //= sim_process.map_err(|e| error::handle_err(e)).unwrap();
        match sim_process {
            Ok(pid) => {
                sim_pid = pid;
            }
            Err(err) => {
                error::handle_err(err);
                return;
            }
        };

        if let Err(err) = handle_process(player_pid) {
            error::handle_err(err);
            return;
        }
        if let Err(err) = handle_process(sim_pid) {
            error::handle_err(err);
            return;
        }
        println!("Exited both process successfully");
        cleanup(P1_IN, P2_IN);
    } else {
        cc_driver::error::handle_err(files.err().unwrap());
    }
}

fn main() {
    handler();
}
