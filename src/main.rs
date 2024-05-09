mod align_four_engine;
mod cli_wrapper;
use cli_wrapper::CLIWrapper;
use gui_wrapper::GUIWrapper;
mod gui_wrapper;

fn gui_run() {
    let wrapper = GUIWrapper::default();
    wrapper.run();
}

fn cli_run() {
    let wrapper = CLIWrapper::default();
    wrapper.run();
}

fn main() {
    match std::env::args().nth(1) {
        Some(arg) => {
            if arg == String::from("gui") {
                gui_run()
            } else if arg == String::from("cli") {
                cli_run()
            }
        }
        None => gui_run(),
    }
}
