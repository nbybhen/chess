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
use sdl2::mouse::MouseState;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 800;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {x: u32, y: u32}

#[derive(PartialEq)]
enum PieceColor{ None, Black, White }

#[derive(Debug, PartialEq)]
enum Type {None, Pawn, Rook, Bishop, Queen, Knight, King}

struct Squares {squares: Vec<Rect>, points: Vec<Point>}
impl Squares{
    fn create(mut self) -> Result<Squares, String>{
        let width: u32 = SCREEN_WIDTH/8;
        let height: u32 = SCREEN_HEIGHT/8;
        for index in 0..64{
            self.squares.push(Rect::new((width*(index % 8)) as i32, (height*(index / 8)) as i32, width, height));
            self.points.push(Point{x: (width*(index % 8))/100, y: (height*(index / 8))/100});
        }
        Ok(self)
    }
}
struct Pieces {
    locations: Vec<Point>,
    colors: Vec<PieceColor>,
    types: Vec<Type>,
    first_move: Vec<bool>
}
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
                        //println!("RUNNING {j}");
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
                        //println!("RUNNING WHITE PAWNS {j}");
                        start_point.y = 1;
                        self.locations.push(start_point);
                        self.colors.push(PieceColor::White);
                        self.types.push(Type::Pawn);
                    }
                    2 => {
                        //println!("RUNNING CURRY FLAVORED PAWNS {j}");
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
                self.first_move.push(true);
            }
        }
        Ok(self)
    }

    fn check_by_point(&self, point_y: u32, point_x: u32) -> Option<usize>{
        self.locations.iter().position(|x| x.x == point_x && x.y == point_y)
    }

    fn possible_moves(&mut self, squares: &Squares, piece_loc: usize) -> Vec<Point>{
        let mut possible_locations: Vec<Point> = vec!();
        let piece_type = self.types.get(piece_loc).unwrap();
        let piece_point = self.locations.get(piece_loc).unwrap();
        let piece_color = self.colors.get(piece_loc).unwrap();
        let piece_first_perms = self.first_move.get(piece_loc).unwrap();

        match piece_type{
            Type::None => {}
            Type::Pawn => {
                match piece_color{
                    PieceColor::None => {}
                    PieceColor::Black => {
                        if *piece_first_perms && self.check_by_point(piece_point.y-1, piece_point.x) == Option::None{
                            possible_locations.push(Point{y: piece_point.y-1, x: piece_point.x});
                            if self.check_by_point(piece_point.y-2, piece_point.x) == Option::None{
                                possible_locations.push(Point{y: piece_point.y-2, x: piece_point.x});
                            }
                        }
                        else{
                            if piece_point.y != 0 && self.check_by_point(piece_point.y-1, piece_point.x) == Option::None{
                                possible_locations.push(Point{y: piece_point.y-1, x: piece_point.x});
                            }
                        }
                    }
                    PieceColor::White => {
                        if *piece_first_perms && self.check_by_point(piece_point.y+1, piece_point.x) == Option::None{
                            possible_locations.push(Point{y: piece_point.y+1, x: piece_point.x});
                            if self.check_by_point(piece_point.y+2, piece_point.x) == Option::None{
                                possible_locations.push(Point{y: piece_point.y+2, x: piece_point.x});
                            }
                        }
                        else{
                            if piece_point.y != 0 && self.check_by_point(piece_point.y+1, piece_point.x) == Option::None{
                                possible_locations.push(Point{y: piece_point.y+1, x: piece_point.x});
                            }
                        }
                    }
                }
                println!("LOADING POSSIBLE PAWN MOVES: {:?}", possible_locations);
                // Ensures "first move" gets two possible spaces

            }
            Type::Rook => {}
            Type::Bishop => {}
            Type::Queen => {}
            Type::Knight => {}
            Type::King => {}
        }
        possible_locations
    }
    fn move_piece(&mut self, valid_moves: &Vec<Point>, loc: usize, point: &Point) -> Result<(), String>{
        println!("MOVING PIECE");
        if valid_moves.iter().position(|x| x == point) != Option::None{
            self.locations[loc] = *point;
            self.first_move[loc] = false;
        }
        Ok(())
    }
}

