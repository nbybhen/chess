use crate::squares::Squares;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum PieceColor {
    Black,
    White,
}

#[derive(Debug, PartialEq)]
pub enum Type {
    Pawn,
    Rook,
    Bishop,
    Queen,
    Knight,
    King,
}


pub struct Pieces {
    pub locations: Vec<Point>,
    pub colors: Vec<PieceColor>,
    pub types: Vec<Type>,
    pub first_move: Vec<bool>,
}
impl Pieces {
    pub fn create(mut self) -> Result<Self, String> {
        debug!("CREATING PIECES");
        let mut start_point: Point = Point { x: 0, y: 0 };
        // Renders all beginning piece locations
        for i in 0..4 {
            for j in 0..8 {
                start_point.x = j;
                match i {
                    0 => {
                        start_point.y = 0;

                        //debug!("RUNNING {j}");
                        match j {
                            0 | 7 => self.types.push(Type::Rook),
                            1 | 6 => self.types.push(Type::Knight),
                            2 | 5 => self.types.push(Type::Bishop),
                            3 => self.types.push(Type::King),
                            4 => self.types.push(Type::Queen),
                            _ => {
                                error!("UNKNOWN")
                            }
                        }

                        self.locations.push(start_point);
                        self.colors.push(PieceColor::White);
                    }
                    1 => {
                        //debug!("RUNNING WHITE PAWNS {j}");
                        start_point.y = 1;
                        self.locations.push(start_point);
                        self.colors.push(PieceColor::White);
                        self.types.push(Type::Pawn);
                    }
                    2 => {
                        //debug!("RUNNING CURRY FLAVORED PAWNS {j}");
                        start_point.y = 6;

                        self.locations.push(start_point);
                        self.colors.push(PieceColor::Black);
                        self.types.push(Type::Pawn);
                    }
                    3 => {
                        start_point.y = 7;

                        match j {
                            0 | 7 => self.types.push(Type::Rook),
                            1 | 6 => self.types.push(Type::Knight),
                            2 | 5 => self.types.push(Type::Bishop),
                            3 => self.types.push(Type::King),
                            4 => self.types.push(Type::Queen),
                            _ => {}
                        }

                        self.locations.push(start_point);
                        self.colors.push(PieceColor::Black);
                    }
                    _ => {}
                }
                self.first_move.push(true);
            }
        }
        Ok(self)
    }

    // Checks if inputted coordinates contain a piece on the board and returns the location
    pub fn check_by_point(&self, point_y: u32, point_x: u32) -> Option<usize> {
        self.locations.iter().position(|x| x.x == point_x && x.y == point_y)
    }

    // Checks Pieces vectors to ensure either:
    // 1. Point is "open" to move (empty vs taken by same color)
    // 2. Point contains piece of opposite color
    pub fn valid_moves(&self, color: &PieceColor, pos_loc: &mut Vec<Point>, pos_kills: &mut Vec<Point>, y: u32, x: u32) -> bool {
        match self.check_by_point(y, x) {
            Some(loc) => {
                if self.colors[loc] != *color {
                    pos_kills.push(Point { y, x });
                }
                true
            }
            None => {
                pos_loc.push(Point { y, x });
                false
            }
        }
    }

