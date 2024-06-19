mod api;
mod console;

use std::io;
use crate::api::game;

fn main() {
    println!("Input your player x(1) o(2)");

    let mut player = String::new();

    io::stdin().read_line(&mut player).expect("err reading string");

    let player: u8 = player.trim().parse().expect("fuck you");

    if player != 1 && player != 2 {
        println!("fuck you");
        return;
    }

    let settings = game::Settings {
        mode: game::Mode::OnePlayer,
        bot_lvl: 10,
        bot_player: player % 2 + 1,
    };

    let mut game = console::ConsoleGame::new(settings);

    game.play();
}
