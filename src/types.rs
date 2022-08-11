use crate::Coord;
pub const NUM_PARAMS: usize = 5;

#[derive(Clone)]
pub struct Position {
    pub my_health: u8,
    pub their_health: u8,
    pub board: Board,
    pub all_bb: u128,
}
#[derive(Clone)]

pub struct Board {
    pub snakes: Vec<Snake>,
    pub food: Vec<Coord>,
}
#[derive(Clone)]

pub struct Snake {
    pub body: Vec<Coord>,
}
