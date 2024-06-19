use rand::Rng;

// Move errors
pub enum MoveError {
    OutOfField,
    AlreadyOccupied,
}
pub enum MoveStatus {
    Correct,
    Error (MoveError),
}

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
#[derive(PartialEq, Eq)]
pub enum GameStatus{
    Active,
    FirstWin,
    SecondWin,
    Draw
}
// pub const ACTIVE: u8 = 0;
// pub const FIRST_WIN: u8 = 1;
// pub const SECOND_WIN: u8 = 2;
// pub const DRAW: u8 = 3;

// Diagonals
const FROM_UP_LEFT: u8 = 0;
const FROM_UP_RIGHT: u8 = 1;

// Field type
const EMPTY: u8 = 0;

pub struct Settings {
    pub mode: Mode,
    pub bot_player: u8,
    pub bot_lvl: u8,
}

impl Settings {
    pub fn default() -> Settings {
        Settings {
            mode: Mode::OnePlayer,
            bot_player: 2,
            bot_lvl: 0,
        }
    }
}

#[derive(Debug)]
pub struct Move {
    pub x: usize,
    pub y: usize,
}

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
            // println!("st {st} mov {:?}", mov);
            if st == GameStatus::Draw || st == GameStatus::Active {
                draw_moves.push(mov);
            }
            else if st == self.win_for(self.cur_player()) {
                win_moves.push(mov);
            }
        }

        let mut best_move: Move = self.choose_random_move(&self.possible_moves());

        if win_moves.len() != 0 {
            best_move = self.choose_random_move(&win_moves);
        }
        else if draw_moves.len() != 0 {
            if self.cur_move() == 0 && self.settings.bot_lvl != 0 {
                best_move = Move {x: 1, y: 1};
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
                    moves.push(Move {x: j, y: i})
                }
            }
        }

        return moves;
    }

    pub fn place(&mut self, mov: &Move) {
        self.field[mov.y][mov.x] = self.cur_player();
    }

    fn remove(&mut self, mov: &Move) {
        self.field[mov.y][mov.x] = EMPTY;
    }

    fn lose_for(&self, player: u8) -> GameStatus {
        return if player == 1 {
            GameStatus::SecondWin
        } else {
            GameStatus::FirstWin
        };
    }

    fn win_for(&self, player: u8) -> GameStatus {
        return if player == 1 {
            GameStatus::FirstWin
        } else {
            GameStatus::SecondWin
        };
    }

    fn status_in(&mut self, mov: &Move, cur_depth: u8, player: u8) -> GameStatus {
        self.place(mov);
        let status = self.status();
        if (status != GameStatus::Active) || (cur_depth == 0) {
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
                if st == self.win_for(player) {
                    res = st;
                }
                else if res == self.lose_for(player) {
                    res = st;
                }
            }
        }
        else {
            for st in sts {
                if st == self.lose_for(player) {
                    res = st;
                }
                else if res == self.win_for(player) {
                    res = st;
                }
            }
        }

        self.remove(mov);

        return res;
    }

    fn choose_random_move(&self, moves: &Vec<Move>) -> Move {
        let random_id = rand::thread_rng().gen_range(0..moves.len());
        let mov = Move {x: moves[random_id].x, y: moves[random_id].y};
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
                if self.prev_player() == 1 { GameStatus::FirstWin } else { GameStatus::SecondWin }
            }
            else {
                if self.has_free_cell() { GameStatus::Active } else { GameStatus::Draw }
            };
    }
}