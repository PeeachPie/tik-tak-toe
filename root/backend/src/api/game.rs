pub mod mov;
pub mod settings;
pub use crate::api::game::mov::*;
pub use crate::api::game::settings::*;

use rand::Rng;

// Gaming modes
pub enum Mode {
    OnePlayer,
    TwoPlayers,
}

// Player type
pub enum PlayerType {
    Bot,
    Man,
}

// Game status
#[derive(Debug)]
#[derive(PartialEq, Eq)]
pub enum EndStatus {
    FirstWin,
    SecondWin,
    Draw,
}
#[derive(Debug)]
#[derive(PartialEq, Eq)]
pub enum GameStatus {
    Active,
    End(EndStatus)
}



// Diagonals
const FROM_UP_LEFT: u8 = 0;
const FROM_UP_RIGHT: u8 = 1;

// Field type
const EMPTY: u8 = 0;
pub struct Game {
    field: [[u8; 3]; 3],
    settings: Settings,
}

impl Game {
    pub fn new(settings: Settings) -> Game {
        Game {
            settings,
            ..Game::default()
        }
    }

    pub fn default() -> Game {
        Game {
            field: [
                [0, 0, 0],
                [0, 0, 0],
                [0, 0, 0],
            ],
            settings: Settings {
                ..Settings::default()
            }
        }
    }

    pub fn get_field(&self) -> [[u8; 3]; 3] {
        return self.field;
    }

    pub fn get_settings(&self) -> &Settings {
        return &self.settings;
    }

    pub fn clear_field(&mut self) {
        for i in 0..3 {
            for j in 0..3 {
                self.field[i][j] = EMPTY;
            }
        }
    }

    pub fn cur_player(&self) -> u8 {
        return (self.cur_move() % 2) + 1;
    }

    pub fn cur_type_of_player(&self) -> PlayerType {
        match self.settings.mode {
            Mode::OnePlayer => {
                if self.cur_player() == self.settings.bot_player {
                    PlayerType::Bot
                }
                else {
                    PlayerType::Man
                }
            },
            Mode::TwoPlayers => PlayerType::Man,
        }
    }

    pub fn prev_player(&self) -> u8 {
        return (self.cur_move() + 1) % 2 + 1;
    }

    pub fn cur_move(&self) -> u8 {
        let mut cnt = 0;
        for i in 0..3 {
            for j in 0..3 {
                if self.field[i][j] != EMPTY {
                    cnt += 1;
                }
            }
        }

        return cnt;
    }

    pub fn get_best_move(&mut self) -> Move {
        let mut win_moves: Vec<Move> = Vec::new();
        let mut draw_moves: Vec<Move> = Vec::new();

        for mov in self.possible_moves() {
            let st = self.status_in(&mov, self.settings.bot_lvl, self.cur_player());
            let win = self.win_for(self.cur_player());
            let lose = self.lose_for(self.cur_player());
            println!("st {:?} mov {:?}",st, mov);

            if st == win {
                win_moves.push(mov);
            }
            else if st != lose {
                draw_moves.push(mov);
            }
        }

        let mut best_move: Move = self.choose_random_move(&self.possible_moves());

        if win_moves.len() != 0 {
            best_move = self.choose_random_move(&win_moves);
        }
        else if draw_moves.len() != 0 {
            if self.cur_move() == 0 && self.settings.bot_lvl != 0 {
                best_move = Move::new(0, 0);
            }
            else {
                best_move = self.choose_random_move(&draw_moves);
            }
        }

        println!("bst move {:?}", best_move);

        return best_move;
    }

    pub fn check_move(&self, x:usize, y: usize) -> MoveStatus {
        return 
            if x > 2 || y > 2 {
                MoveStatus::Error(MoveError::OutOfField)
            }
            else if self.field[y][x] != EMPTY {
                MoveStatus::Error(MoveError::AlreadyOccupied)
            }
            else {
                MoveStatus::Correct
            };
    }

