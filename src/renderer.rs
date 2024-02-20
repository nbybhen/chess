use sdl2::image::LoadTexture;

use crate::pieces::Pieces;
use crate::pieces::Type;
use crate::pieces::Point;
use crate::pieces::PieceColor;
use crate::squares::Squares;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::*;
use std::path::Path;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 800;

pub struct Renderer {
    pub canvas: WindowCanvas,
}

impl Renderer {
    // Initializes renderer
    pub fn new(win: sdl2::video::Window) -> Result<Renderer, String> {
        let canvas = win.into_canvas().build().map_err(|e| e.to_string())?;
        Ok(Renderer { canvas })
    }

    // Creates board tiles and renders them
    pub fn render_board(&mut self) -> Result<(), String> {
        self.canvas.set_draw_color(Color::RGB(172, 113, 57));
        self.canvas.clear();

        for row in 0..8 {
            for column in 0..8 {
                if row % 2 == 0 && column % 2 == 0 {
                    self.canvas.set_draw_color(Color::RGB(230, 204, 179));
                    self.canvas.fill_rect(Rect::new((column * (SCREEN_HEIGHT / 8)) as i32, (row * (SCREEN_WIDTH / 8)) as i32, SCREEN_WIDTH / 8, SCREEN_HEIGHT / 8))?;
                }
                if row % 2 == 1 && column % 2 == 1 {
                    self.canvas.set_draw_color(Color::RGB(230, 204, 179));
                    self.canvas.fill_rect(Rect::new((column * (SCREEN_HEIGHT / 8)) as i32, (row * (SCREEN_WIDTH / 8)) as i32, SCREEN_WIDTH / 8, SCREEN_HEIGHT / 8))?;
                }
            }
        }
        //self.canvas.present();
        Ok(())
    }

    // Renders pieces onto board tiles
    pub fn render_pieces(&mut self, squares: &Squares, pieces: &Pieces) -> Result<(), String> {
        let texture_creator = self.canvas.texture_creator();
        //debug!("Len of list: {:}", pieces.types.len());
        for index in 0..pieces.types.len() {
            let place = pieces.locations.get(index).unwrap();

            match pieces.types.get(index).unwrap() {
                Type::Pawn => {
                    //debug!("Pawn: {:}", place.y*8+place.x);
                    let png: &Path = if *pieces.colors.get(index).unwrap() == PieceColor::Black { Path::new("sprites/Pawn.png") } else { Path::new("sprites/WhitePawn.png") };
                    let texture = texture_creator.load_texture(png)?;
                    self.canvas.copy(&texture, None, *squares.squares.get((place.y * 8 + place.x) as usize).unwrap())?;
                }
                Type::Rook => {
                    //debug!("Rook: {:}", place.y*8+place.x);
                    let png: &Path = if *pieces.colors.get(index).unwrap() == PieceColor::Black { Path::new("sprites/Rook.png") } else { Path::new("sprites/WhiteRook.png") };
                    let texture = texture_creator.load_texture(png)?;
                    self.canvas.copy(&texture, None, *squares.squares.get((place.y * 8 + place.x) as usize).unwrap()).expect("COULDNT RENDER ROOK");
                }
                Type::Bishop => {
                    //debug!("Bishop: {:}", place.y*8+place.x);
                    let png: &Path = if *pieces.colors.get(index).unwrap() == PieceColor::Black { Path::new("sprites/Bishop.png") } else { Path::new("sprites/WhiteBishop.png") };
                    let texture = texture_creator.load_texture(png)?;
                    self.canvas.copy(&texture, None, *squares.squares.get((place.y * 8 + place.x) as usize).unwrap()).expect("COULDNT RENDER BISHOP");
                }
                Type::Queen => {
                    //debug!("Rook: {:}", place.y*8+place.x);
                    let png: &Path = if *pieces.colors.get(index).unwrap() == PieceColor::Black { Path::new("sprites/Queen.png") } else { Path::new("sprites/WhiteQueen.png") };
                    let texture = texture_creator.load_texture(png)?;
                    self.canvas.copy(&texture, None, *squares.squares.get((place.y * 8 + place.x) as usize).unwrap()).expect("COULDNT RENDER ROOK");
                }
                Type::Knight => {
                    //debug!("Knight: {:}", place.y*8+place.x);
                    let png: &Path = if *pieces.colors.get(index).unwrap() == PieceColor::Black { Path::new("sprites/Knight.png") } else { Path::new("sprites/WhiteKnight.png") };
                    let texture = texture_creator.load_texture(png)?;
                    self.canvas.copy(&texture, None, *squares.squares.get((place.y * 8 + place.x) as usize).unwrap()).expect("COULDNT RENDER ROOK");
                }
                Type::King => {
                    //debug!("King: {:}", place.y*8+place.x);
                    let png: &Path = if *pieces.colors.get(index).unwrap() == PieceColor::Black { Path::new("sprites/King.png") } else { Path::new("sprites/WhiteKing.png") };
                    let texture = texture_creator.load_texture(png)?;
                    self.canvas.copy(&texture, None, *squares.squares.get((place.y * 8 + place.x) as usize).unwrap()).expect("COULDNT RENDER KING");
                }
            }
        }
        self.canvas.present();
        Ok(())
    }

    pub fn render_selected(&mut self, square: &Squares, pieces: &Pieces, loc: usize) -> Result<(), String> {
        debug!("RENDERING SELECTED SQUARE");
        self.canvas.set_draw_color(Color::RGB(179, 204, 255));
        let point = pieces.locations.get(loc).expect("CANNOT FIND PIECE LOCATION");
        let _ = self.canvas.fill_rect(*square.squares.get((point.y * 8 + point.x) as usize).unwrap());
        Ok(())
    }

    // Renders possible moves based on piece
    pub fn render_moves(&mut self, squares: &Squares, possible_moves: &Vec<Point>) -> Result<(), String> {
        debug!("RENDERING MOVES");
        //debug!("SQUARES: {:?}", squares.points);
        self.canvas.set_draw_color(Color::RGB(255, 235, 153));
        for item in possible_moves {
            let loc = squares.points.iter().position(|p| p == item);
            match loc {
                Some(p) => {
                    //debug!("Item: {item:?}");
                    debug!("AT POINT: {:?}", p);
                    self.canvas.fill_rect(*squares.squares.get(p).unwrap())?;
                }
                None => {
                    error!("POINT NOT FOUND: {:?}", *item);
                }
            }
        }
        Ok(())
    }

    pub fn render_kills(&mut self, squares: &Squares, possible_kills: &Vec<Point>) -> Result<(), String> {
        debug!("RENDERING KILLS");
        self.canvas.set_draw_color(Color::RGB(255, 51, 51));
        for item in possible_kills {
            let loc = squares.points.iter().position(|p| p.x == item.x && p.y == item.y);
            match loc {
                Some(x) => {
                    debug!("KILL AT POINT: {:?}", x);
                    self.canvas.fill_rect(*squares.squares.get(x).unwrap())?;
                }
                None => {
                    error!("KILL POINT NOT FOUND");
                }
            }
        }
        Ok(())
    }

    pub fn render_as_pred(&mut self, squares: &Squares, path: &Vec<Point>) {
        debug!("Setting predators to GREEN");
        println!("Path: {path:?}");
        // Sets predators to green
        self.canvas.set_draw_color(Color::RGB(75, 200, 10));
        for point in path.clone() {
            self.canvas.fill_rect(*squares.squares.get((point.y * 8 + point.x) as usize).unwrap()).unwrap();
        }
        //self.render_board();
    }
}


