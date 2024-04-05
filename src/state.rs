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

    // Changes state to Check if King is at risk, and returns King's index
    pub(crate) fn is_king_endangered(&mut self, squares: &Squares, pieces: &mut Pieces, pred_index: &mut Vec<usize>, prey_index: &mut usize) -> State {
        let num_of_pieces: usize = pieces.locations.len();

        let mut temp = State::Play;

        let black_king_index = pieces.types.iter()
            .enumerate()
            .position(|(i, t)| *t == Type::King && *pieces.colors.get(i).unwrap() == PieceColor::Black).unwrap();

        let white_king_index = pieces.types.iter()
            .enumerate()
            .position(|(i, t)| *t == Type::King && *pieces.colors.get(i).unwrap() == PieceColor::White).unwrap();

        // Checks if each piece has a King in its kill path
        for index in 0..num_of_pieces {
            let (_, valid_kills) = pieces.possible_moves(&squares, index);
            for pnt in valid_kills {
                if &pnt == pieces.locations.get(black_king_index).unwrap() {
                    debug!("Black King in DANGER!");
                    pred_index.push(index);
                    *prey_index = black_king_index;
                    return State::Check;
                }

                if &pnt == pieces.locations.get(white_king_index).unwrap() {
                    debug!("White King in DANGER!");
                    pred_index.push(index);
                    *prey_index = white_king_index;
                    return State::Check;
                }
            }
        }
        return State::Play;
    }
    pub(crate) fn change_state(self, squares: &Squares, pieces: &mut Pieces) -> Result<(State, Vec<usize>), String> {
        todo!()
    }
}


