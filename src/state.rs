use crate::squares::Squares;

use crate::pieces::Pieces;
use crate::pieces::Type;
use crate::pieces::PieceColor;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum State {
    Paused,
    Play,
    Check,
}

impl State {
    pub fn change_state(self, squares: &Squares, pieces: &mut Pieces) -> Result<(State, Vec<usize>), String> {
        todo!()
    }
}


