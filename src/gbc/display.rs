extern crate sdl2;

use sdl2::Sdl;
use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;

use super::Memory;

const LCD_WIDTH: u32 = 160;
const LCD_HEIGHT: u32 = 144;
const WINDOW_SCALE: u32 = 4;

const BACKGROUND_WIDTH: usize = 256;
const BACKGROUND_HEIGHT: usize = 256;

pub struct Display {
    canvas: WindowCanvas,

    background: [[u8; BACKGROUND_WIDTH]; BACKGROUND_HEIGHT],
    current_pixel: (u8, u8)
}

impl Display {

    pub fn new(sdl_context: &Sdl) -> Self {
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window(
                "Game Boy Color",
                LCD_WIDTH * WINDOW_SCALE,
                LCD_HEIGHT * WINDOW_SCALE
            )
            .position_centered()
            .build()
            .unwrap();

        Self {
            canvas: window.into_canvas().build().unwrap(),
            background: [[0; BACKGROUND_WIDTH]; BACKGROUND_HEIGHT],
            current_pixel: (0, 0)
        }
    }

    pub fn update(&mut self, memory: &Memory) {
        let lcdc = memory.get_lcdc();

        // Is screen enabled?
        if (lcdc >> 7) != 0 {
            let bg_x = (memory.get_scx() + self.current_pixel.0) % (BACKGROUND_WIDTH as u8);
            let bg_y = (memory.get_scy() + self.current_pixel.1) % (BACKGROUND_HEIGHT as u8);
        } else {
            self.canvas.set_draw_color(Color::RGB(0, 0, 0));
            self.canvas.clear();
        }

        // Draw canvas
        self.canvas.present();
    }

}

