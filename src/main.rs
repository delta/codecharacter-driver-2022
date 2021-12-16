#![allow(dead_code, unused_imports)]
use cc_driver::*;

fn main() {
    let (p1_stdin, p1_stdout, p2_stdin, p2_stdout) = setup_pipes();

    let player_process = py::Runner::new("tests/py/player.py".to_owned()).run(p1_stdin, p1_stdout);

    // let player_process = cpp::Runner::new("tests/cpp/player.cpp".to_owned(), "player".to_owned())
    // .run(p1_stdin, p1_stdout);

    let sim_process =
        simulator::Simulator::new("python3", vec!["tests/simulator.py"]).run(p2_stdin, p2_stdout);

    let processes = match (player_process, sim_process) {
        (Ok(p1), Ok(p2)) => Ok((p1, p2)),
        (Err(e), _) => Err(e),
        (_, Err(e)) => Err(e),
    };

    if let Ok((process_1, process_2)) = processes {
        let y = handle_process(process_2);
        let x = handle_process(process_1);
        println!("p1 status: {:?}, p2 status: {:?}", x, y);
    } else {
        //TODO: proper error handling
        eprintln!("{:?}", processes.unwrap_err());
        println!("Bye");
    }
    cleanup();
}
