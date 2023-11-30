use crate::game_rules::entities::tetromino::Tetromino;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Game {
    pub game_over :bool,
    pub next_piece: Tetromino,
    pub current_piece: Tetromino,
    pub current_score: u64
}
