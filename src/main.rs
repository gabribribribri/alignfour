mod align_four_engine;
mod cli_wrapper;
use cli_wrapper::CLIWrapper;

fn main() {
    let game = CLIWrapper::default();
    game.run();
}
