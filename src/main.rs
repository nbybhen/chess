extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;
use sdl2::rect::Rect;
use sdl2::render::*;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 800;

struct Renderer {canvas: WindowCanvas}

impl Renderer{
    fn new(win: sdl2::video::Window) -> Result<Renderer, String>{
        let canvas = win.into_canvas().build().map_err(|e| e.to_string())?;
        Ok(Renderer{canvas})
    }

    fn createBoard(&mut self) -> Result<(), String>{
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();

        self.canvas.set_draw_color(Color::RGB(255, 0, 0));
        self.canvas.fill_rect(Rect::new(0, 0, SCREEN_WIDTH/8, SCREEN_HEIGHT/8))?;

        self.canvas.present();
        Ok(())
    }
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    // Creates Window
    let win = video_subsystem.window("CHESS", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    // Creates Renderer struct for handling canvas renders
    let mut renderer = Renderer::new(win)?;

    // Creates Event Loop
    let mut event_pump = sdl_context.event_pump()?;

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

        renderer.createBoard();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }

    Ok(())
}
