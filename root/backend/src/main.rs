mod api;
mod console;

use std::io;

use crate::api::game;
use clap::{Parser, ValueEnum};

fn start(){
    println!("Input your player x(1) o(2)");

    let mut player = String::new();

    io::stdin().read_line(&mut player).expect("err reading string");

    let player: u8 = player.trim().parse().expect("fuck you");

    if player != 1 && player != 2 {
        println!("fuck you");
        return;
    }

    let settings = game::Settings::bot_game(1, 9);

    let mut game = console::ConsoleGame::new(settings);

    game.play();
}

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
        Mode::Client => {start()},
        Mode::Server => {println!("Not implemented")}
    }
}
