extern crate sdl2;

#[macro_use]
extern crate log;

mod state;
mod squares;
mod pieces;
mod renderer;

use crate::renderer::Renderer;
use crate::squares::Squares;
use crate::state::State;

use crate::pieces::PieceColor;
use crate::pieces::Pieces;
use crate::pieces::Type;
use crate::pieces::Point;

use log::debug;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 800;

// Changes state to Check if King is at risk, and returns King's index
fn is_king_endangered(squares: &Squares, pieces: &mut Pieces, pred_index: &mut Vec<usize>) -> (bool, usize) {
    let num_of_pieces: usize = pieces.locations.len();

    let black_king_index = pieces.types.iter().enumerate().position(|(i, t)| *t == Type::King && *pieces.colors.get(i).unwrap() == PieceColor::Black).unwrap();
    let white_king_index = pieces.types.iter().enumerate().position(|(i, t)| *t == Type::King && *pieces.colors.get(i).unwrap() == PieceColor::White).unwrap();

    let mut pred_exists: bool = false;
    let mut prey_index: usize = usize::MAX;
    
    // Checks if each piece has a King in it's kill path
    for index in 0..num_of_pieces {
        let (_, valid_kills) = pieces.possible_moves(&squares, index);
        for pnt in valid_kills {
            if &pnt == pieces.locations.get(black_king_index).unwrap() {
                debug!("Black King in DANGER!");
                pred_index.push(index);
                pred_exists = true;
                prey_index = black_king_index;
            }

            if &pnt == pieces.locations.get(white_king_index).unwrap() {
                debug!("White King in DANGER!");
                pred_index.push(index);
                pred_exists = true;
                prey_index = white_king_index;
            }
        }
    } 

    (pred_exists, prey_index)
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    // Initializes the logger
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    // Creates Window
    let win = video_subsystem.window("CHESS", SCREEN_WIDTH, SCREEN_HEIGHT).position_centered().build().map_err(|e| e.to_string())?;

    // Creates Renderer struct for handling canvas renders
    let mut renderer = Renderer::new(win)?;

    // Creates vector for board squares
    let squares: Squares = Squares { squares: vec![], points: vec![] }.create().unwrap();
    let mut pieces: Pieces = Pieces { locations: vec![], colors: vec![], types: vec![], first_move: vec![] }.create().unwrap();

    // Creates Event Loop
    let mut events = sdl_context.event_pump()?;

    let _ = renderer.render_board();
    let _ = renderer.render_pieces(&squares, &pieces);

    // Presets variables (mutable)
    let mut first_click: bool = true;
    let mut current_piece_loc: Option<usize> = Default::default();
    let mut valid_moves: Vec<Point> = vec![];
    let mut valid_kills: Vec<Point> = vec![];
    let mut prey_valid_kills: Vec<Point> = vec![];
    let mut state: State = State::Play;
    let mut predators_index: Vec<usize> = vec![];
    let mut current_piece = Point{y: u32::MAX, x: u32::MAX};
    let mut prey_index: usize = usize::MAX;

    // Event Loop
    'running: loop {
        match state {
            // Checks if it is in CHECK
            State::Check => {
                 debug!("Predator(s) are {:?}", predators_index.iter().map(|x| pieces.types.get(*x).unwrap()).collect::<Vec<_>>());

                let mut danger_zone: Vec<Point> = vec![];
                let king_loc: Point = pieces.locations[prey_index]; 

                // Obtain the type of the predator pieces to get pathing
                for index in &predators_index {
                    match *pieces.types.get(*index).unwrap() {
                        Type::Bishop => {
                            let bish_loc = pieces.locations.get(*index).unwrap();
                            let (mut x, mut y) = (bish_loc.x, bish_loc.y);
                            // NE
                            if king_loc.x > bish_loc.x && king_loc.y < bish_loc.y {
                                while x < king_loc.x && y > king_loc.y {
                                    x += 1;
                                    y -= 1;
                                    danger_zone.push(Point {x, y});
                                } 
                            }
                            // NW
                            if king_loc.x < bish_loc.x && king_loc.y < bish_loc.y {
                                while x > king_loc.x && y > king_loc.y {
                                    x -= 1;
                                    y -= 1;
                                    danger_zone.push(Point {x, y});
                                }
                            }
                            // SE
                            if king_loc.x > bish_loc.x && king_loc.y > bish_loc.y {
                                while x < king_loc.x && king_loc.y < y {
                                    x += 1;
                                    y += 1;
                                    danger_zone.push(Point {x, y});
                                }
                            }
                            // SW
                            if king_loc.x < bish_loc.x && king_loc.y > bish_loc.y {
                                while x > king_loc.x && king_loc.y > y {
                                    x -= 1;
                                    y += 1;
                                    danger_zone.push(Point {x, y});
                                }
                            }
                        },

                        // Pawn just needs to highlight the King's square 
                        Type::Pawn => danger_zone.push(Point {x: king_loc.x, y: king_loc.y}),

                        Type::Rook => {
                            let rook_loc = pieces.locations.get(*index).unwrap();
                            let (mut x, mut y) = (rook_loc.x, rook_loc.y);

                            // North
                            while y > king_loc.y {
                                y -= 1;
                                danger_zone.push(Point{x, y});
                            }

                            // South
                            while y < king_loc.y {
                                y += 1;
                                danger_zone.push(Point{x, y});
                            }

                            // East
                            while x < king_loc.x {
                                x += 1;
                                danger_zone.push(Point{x, y});
                            }

                            // West
                            while x > king_loc.x {
                                x -= 1;
                                danger_zone.push(Point{x, y});
                            }
                        },

                        Type::Queen => {
                            let queen_loc = pieces.locations.get(*index).unwrap();
                            let (mut x, mut y) = (queen_loc.x, queen_loc.y);

                            // Ensures King has same Y or X value for Rook moves
                            if (x == king_loc.x || y == king_loc.y) {
                                // North
                                while y > king_loc.y {
                                    y -= 1;
                                    danger_zone.push(Point{x, y});
                                }

                                // South
                                while y < king_loc.y {
                                    y += 1;
                                    danger_zone.push(Point{x, y});
                                }

                                // East
                                while x < king_loc.x {
                                    x += 1;
                                    danger_zone.push(Point{x, y});
                                }

                                // West
                                while x > king_loc.x {
                                    x -= 1;
                                    danger_zone.push(Point{x, y});
                                }
                            }
                            else {
                                // NE
                                if king_loc.x > queen_loc.x && king_loc.y < queen_loc.y {
                                    while x < king_loc.x && y > king_loc.y {
                                        x += 1;
                                        y -= 1;
                                        danger_zone.push(Point {x, y});
                                    } 
                                }
                                // NW
                                if king_loc.x < queen_loc.x && king_loc.y < queen_loc.y {
                                    while x > king_loc.x && y > king_loc.y {
                                        x -= 1;
                                        y -= 1;
                                        danger_zone.push(Point {x, y});
                                    }
                                }
                                // SE
                                if king_loc.x > queen_loc.x && king_loc.y > queen_loc.y {
                                    while x < king_loc.x && king_loc.y < y {
                                        x += 1;
                                        y += 1;
                                        danger_zone.push(Point {x, y});
                                    }
                                }
                                // SW
                                if king_loc.x < queen_loc.x && king_loc.y > queen_loc.y {
                                    while x > king_loc.x && king_loc.y > y {
                                        x -= 1;
                                        y += 1;
                                        danger_zone.push(Point {x, y});
                                    }
                                }
                            }

                        },

                       Type::Knight => danger_zone.push(Point{ x: king_loc.x, y: king_loc.y}), 
                        
                       Type::King | _ => {unreachable!()}
                    }
                }

                renderer.render_board()?;
                renderer.render_danger_zones(&squares, &danger_zone); 
                renderer.render_pieces(&squares, &pieces)?;


                for event in events.wait_iter() {
                    match event {
                        Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                        Event::MouseButtonDown { x, y, .. } => {
                            let clicked = Point {
                                x: (x / (SCREEN_WIDTH / 8) as i32) as u32,
                                y: (y / (SCREEN_HEIGHT / 8) as i32) as u32,
                            };
                            if first_click {
                                debug!("FIRST CLICK");
                                debug!("Predators_index: {predators_index:?}");

                                // If only one piece puts King at risk,
                                // another piece can block the path or kill the predator
                                if predators_index.is_empty() {
                                    state = State::Play;
                                    break;
                                }

                                if predators_index.len() == 1 {
                                    // 1. Find all pieces of the same color as prey
                                    let prey_color = pieces.colors.get(prey_index).unwrap();
                                    let defense_pieces: Vec<usize> = pieces.colors.iter().enumerate().filter(|(_, x)| *x == prey_color).map(|(i, _)| i).collect();

                                    // Contains index to pieces that can protect
                                    let mut defenders: Vec<usize> = vec![];
                                    // 2. Check if any can kill the predator
                                    let pred_loc: Point = pieces.locations[predators_index[0]];
                                    for idx in defense_pieces {
                                        let (_, valid_kills) = pieces.possible_moves(&squares, idx);
                                        if valid_kills.iter().any(|x| *x == pred_loc) {
                                            println!("Defender type: {:?}", pieces.types[idx]);
                                            defenders.push(idx);
                                        }
                                    }
                                    debug!("Defenders: {defenders:?}");
                                    // 3. Check if any can stand in the danger_path
                                    // 4. Ensure those that pass #3 will take King out of check
                                    // 5. Only allow those to move.
                                    //let selected_index = pieces.locations.iter().position(|p| *p == clicked);

                                    if let Some(selected_idx) = pieces.locations.iter().position(|p| *p == clicked) {
                                        if defenders.contains(&selected_idx) {
                                            (_, prey_valid_kills) = pieces.possible_moves(&squares, selected_idx);
                                            current_piece = pieces.locations[selected_idx];
                                            renderer.render_board()?;
                                            renderer.render_selected(&squares, &pieces, selected_idx)?;
                                            renderer.render_kills(&squares, &prey_valid_kills)?;
                                            renderer.render_pieces(&squares, &pieces)?;
                                            first_click = false;
                                            debug!("First_click: {first_click}");
                                        }
                                    }
                                }
                                // Multiple predators
                                else {
                                    todo!()
                                }
                            }
                            else {
                                debug!("Second click!");

                                debug!("Valid Kills: {prey_valid_kills:?}");
                                if pieces.move_piece(&vec![], &prey_valid_kills, &current_piece, &clicked).unwrap() {
                                    state = State::Play;
                                    renderer.render_board()?;
                                    renderer.render_pieces(&squares, &pieces);

                                    // Empties vector
                                    predators_index = vec![];

                                }
                                else {
                                    renderer.render_board()?;
                                    renderer.render_danger_zones(&squares, &danger_zone); 
                                    renderer.render_pieces(&squares, &pieces);
                                }
                                first_click = true;
                            }
                        }
                        _ => {}
                    }
                }

            }
            State::Play => {
                for event in events.poll_iter() {
                    match event {
                        Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                        Event::MouseButtonDown { x, y, .. } => {
                            let clicked = Point {
                                x: (x / (SCREEN_WIDTH / 8) as i32) as u32,
                                y: (y / (SCREEN_HEIGHT / 8) as i32) as u32,
                            };
                            if first_click {
                                // Gets piece that's clicked on
                                debug!("FIRST CLICK");
                                //debug!("Coords: X: {:}, Y: {:}", clicked.x, clicked.y);

                                // Ensures a piece exists at tile clicked on 
                                current_piece_loc = pieces.locations.iter().position(|p| p.x == clicked.x && p.y == clicked.y);
                                current_piece = clicked;
                                let selected_type = match current_piece_loc {
                                    Some(x) => pieces.types.get(x),
                                    None => Option::None,
                                };

                                // Renders moves for selected piece
                                debug!("This piece is: {:?}", selected_type);
                                if selected_type.is_some() {
                                    (valid_moves, valid_kills) = pieces.possible_moves(&squares, current_piece_loc.unwrap());
                                    debug!("Valid moves: {valid_moves:?}");
                                    debug!("Valid kills: {valid_kills:?}");

                                    renderer.render_board()?;
                                    renderer.render_selected(&squares, &pieces, current_piece_loc.unwrap())?;
                                    renderer.render_moves(&squares, &valid_moves)?;
                                    renderer.render_kills(&squares, &valid_kills)?;
                                    renderer.render_pieces(&squares, &pieces)?;
                                    first_click = false;
                                }
                            } else {
                                debug!("SECOND CLICK");
                                //debug!("Coords: X: {:}, Y: {:}", clicked.x, clicked.y);
                                pieces.move_piece(&valid_moves, &valid_kills, &current_piece, &clicked)?;
                                renderer.render_board()?;
                                renderer.render_pieces(&squares, &pieces)?;
                                first_click = true;

                                let tup = is_king_endangered(&squares, &mut pieces, &mut predators_index);
                                if tup.0 {
                                    state = State::Check;
                                    prey_index = tup.1;
                                }

                                debug!("Prey_index: {prey_index}");

                                debug!("Current state: {state:?}");
                            }
                        }
                        _ => {}
                    }
                }
            }

            State::Paused => unreachable!(),
        }

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }

    Ok(())
}