struct Renderer {canvas: WindowCanvas}
impl Renderer{
    // Initializes renderer
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
                    //println!("Pawn: {:}", place.y*8+place.x);
                    let png: &Path = if *pieces.colors.get(index).unwrap() == PieceColor::Black {Path::new("sprites/Pawn.png")} else {Path::new("sprites/WhitePawn.png")};
                    let texture = texture_creator.load_texture(png)?;
                    self.canvas.copy(&texture, None,
                                     *squares.squares.get((place.y*8+place.x) as usize).unwrap())?;
                }
                Type::Rook => {
                    //println!("Rook: {:}", place.y*8+place.x);
                    let png: &Path = if *pieces.colors.get(index).unwrap() == PieceColor::Black {Path::new("sprites/Rook.png")} else {Path::new("sprites/WhiteRook.png")};
                    let texture = texture_creator.load_texture(png)?;
                    self.canvas.copy(&texture, None,
                                      *squares.squares.get((place.y*8 + place.x) as usize).unwrap()).expect("COULDNT RENDER ROOK");
                }
                Type::Bishop => {
                    //println!("Bishop: {:}", place.y*8+place.x);
                    let png: &Path = if *pieces.colors.get(index).unwrap() == PieceColor::Black {Path::new("sprites/Bishop.png")} else {Path::new("sprites/WhiteBishop.png")};
                    let texture = texture_creator.load_texture(png)?;
                    self.canvas.copy(&texture, None, *squares.squares.get((place.y*8 + place.x) as usize).unwrap()).expect("COULDNT RENDER BISHOP");
                }
                Type::Queen => {
                    //println!("Rook: {:}", place.y*8+place.x);
                    let png: &Path = if *pieces.colors.get(index).unwrap() == PieceColor::Black {Path::new("sprites/Queen.png")} else {Path::new("sprites/WhiteQueen.png")};
                    let texture = texture_creator.load_texture(png)?;
                    self.canvas.copy(&texture, None,
                                     *squares.squares.get((place.y*8 + place.x) as usize).unwrap()).expect("COULDNT RENDER ROOK");
                }
                Type::Knight => {
                    //println!("Knight: {:}", place.y*8+place.x);
                    let png: &Path = if *pieces.colors.get(index).unwrap() == PieceColor::Black {Path::new("sprites/Knight.png")} else {Path::new("sprites/WhiteKnight.png")};
                    let texture = texture_creator.load_texture(png)?;
                    self.canvas.copy(&texture, None,
                                     *squares.squares.get((place.y*8 + place.x) as usize).unwrap()).expect("COULDNT RENDER ROOK");
                }
                Type::King => {
                    //println!("King: {:}", place.y*8+place.x);
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

    fn render_selected(&mut self, square: &Squares, pieces: &Pieces, loc: usize) -> Result<(), String>{
        println!("RENDERING SELECTED SQUARE");
        self.canvas.set_draw_color(Color::RGB(179, 204, 255));
        let point = pieces.locations.get(loc).expect("CANNOT FIND PIECE LOCATION");
        self.canvas.fill_rect(*square.squares.get((point.y*8+point.x) as usize).unwrap());
        Ok(())
    }

    // Renders possible moves based on piece
    fn render_moves(&mut self, squares: &Squares, possible_moves: &Vec<Point>) -> Result<(), String>{
        println!("RENDERING MOVES");
        //println!("SQUARES: {:?}", squares.points);
        self.canvas.set_draw_color(Color::RGB(255, 235, 153));
        for item in possible_moves{
            let loc = squares.points.iter().position(|p| p.x == item.x && p.y == item.y);
            match loc{
                Some(x) => {
                    println!("AT POINT: {:?}", x);
                    self.canvas.fill_rect(*squares.squares.get(x).unwrap())?;
                },
                None => {
                    println!("POINT NOT FOUND");
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
    let mut squares: Squares = Squares{squares: vec![], points: vec![]}.create().unwrap();
    let mut pieces: Pieces = Pieces{locations: vec![], colors: vec![], types: vec![], first_move: vec![]}.create().unwrap();

    // Creates Event Loop
    let mut events = sdl_context.event_pump()?;

    let _ = renderer.render_board();
    let _ = renderer.render_pieces(&squares, &pieces);


    let mut first_click: bool = true;
    let mut loc: Option<usize> = Default::default();
    let mut valid_moves: Vec<Point> = vec!();


    // Event Loop
    'running: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::MouseButtonDown {x, y, ..} => {
                    let clicked = Point{x: (x/100) as u32, y: (y/100) as u32};
                    if first_click {
                        // Gets piece that's clicked on
                        println!("FIRST CLICK");
                        println!("Coords: X: {:}, Y: {:}", clicked.x, clicked.y);

                        // Ensures it exists
                        loc = pieces.locations.iter().position(|p| p.x == clicked.x && p.y == clicked.y);
                        let selected_type = match loc{
                            Some(x) => pieces.types.get(x).unwrap(),
                            None => &Type::None
                        };

                        // Renders moves for selected piece
                        println!("This piece is: {:?}", selected_type);
                        if *selected_type != Type::None{
                            valid_moves = pieces.possible_moves(&squares,loc.unwrap());
                            renderer.render_selected(&squares, &pieces, loc.unwrap())?;
                            renderer.render_moves(&squares, &valid_moves)?;
                            renderer.render_pieces(&squares, &pieces)?;
                            first_click = false;
                        }
                    }
                    else{
                        println!("SECOND CLICK");
                        println!("Coords: X: {:}, Y: {:}", clicked.x, clicked.y);
                        pieces.move_piece(&valid_moves, loc.unwrap(), &clicked)?;
                        renderer.render_board()?;
                        renderer.render_pieces(&squares, &pieces)?;
                        first_click = true;
                    }
                }
                _ => {}
            }
        }


        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }

    Ok(())
}
