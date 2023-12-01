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
use sdl2::image::InitFlag;
use sdl2::keyboard::Keycode;
use std::time::Duration;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 800;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;

    // Initializes the logger
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
    let mut loc: Option<usize> = Default::default();
    let mut valid_moves: Vec<Point> = vec![];
    let mut valid_kills: Vec<Point> = vec![];
    let mut state: State = State::Play;
    let mut predators: Vec<usize> = vec![];

    // Event Loop
    'running: loop {
        match state {
            // Checks if it is in CHECK
            State::Check => {
                let mut danger_path: Vec<Point> = vec![];
                for item in predators.clone() {
                    let point = pieces.locations.get(item).unwrap();
                    let pred_loc = squares.points.iter().position(|p| p.x == point.x && p.y == point.y);
                    match pred_loc {
                        Some(_) => {
                            debug!("FOUND PREDATOR");
                            let mut check = false;

                            // Sets path to KING as green as well
                            match pieces.types.get(item).unwrap() {
                                Type::Bishop => {
                                    let mut x_clone = point.x;
                                    let mut y_clone = point.y;
                                    while !check {
                                        // North-East
                                        while x_clone < 7 && y_clone > 0 {
                                            match pieces.check_by_point(y_clone - 1, x_clone + 1) {
                                                Some(loc) => {
                                                    if pieces.types.get(loc).unwrap() == &Type::King {
                                                        debug!("King found!");
                                                        check = true;
                                                        danger_path.push(Point { x: x_clone, y: y_clone });
                                                        // color the points
                                                    }
                                                    break;
                                                }
                                                None => danger_path.push(Point { x: x_clone, y: y_clone }),
                                            }
                                            y_clone -= 1;
                                            x_clone += 1;
                                        }
                                    }
                                }

                                _ => {}
                            }
                        }
                        None => {
                            error!("CANNOT FIND PREDATOR");
                        }
                    }
                }

                renderer.render_as_pred(&squares, &danger_path);
                let _ = renderer.render_pieces(&squares, &pieces);
                for event in events.wait_iter() {
                    match event {
                        Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,

                        Event::MouseButtonDown { x, y, .. } => {
                            let clicked = Point {
                                x: (x / (SCREEN_WIDTH / 8) as i32) as u32,
                                y: (y / (SCREEN_HEIGHT / 8) as i32) as u32,
                            };
                            let _allowed = false;

                            // Find the path from pred -> king

                            let white_king_index = pieces.types.iter().enumerate().position(|(i, x)| *x == Type::King && pieces.colors.get(i).unwrap() == &PieceColor::White).unwrap();

                            let white_king_point = pieces.locations.get(white_king_index).unwrap();
                            if first_click {
                                // Ensures selected piece exists
                                loc = pieces.locations.iter().position(|p| p.x == clicked.x && p.y == clicked.y);
                                let selected_type = match loc {
                                    Some(x) => pieces.types.get(x),
                                    None => Option::None,
                                };

                                // Renders moves for selected piece
                                debug!("This piece is: {:?}", selected_type);
                                if selected_type.is_some() {
                                    // Highlights legal moves for selected piece to remove King from check
                                    let moves = pieces.possible_check_moves(&squares, loc.unwrap(), &danger_path);
                                    renderer.render_selected(&squares, &pieces, loc.unwrap())?;
                                    renderer.render_moves(&squares, &moves)?;
                                    renderer.render_pieces(&squares, &pieces)?;
                                    first_click = false;
                                }
                                
                                // Allow for only moves onto the mask or for King to move onto a
                                // different path
                            }
                            else {
                                println!("SECOND CLICK");
                                renderer.render_board()?;
                                renderer.render_pieces(&squares, &pieces)?;
                                first_click = true;
                            }
                            // Mask the path (hashset) to king
                            // Allow for only moves onto the mask or for King to move onto a
                            // different path
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

                                // Ensures it exists
                                loc = pieces.locations.iter().position(|p| p.x == clicked.x && p.y == clicked.y);
                                let selected_type = match loc {
                                    Some(x) => pieces.types.get(x),
                                    None => Option::None,
                                };

                                // Renders moves for selected piece
                                debug!("This piece is: {:?}", selected_type);
                                if selected_type.is_some() {
                                    let pair = pieces.possible_moves(&squares, loc.unwrap());
                                    valid_moves = pair.0;
                                    valid_kills = pair.1;
                                    renderer.render_selected(&squares, &pieces, loc.unwrap())?;
                                    renderer.render_moves(&squares, &valid_moves)?;
                                    renderer.render_kills(&squares, &valid_kills)?;
                                    renderer.render_pieces(&squares, &pieces)?;
                                    first_click = false;
                                }
                            } else {
                                debug!("SECOND CLICK");
                                //debug!("Coords: X: {:}, Y: {:}", clicked.x, clicked.y);
                                pieces.move_piece(&valid_moves, &valid_kills, loc.unwrap(), &clicked)?;
                                renderer.render_board()?;
                                renderer.render_pieces(&squares, &pieces)?;
                                let temp = state.change_state(&squares, &mut pieces)?;
                                state = temp.0;
                                predators = temp.1;
                                first_click = true;

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
