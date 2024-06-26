mod api;
mod console;
mod server;
use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// What mode to run the program in
    #[clap(value_enum, default_value_t=Mode::Client)]
    mode: Mode
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Mode {
    Client,
    Server
}

fn main() {
    
    let args = Cli::parse();

    match args.mode {
        Mode::Client => {console::start_client()},
        Mode::Server => {server::start_server()}
    }
}
