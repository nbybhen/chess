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
                for event in events.poll_iter() {
                    match event {
                        Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                        Event::MouseButtonDown { x, y, .. } => {
                            let clicked = Point {
                                x: (x / (SCREEN_WIDTH / 8) as i32) as u32,
                                y: (y / (SCREEN_HEIGHT / 8) as i32) as u32,
                            };
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
                                    debug!("Valid moves: {valid_moves:?}");
                                    renderer.render_board()?;
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
