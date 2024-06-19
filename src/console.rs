use crate::api::game::{self, GameStatus, Mode, Settings};
use std::io;

pub struct ConsoleGame {
    game: game::Game,
}

impl ConsoleGame {
    pub fn new(settings: game::Settings) -> ConsoleGame {
        ConsoleGame {
            game: game::Game::new(settings),
        }
    }

    pub fn play(&mut self) {
        self.game.clear_field();
        self.print_game_info();

        loop {
            self.print_move_info();
            self.next_move();

            let status = self.game.status();

            if status != game::GameStatus::Active {
                Self::end(status);
                self.print_field();
                break;
            }
        }
    }

    fn next_move(&mut self) {
        let mov = self.get_next_move();
        self.game.place(&mov);
    }

    fn get_next_move(&mut self) -> game::Move {
        match self.game.cur_type_of_player() {
            game::PlayerType::Man => {
                let (x, y) = loop {
                    let (x, y) = Self::input_move();
                    match self.game.check_move(x, y) {
                        game::MoveStatus::Correct => break (x, y),
                        game::MoveStatus::Error(err) => self.print_move_error(err),
                    }
                };

                return game::Move {x, y};
            }
            game::PlayerType::Bot => {
                return self.game.get_best_move();
            }
        }
    }


    fn end(status: GameStatus) {
        println!("GAME OVER!");
        println!("Game ended {}", 
            if status == game::GameStatus::Draw {"in a draw"} 
            else if status == game::GameStatus::FirstWin {"with 1st player winning!"}
            else {"with 2nd player winning!"}
        )
    }

    fn print_move_info(&self) {
        println!("Move: {} Player: {}", self.game.cur_move() + 1, self.game.cur_player());
        self.print_field();
    }

    fn print_game_info(&self) {
        let settings = self.game.get_settings();
        println!("Game started in {}", match settings.mode {
            Mode::TwoPlayers => String::from("two players mode"),
            Mode::OnePlayer => format!("one player mode, bot lvl is {}", settings.bot_lvl),
        });

    }

    fn print_field(&self) {
        let field = self.game.get_field();
        for i in 0..3 {
            for j in 0..3 {
                print!("{} ", 
                    if field[i][j] == 1 {"x"}
                    else if field[i][j] == 2 {"o"}
                    else {"."}
                );
            }
            print!("\n");
        }
    }

    fn print_move_error(&self, err: game::MoveError) {
        match err {
            game::MoveError::OutOfField => 
                println!("Your move is out of field"),
            game::MoveError::AlreadyOccupied => 
                println!("This cell is already occupied"),
        }
    }

    fn input_move() -> (usize, usize) {
        let mut s = String::new();

        return loop {
            println!("Please enter your next move");
            s.clear();
            match io::stdin().read_line(&mut s) {
                Ok(res) => res,
                Err(err) => {
                    println!("Error on input {err}");
                    continue;
                },
            };

            let v: Vec<&str> = s.split_whitespace().collect();

            if v.len() != 2 {
                println!("Need 2 coordinates, found {}", v.len());
                continue; 
            }
            
            let a: usize = match v[0].parse() {
                Ok(num) => num,
                Err(err) => {
                    println!("Error on parsing '{}' {}", v[0], err);
                    continue;
                }
            };

            let b: usize = match v[1].parse() {
                Ok(num) => num,
                Err(err) => {
                    println!("Error on parsing '{}' {}", v[1], err);
                    continue;
                }
            };

            break (a, b);
        };
    }
}