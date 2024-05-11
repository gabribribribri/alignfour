// #![allow(dead_code)]
use std::env;

mod align_four_engine;
mod cli_wrapper;
use cli_wrapper::CLIWrapper;
use gui_wrapper::GUIWrapper;
mod gui_wrapper;

fn gui_run(width: usize, height: usize) {
    let mut wrapper = GUIWrapper::new(width, height);
    wrapper.gameloop(GUIWrapper::run);
}

fn cli_run(width: usize, height: usize) {
    if width >= 100 {
        println!("Nan ça c'est trop grand abuse pas.");
        return;
    }
    let wrapper = CLIWrapper::new(width, height);
    wrapper.run();
}

const HELP_MESSAGE: &'static str = "
    Alignfour game. Defaults to 7x6 grid.
    
    Commands:
        cli (<width> <height>)    Launch the game in CLI mod
        gui (<width> <height>)    Launch the game in GUI mod
";

fn main() {
    match (env::args().nth(1), env::args().nth(2), env::args().nth(3)) {
        (Some(command), None, None) => {
            if command == String::from("gui") {
                gui_run(7, 6);
            } else if command == String::from("cli") {
                cli_run(7, 6);
            } else {
                println!("{HELP_MESSAGE}")
            }
        }

        (Some(command), Some(width), Some(height)) => {
            if command == String::from("gui") {
                gui_run(width.parse().unwrap(), height.parse().unwrap());
            } else if command == String::from("cli") {
                cli_run(width.parse().unwrap(), height.parse().unwrap());
            } else {
                println!("{HELP_MESSAGE}")
            }
        }

        (Some(width), Some(height), None) => {
            gui_run(width.parse().unwrap(), height.parse().unwrap())
        }

        (None, None, None) => gui_run(7, 6),

        _ => println!("{HELP_MESSAGE}"),
    }
}