    pub fn possible_moves(&self, _squares: &Squares, piece_loc: usize) -> (Vec<Point>, Vec<Point>) {
        let mut possible_locations: Vec<Point> = vec![];
        let mut possible_kills: Vec<Point> = vec![];

        // Data of the selected piece
        let piece_type = self.types.get(piece_loc).unwrap();
        let piece_point = self.locations[piece_loc];
        let piece_color = self.colors[piece_loc].clone();
        let piece_first_perms = self.first_move.get(piece_loc).unwrap();

        match piece_type {
            Type::Pawn => {
                match piece_color {
                    PieceColor::Black => {
                        // Ensures "first move" gets two possible spaces
                        if *piece_first_perms && self.check_by_point(piece_point.y - 1, piece_point.x).is_none() {
                            possible_locations.push(Point { y: piece_point.y - 1, x: piece_point.x });
                            if self.check_by_point(piece_point.y - 2, piece_point.x).is_none() {
                                possible_locations.push(Point { y: piece_point.y - 2, x: piece_point.x });
                            }
                             // Left kill
                            if piece_point.x != 0 {
                                match self.check_by_point(piece_point.y - 1, piece_point.x - 1) {
                                    Some(loc) => {
                                        if piece_color != self.colors[loc] {
                                            possible_kills.push(Point { y: piece_point.y - 1, x: piece_point.x - 1 })
                                        }
                                    }
                                    None => {}
                                }
                            }

                            // Right kill
                            if piece_point.x != 7 {
                                match self.check_by_point(piece_point.y - 1, piece_point.x + 1) {
                                    Some(loc) => {
                                        if piece_color != self.colors[loc] {
                                            possible_kills.push(Point { y: piece_point.y - 1, x: piece_point.x + 1 })
                                        }
                                    }
                                    None => {}
                                }
                            }

                    } 
                        else {
                            if piece_point.y != 0 && self.check_by_point(piece_point.y - 1, piece_point.x).is_none() {
                                possible_locations.push(Point { y: piece_point.y - 1, x: piece_point.x });
                            }

                            if piece_point.y != 0 {
                                // Left kill
                                if piece_point.x != 0 {
                                    match self.check_by_point(piece_point.y - 1, piece_point.x - 1) {
                                        Some(loc) => {
                                            if piece_color != self.colors[loc] {
                                                possible_kills.push(Point { y: piece_point.y - 1, x: piece_point.x - 1 })
                                            }
                                        }
                                        None => {}
                                    }
                                }

                                // Right kill
                                if piece_point.x != 7 {
                                    match self.check_by_point(piece_point.y - 1, piece_point.x + 1) {
                                        Some(loc) => {
                                            if piece_color != self.colors[loc] {
                                                possible_kills.push(Point { y: piece_point.y - 1, x: piece_point.x + 1 })
                                            }
                                        }
                                        None => {}
                                    }
                                }
                            }
                        }
                    }
                    PieceColor::White => {
                        // Ensures "first move" gets two possible spaces
                        if *piece_first_perms && self.check_by_point(piece_point.y + 1, piece_point.x).is_none() {
                            possible_locations.push(Point { y: piece_point.y + 1, x: piece_point.x });
                            if self.check_by_point(piece_point.y + 2, piece_point.x).is_none() {
                                possible_locations.push(Point { y: piece_point.y + 2, x: piece_point.x });
                            }
                            // Left kill
                            if piece_point.x != 0 {
                                match self.check_by_point(piece_point.y - 1, piece_point.x - 1) {
                                    Some(loc) => {
                                        if piece_color != self.colors[loc] {
                                            possible_kills.push(Point { y: piece_point.y - 1, x: piece_point.x - 1 })
                                        }
                                    }
                                    None => {}
                                }
                            }

                            // Right kill
                            if piece_point.x != 7 {
                                match self.check_by_point(piece_point.y - 1, piece_point.x + 1) {
                                    Some(loc) => {
                                        if piece_color != self.colors[loc] {
                                            possible_kills.push(Point { y: piece_point.y - 1, x: piece_point.x + 1 })
                                        }
                                    }
                                    None => {}
                                }
                            }

                        } else {
                            if piece_point.y != 7 && self.check_by_point(piece_point.y + 1, piece_point.x).is_none() {
                                possible_locations.push(Point { y: piece_point.y + 1, x: piece_point.x });
                            }

                            if piece_point.y != 0 {
                                // Left kill
                                if piece_point.x != 0 {
                                    match self.check_by_point(piece_point.y + 1, piece_point.x - 1) {
                                        Some(loc) => {
                                            if piece_color != self.colors[loc] {
                                                possible_kills.push(Point { y: piece_point.y + 1, x: piece_point.x - 1 })
                                            }
                                        }
                                        None => {}
                                    }
                                }

                                // Right kill
                                if piece_point.x != 7 {
                                    match self.check_by_point(piece_point.y + 1, piece_point.x + 1) {
                                        Some(loc) => {
                                            if piece_color != self.colors[loc] {
                                                possible_kills.push(Point { y: piece_point.y + 1, x: piece_point.x + 1 })
                                            }
                                        }
                                        None => {}
                                    }
                                }
                            }
                        }
                    }
                }
                //debug!("LOADING POSSIBLE PAWN MOVES: {:?}", possible_locations);
            }
            Type::Rook => {
                // North
                for index in (0..piece_point.y).rev() {
                    // Ensures there is no piece in the way for valid_moves
                    match self.valid_moves(&piece_color, &mut possible_locations, &mut possible_kills, index, piece_point.x) {
                        true => {
                            break;
                        }
                        false => {}
                    }
                }

                // South
                for index in piece_point.y + 1..=7 {
                    match self.valid_moves(&piece_color, &mut possible_locations, &mut possible_kills, index, piece_point.x) {
                        true => {
                            break;
                        }
                        false => {}
                    }
                }

                // East
                for index in piece_point.x + 1..=7 {
                    match self.valid_moves(&piece_color, &mut possible_locations, &mut possible_kills, piece_point.y, index) {
                        true => {
                            break;
                        }
                        false => {}
                    }
                }

                // West
                for index in (0..piece_point.x).rev() {
                    match self.valid_moves(&piece_color, &mut possible_locations, &mut possible_kills, piece_point.y, index) {
                        true => {
                            break;
                        }
                        false => {}
                    }
                }
            }
            Type::Bishop => {
                // North-west
                let mut x_clone = piece_point.x;
                let mut y_clone = piece_point.y;

                while x_clone > 0 && y_clone > 0 {
                    x_clone -= 1;
                    y_clone -= 1;

                    match self.valid_moves(&piece_color, &mut possible_locations, &mut possible_kills, y_clone, x_clone) {
                        true => {
                            break;
                        }
                        false => {}
                    }
                }

                // North-east
                y_clone = piece_point.y;
                x_clone = piece_point.x;

                while x_clone < 7 && y_clone > 0 {
                    x_clone += 1;
                    y_clone -= 1;

                    match self.valid_moves(&piece_color, &mut possible_locations, &mut possible_kills, y_clone, x_clone) {
                        true => {
                            break;
                        }
                        false => {}
                    }
                }
                // South-east
                y_clone = piece_point.y;
                x_clone = piece_point.x;

                while x_clone < 7 && y_clone < 7 {
                    x_clone += 1;
                    y_clone += 1;

                    match self.valid_moves(&piece_color, &mut possible_locations, &mut possible_kills, y_clone, x_clone) {
                        true => {
                            break;
                        }
                        false => {}
                    }
                }

                // South-west
                y_clone = piece_point.y;
                x_clone = piece_point.x;

                while x_clone > 0 && y_clone < 7 {
                    x_clone -= 1;
                    y_clone += 1;

                    match self.valid_moves(&piece_color, &mut possible_locations, &mut possible_kills, y_clone, x_clone) {
                        true => {
                            break;
                        }
                        false => {}
                    }
                }
            }
            Type::Queen => {
                // Bishop abilities
                // North-west
                let mut x_clone = piece_point.x;
                let mut y_clone = piece_point.y;

                while x_clone > 0 && y_clone > 0 {
                    x_clone -= 1;
                    y_clone -= 1;

                    match self.valid_moves(&piece_color, &mut possible_locations, &mut possible_kills, y_clone, x_clone) {
                        true => {
                            break;
                        }
                        false => {}
                    }
                }

                // North-east
                y_clone = piece_point.y;
                x_clone = piece_point.x;

                while x_clone < 7 && y_clone > 0 {
                    x_clone += 1;
                    y_clone -= 1;

                    match self.valid_moves(&piece_color, &mut possible_locations, &mut possible_kills, y_clone, x_clone) {
                        true => {
                            break;
                        }
                        false => {}
                    }
                }
                // South-east
                y_clone = piece_point.y;
                x_clone = piece_point.x;

                while x_clone < 7 && y_clone < 7 {
                    x_clone += 1;
                    y_clone += 1;

                    match self.valid_moves(&piece_color, &mut possible_locations, &mut possible_kills, y_clone, x_clone) {
                        true => {
                            break;
                        }
                        false => {}
                    }
                }

                // South-west
                y_clone = piece_point.y;
                x_clone = piece_point.x;

                while x_clone > 0 && y_clone < 7 {
                    x_clone -= 1;
                    y_clone += 1;

                    match self.valid_moves(&piece_color, &mut possible_locations, &mut possible_kills, y_clone, x_clone) {
                        true => {
                            break;
                        }
                        false => {}
                    }
                }

                // ROOK
                // North
                for index in (0..piece_point.y).rev() {
                    // Ensures there is no piece in the way for valid_moves
                    match self.valid_moves(&piece_color, &mut possible_locations, &mut possible_kills, index, piece_point.x) {
                        true => {
                            break;
                        }
                        false => {}
                    }
                }

                // South
                for index in piece_point.y + 1..=7 {
                    match self.valid_moves(&piece_color, &mut possible_locations, &mut possible_kills, index, piece_point.x) {
                        true => {
                            break;
                        }
                        false => {}
                    }
                }

                // East
                for index in piece_point.x + 1..=7 {
                    match self.valid_moves(&piece_color, &mut possible_locations, &mut possible_kills, piece_point.y, index) {
                        true => {
                            break;
                        }
                        false => {}
                    }
                }

                // West
                for index in (0..piece_point.x).rev() {
                    match self.valid_moves(&piece_color, &mut possible_locations, &mut possible_kills, piece_point.y, index) {
                        true => {
                            break;
                        }
                        false => {}
                    }
                }
            }
            Type::Knight => {
                // Above
                if piece_point.y > 1 {
                    // Upper left
                    if piece_point.x > 0 {
                        self.valid_moves(&piece_color, &mut possible_locations, &mut possible_kills, piece_point.y - 2, piece_point.x - 1);
                    }

                    // Upper right
                    if piece_point.x < 7 {
                        self.valid_moves(&piece_color, &mut possible_locations, &mut possible_kills, piece_point.y - 2, piece_point.x + 1);
                    }
                }

                // Below
                if piece_point.y < 6 {
                    // Lower left
                    if piece_point.x > 0 {
                        self.valid_moves(&piece_color, &mut possible_locations, &mut possible_kills, piece_point.y + 2, piece_point.x - 1);
                    }

                    // Lower right
                    if piece_point.x < 7 {
                        self.valid_moves(&piece_color, &mut possible_locations, &mut possible_kills, piece_point.y + 2, piece_point.x + 1);
                    }
                }

                // Left
                if piece_point.x > 1 {
                    // Upper left
                    if piece_point.y > 0 {
                        self.valid_moves(&piece_color, &mut possible_locations, &mut possible_kills, piece_point.y - 1, piece_point.x - 2);
                    }
                    // Lower left
                    if piece_point.y < 7 {
                        self.valid_moves(&piece_color, &mut possible_locations, &mut possible_kills, piece_point.y + 1, piece_point.x - 2);
                    }
                }

                // Right
                if piece_point.x < 6 {
                    // Upper right
                    if piece_point.y > 0 {
                        self.valid_moves(&piece_color, &mut possible_locations, &mut possible_kills, piece_point.y - 1, piece_point.x + 2);
                    }
                    // Lower right
                    if piece_point.y < 7 {
                        self.valid_moves(&piece_color, &mut possible_locations, &mut possible_kills, piece_point.y + 1, piece_point.x + 2);
                    }
                }
            }
            Type::King => {

                // Check if spots are in attack range of opposing pieces

                // Left
                if piece_point.x > 0 {
                    self.valid_moves(&piece_color, &mut possible_locations, &mut possible_kills, piece_point.y, piece_point.x - 1);
                }

                // Right
                if piece_point.x < 7 {
                    self.valid_moves(&piece_color, &mut possible_locations, &mut possible_kills, piece_point.y, piece_point.x + 1);
                }

                // Top
                if piece_point.y > 0 {
                    // Above
                    self.valid_moves(&piece_color, &mut possible_locations, &mut possible_kills, piece_point.y - 1, piece_point.x);

                    // North-west
                    if piece_point.x > 0 {
                        self.valid_moves(&piece_color, &mut possible_locations, &mut possible_kills, piece_point.y - 1, piece_point.x - 1);
                    }

                    // North-east
                    if piece_point.x < 7 {
                        self.valid_moves(&piece_color, &mut possible_locations, &mut possible_kills, piece_point.y - 1, piece_point.x + 1);
                    }
                }

                // Bottom
                if piece_point.y < 7 {
                    // Backwards
                    self.valid_moves(&piece_color, &mut possible_locations, &mut possible_kills, piece_point.y + 1, piece_point.x);

                    // South-west
                    if piece_point.x > 0 {
                        self.valid_moves(&piece_color, &mut possible_locations, &mut possible_kills, piece_point.y + 1, piece_point.x - 1);
                    }

                    // South-east
                    if piece_point.x < 7 {
                        self.valid_moves(&piece_color, &mut possible_locations, &mut possible_kills, piece_point.y + 1, piece_point.x + 1);
                    }
                }
            }
        }

        (possible_locations, possible_kills)
    }

