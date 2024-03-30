extern crate sdl2;

use sdl2::Sdl;
use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;
use sdl2::rect::Point;

const SCREEN_WIDTH: u32 = 160;
const SCREEN_HEIGHT: u32 = 144;
const WINDOW_SCALE: u32 = 4;

const BACKGROUND_WIDTH: usize = 256;
const BACKGROUND_HEIGHT: usize = 256;

pub struct Display {
    canvas: WindowCanvas,
    background: [[u8; BACKGROUND_WIDTH]; BACKGROUND_HEIGHT]
}

impl Display {

    pub fn new(sdl_context: &Sdl) -> Self {
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window(
                "Game Boy Color",
                SCREEN_WIDTH * WINDOW_SCALE,
                SCREEN_HEIGHT * WINDOW_SCALE
            )
            .position_centered()
            .build()
            .unwrap();

        Self {
            canvas: window.into_canvas().build().unwrap(),
            background: [[0; BACKGROUND_WIDTH]; BACKGROUND_HEIGHT]
        }
    }

    pub fn update(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();

        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        let _ = self.canvas.draw_line(Point::new(0, 0),
            Point::new(
                (SCREEN_WIDTH * WINDOW_SCALE / 2) as i32,
                (SCREEN_HEIGHT * WINDOW_SCALE / 2) as i32
            )
        );

        self.canvas.present();
    }

}

