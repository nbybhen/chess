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
        let piece_count = pieces.locations.len();

        let white_king_loc = pieces.types.iter().enumerate().position(|(i, x)| *x == Type::King && pieces.colors.get(i).unwrap() == &PieceColor::White).unwrap();

        let mut predators_locs: Vec<usize> = vec![];
        let mut make_check = false;

        for index in 0..piece_count {
            let grouped = pieces.possible_moves(squares, index);
            //let moves = grouped.0;
            let kills = grouped.1;

            for point in kills {
                if point == *pieces.locations.get(white_king_loc).unwrap() {
                    warn!("KING IS IN DANGER!!!! ");
                    make_check = true;
                    predators_locs.push(index);
                }
            }
        }
        if make_check {
            Ok((State::Check, predators_locs))
        } else {
            Ok((State::Play, predators_locs))
        }
    }
}


