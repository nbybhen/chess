extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;
use sdl2::rect::Rect;
use sdl2::render::*;
use std::env;
use std::path::Path;
use sdl2::image::{InitFlag, LoadTexture};

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 800;

#[derive(Clone, Copy)]
struct Point {x: u32, y: u32}

#[derive(PartialEq)]
enum PieceColor{ None, Black, White }

enum Type {None, Pawn, Rook, Bishop, Queen, Knight, King}

struct Squares {squares: Vec<Rect>, points: Vec<Point>}
impl Squares{
    fn create(mut self) -> Result<Squares, String>{
        let width: u32 = SCREEN_WIDTH/8;
        let height: u32 = SCREEN_HEIGHT/8;
        for index in 0..64{
            self.squares.push(Rect::new((width*(index % 8)) as i32, (height*(index / 8)) as i32, width, height));
        }
        Ok(self)
    }
}
struct Pieces {locations: Vec<Point>, colors: Vec<PieceColor>, types: Vec<Type>}
impl Pieces{
    fn create(mut self) -> Result<Pieces, String>{
        let mut start_point: Point = Point { x: 0, y: 0 };
        // Renders all beginning piece locations
        for i in 0..4{
            for j in 0..8{
                start_point.x = j;
                match i{
                    0 => {
                        start_point.y = 0;
                        println!("RUNNING {j}");
                        match j{
                            0 | 7 => self.types.push(Type::Rook),
                            1 | 6 => self.types.push(Type::Knight),
                            2 | 5 => self.types.push(Type::Bishop),
                            3 => self.types.push(Type::King),
                            4 => self.types.push(Type::Queen),
                            _ => {println!("UNKNOWN")}
                        }

                        self.locations.push(start_point);
                        self.colors.push(PieceColor::White);
                    }
                    1 => {
                        println!("RUNNING PAWNS {j}");
                        start_point.y = 1;
                        self.locations.push(start_point);
                        self.colors.push(PieceColor::White);
                        self.types.push(Type::Pawn);
                    }
                    2 => {
                        start_point.y = 6;

                        self.locations.push(start_point);
                        self.colors.push(PieceColor::Black);
                        self.types.push(Type::Pawn);
                    }
                    3 => {
                        start_point.y = 7;

                        match j{
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
            }
        }
        Ok(self)
    }
}

struct Renderer {canvas: WindowCanvas}
impl Renderer{
    fn new(win: sdl2::video::Window) -> Result<Renderer, String>{
        let canvas = win.into_canvas().build().map_err(|e| e.to_string())?;
        Ok(Renderer{canvas})
    }

    // Creates board tiles and renders them
    fn render_board(&mut self) -> Result<(), String>{
        self.canvas.set_draw_color(Color::RGB(172, 113, 57));
        self.canvas.clear();

        for row in 0..8{
            for column in 0..8{
                if row%2 == 0 && column%2 == 0{
                    self.canvas.set_draw_color(Color::RGB(230, 204, 179));
                    self.canvas.fill_rect(Rect::new(column*100, row*100, SCREEN_WIDTH/8, SCREEN_HEIGHT/8))?;
                }
                if row%2 == 1 && column%2 == 1{
                    self.canvas.set_draw_color(Color::RGB(230, 204, 179));
                    self.canvas.fill_rect(Rect::new(column*100, row*100, SCREEN_WIDTH/8, SCREEN_HEIGHT/8))?;
                }
            }
        }
        self.canvas.present();
        Ok(())
    }

    // Renders pieces onto board tiles
    fn render_pieces(&mut self, squares: &Squares, pieces: &Pieces) -> Result<(), String>{
        let texture_creator = self.canvas.texture_creator();
        println!("Len of list: {:}", pieces.types.len());
        for index in 0..pieces.types.len(){
            let place = pieces.locations.get(index).unwrap();

            match pieces.types.get(index).unwrap(){
                Type::None => {}
                Type::Pawn => {
                    println!("Pawn: {:}", place.y*8+place.x);
                    let png: &Path = if *pieces.colors.get(index).unwrap() == PieceColor::Black {Path::new("sprites/Pawn.png")} else {Path::new("sprites/WhitePawn.png")};
                    let texture = texture_creator.load_texture(png)?;
                    self.canvas.copy(&texture, None,
                                     *squares.squares.get((place.y*8+place.x) as usize).unwrap())?;
                }
                Type::Rook => {
                    println!("Rook: {:}", place.y*8+place.x);
                    let png: &Path = if *pieces.colors.get(index).unwrap() == PieceColor::Black {Path::new("sprites/Rook.png")} else {Path::new("sprites/WhiteRook.png")};
                    let texture = texture_creator.load_texture(png)?;
                    self.canvas.copy(&texture, None,
                                      *squares.squares.get((place.y*8 + place.x) as usize).unwrap()).expect("COULDNT RENDER ROOK");
                }
                Type::Bishop => {
                    println!("Bishop: {:}", place.y*8+place.x);
                    let png: &Path = if *pieces.colors.get(index).unwrap() == PieceColor::Black {Path::new("sprites/Bishop.png")} else {Path::new("sprites/WhiteBishop.png")};
                    let texture = texture_creator.load_texture(png)?;
                    self.canvas.copy(&texture, None, *squares.squares.get((place.y*8 + place.x) as usize).unwrap()).expect("COULDNT RENDER BISHOP");
                }
                Type::Queen => {
                    println!("Rook: {:}", place.y*8+place.x);
                    let png: &Path = if *pieces.colors.get(index).unwrap() == PieceColor::Black {Path::new("sprites/Queen.png")} else {Path::new("sprites/WhiteQueen.png")};
                    let texture = texture_creator.load_texture(png)?;
                    self.canvas.copy(&texture, None,
                                     *squares.squares.get((place.y*8 + place.x) as usize).unwrap()).expect("COULDNT RENDER ROOK");
                }
                Type::Knight => {
                    println!("Knight: {:}", place.y*8+place.x);
                    let png: &Path = if *pieces.colors.get(index).unwrap() == PieceColor::Black {Path::new("sprites/Knight.png")} else {Path::new("sprites/WhiteKnight.png")};
                    let texture = texture_creator.load_texture(png)?;
                    self.canvas.copy(&texture, None,
                                     *squares.squares.get((place.y*8 + place.x) as usize).unwrap()).expect("COULDNT RENDER ROOK");
                }
                Type::King => {
                    println!("King: {:}", place.y*8+place.x);
                    let png: &Path = if *pieces.colors.get(index).unwrap() == PieceColor::Black {Path::new("sprites/King.png")} else {Path::new("sprites/WhiteKing.png")};
                    let texture = texture_creator.load_texture(png)?;
                    self.canvas.copy(&texture, None,
                                     *squares.squares.get((place.y*8 + place.x) as usize).unwrap()).expect("COULDNT RENDER KING");
                }
            }
        }

        self.canvas.present();

        Ok(())
    }
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;


    // Creates Window
    let win = video_subsystem.window("CHESS", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    // Creates Renderer struct for handling canvas renders
    let mut renderer = Renderer::new(win)?;

    // Creates vector for board squares
    let squares: Squares = Squares{squares: vec![], points: vec![]}.create().unwrap();
    let pieces: Pieces = Pieces{locations: vec![], colors: vec![], types: vec![]}.create().unwrap();

    // Creates Event Loop
    let mut event_pump = sdl_context.event_pump()?;

    let _ = renderer.render_board();
    let _ = renderer.render_pieces(&squares, &pieces);

    // Event Loop
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }


        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }

    Ok(())
}
