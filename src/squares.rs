use sdl2::rect::Rect;
use crate::pieces::Point;

pub struct Squares {
    pub squares: Vec<Rect>,
    pub points: Vec<Point>,
}

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 800;

impl Squares {
    pub fn create(mut self) -> Result<Self, String> {
        let width: u32 = SCREEN_WIDTH / 8;
        let height: u32 = SCREEN_HEIGHT / 8;
        for index in 0..64 {
            self.squares.push(Rect::new((width * (index % 8)) as i32, (height * (index / 8)) as i32, width, height));
            self.points.push(Point {
                x: (width * (index % 8)) / (SCREEN_WIDTH / 8),
                y: (height * (index / 8)) / (SCREEN_HEIGHT / 8),
            });
        }
        Ok(self)
    }
}
