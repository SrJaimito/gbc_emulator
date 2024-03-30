extern crate sdl2;

use sdl2::Sdl;
use sdl2::EventPump;
use sdl2::event::Event;

mod core;
mod memory;
mod display;

use core::Core;
use memory::Memory;
use display::Display;

pub struct GameBoyColor {
    sdl_context: Sdl,
    sdl_event_pump: EventPump,

    core: Core,
    memory: Memory,
    display: Display
}

impl GameBoyColor {

    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let sdl_event_pump = sdl_context.event_pump().unwrap();

        let display = Display::new(&sdl_context);

        Self {
            sdl_context,
            sdl_event_pump,
            core: Core::new(),
            memory: Memory::new(),
            display
        }
    }

    pub fn run(&mut self) {
        'main_loop: loop {
            // Check input events
            for event in self.sdl_event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'main_loop,
                    _ => {}
                }
            }

            // Next CPU instruction
            // self.core.run_step(&mut self.memory);

            // Update display
            self.display.update();
        }
    }

}