    fn possible_moves(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();

        for i in 0..3 {
            for j in 0..3 {
                if self.field[i][j] == EMPTY {
                    moves.push(Move::new(j, i));
                }
            }
        }

        return moves;
    }

    pub fn place(&mut self, mov: &Move) {
        self.field[mov.y()][mov.x()] = self.cur_player();
    }

    fn remove(&mut self, mov: &Move) {
        self.field[mov.y()][mov.x()] = EMPTY;
    }

    fn lose_for(&self, player: u8) -> GameStatus {
        return if player == 1 {
            GameStatus::End(EndStatus::SecondWin)
        } else {
            GameStatus::End(EndStatus::FirstWin)
        };
    }

    fn win_for(&self, player: u8) -> GameStatus {
        return if player == 1 {
            GameStatus::End(EndStatus::FirstWin)
        } else {
            GameStatus::End(EndStatus::SecondWin)
        };
    }

    fn status_in(&mut self, mov: &Move, cur_depth: u8, player: u8) -> GameStatus {
        self.place(mov);
        let status = self.status();

        if (cur_depth == 0) || (status != GameStatus::Active) {
            self.remove(&mov);
            return status;
        }

        let mut sts: Vec<GameStatus> = Vec::new();

        for mov in self.possible_moves() {
            sts.push(self.status_in(&mov, cur_depth - 1, player));
        }

        let mut res = if self.cur_player() == player {
            self.lose_for(player)
        } else {
            self.win_for(player)
        };

        if self.cur_player() == player {
            for st in sts {
                let win = self.win_for(player);
                let lose = self.lose_for(player);
                if st == win {
                    res = st;
                }
                else if res == lose {
                    res = st;
                }
            }
        }
        else {
            for st in sts {
                let win = self.win_for(player);
                let lose = self.lose_for(player);
                if st == lose {
                    res = st;
                }
                else if res == win {
                    res = st;
                }
            }
        }

        self.remove(mov);

        return res;
    }

    fn choose_random_move(&self, moves: &Vec<Move>) -> Move {
        let rand_id = rand::thread_rng().gen_range(0..moves.len());
        let mov = Move::new(moves[rand_id].x(), moves[rand_id].y());
        return mov;
    }

    fn has_free_cell(&self) -> bool {
        return self.cur_move() < 9;
    }

    fn is_win_row(&self, row: usize) -> bool {
        return 
            self.field[row][0] == self.field[row][1] &&
            self.field[row][1] == self.field[row][2] &&
            self.field[row][0] != EMPTY;
    }

    fn is_win_column(&self, column: usize) -> bool {
        return
            self.field[0][column] == self.field[1][column] &&
            self.field[1][column] == self.field[2][column] &&
            self.field[0][column] != EMPTY;
    }

    fn is_win_diagonal(&self, diagonal: u8) -> bool {
        return 
            if diagonal == FROM_UP_LEFT {
                self.field[0][0] == self.field[1][1] &&
                self.field[1][1] == self.field[2][2]
            }
            else {
                self.field[0][2] == self.field[1][1] &&
                self.field[1][1] == self.field[2][0]
            } && self.field[1][1] != EMPTY;
    }

    fn is_win_position(&self) -> bool {
        let mut win_pos = false;

        for row in 0..3 {
            win_pos |= self.is_win_row(row);
        }

        for column in 0..3 {
            win_pos |= self.is_win_column(column);
        }

        win_pos |= self.is_win_diagonal(FROM_UP_LEFT);
        win_pos |= self.is_win_diagonal(FROM_UP_RIGHT);

        return win_pos;
    }

    pub fn status(&self) -> GameStatus {
        return 
            if self.is_win_position() {
                if self.prev_player() == 1 { 
                    GameStatus::End(EndStatus::FirstWin) 
                } else { 
                    GameStatus::End(EndStatus::SecondWin)
                }
            }
            else {
                if self.has_free_cell() { 
                    GameStatus::Active 
                } else { 
                    GameStatus::End(EndStatus::Draw) 
                }
            };
    }
}