    // current_piece = piece being moved
    pub fn move_piece(&mut self, valid_moves: &Vec<Point>, valid_kills: &Vec<Point>, current_piece: &Point, point: &Point) -> Result<bool, String> {
        let mut was_moved: bool = false;
        let mut current_piece_loc = self.locations.iter().position(|p| p == current_piece).unwrap();

        // Ensures piece isn't double-clicked
        if self.locations.get(current_piece_loc).unwrap() != point {
            if valid_moves.iter().any(|x| x == point) {
                debug!("MOVING PIECE");
                self.locations[current_piece_loc] = *point;
                self.first_move[current_piece_loc] = false;
                was_moved = true;
                debug!("Piece Moved successfully!");
            } 
            else if valid_kills.iter().any(|x| x == point) {
                debug!("KILLING PIECE");

                // Deletes previous piece
                let dying_piece_loc = self.locations.iter().position(|p| p == point).unwrap();
                debug!("Dying piece color: {:?}", self.colors.get(dying_piece_loc).unwrap());

                // Replaces with moved piece
                self.locations.remove(dying_piece_loc);
                self.colors.remove(dying_piece_loc);
                self.types.remove(dying_piece_loc);
                self.first_move.remove(dying_piece_loc);

                current_piece_loc = self.locations.iter().position(|p| p == current_piece).unwrap();

                self.locations[current_piece_loc] = *point;
                debug!("Changing current_piece to {point:?}");
                self.first_move[current_piece_loc] = false;

                was_moved = true;
            }
        }
        Ok(was_moved)
    }

    pub fn possible_check_moves(&mut self, squares: &Squares, piece_index: usize, danger_locations: &Vec<Point>) -> Vec<Point> {
        let piece_type = self.types.get(piece_index).unwrap();

        // Kings cannot enter danger path
        if piece_type == &Type::King {
            return vec![];
        }
        let hold = self.possible_moves(squares, piece_index);
        let possible_moves = hold.0;
        let _possible_kills = hold.1;
        let mut ret: Vec<Point> = vec![];

        // Move(s) that blocks path
        for pnt in possible_moves {
            if danger_locations.contains(&pnt) {
                ret.push(pnt);
            }
        }

        debug!("Possible Check Moves: {ret:?}");
        ret

    } 
}


