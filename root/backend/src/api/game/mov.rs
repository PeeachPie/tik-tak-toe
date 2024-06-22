// Move errors
pub enum MoveError {
    OutOfField,
    AlreadyOccupied,
}
// Move status
pub enum MoveStatus {
    Correct,
    Error (MoveError),
}

#[derive(Debug)]
pub struct Move {
    x: usize,
    y: usize,
}

impl Move {
    pub fn new(x: usize, y: usize) -> Move {
        if x > 2 || y > 2 {
            panic!("Move values must be in range from 0 to 2, got x={x} y={y}");
        }
        return Move {x, y};
    }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }
}