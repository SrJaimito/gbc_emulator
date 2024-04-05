use std::time::{Instant, Duration};

extern crate sdl2;

use sdl2::Sdl;
use sdl2::EventPump;
use sdl2::event::Event;

mod core;
mod memory;
mod display;

use core::{Core, SLOW_CLK_PERIOD};
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

    pub fn load_rom(&mut self, rom: Vec<u8>) {
        for (addr, byte) in rom.into_iter().enumerate() {
            self.memory.write(addr as u16, byte);
        }
    }

    pub fn run(&mut self) {
        let mut cpu_last_time = Instant::now();
        let mut waiting_cpu_time = 0u128;

        let mut display_last_time = Instant::now();

        'main_loop: loop {
            // Check input events
            for event in self.sdl_event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'main_loop,
                    _ => {}
                }
            }

            // Did we finish executing last instruction?
            if cpu_last_time.elapsed().as_nanos() >= waiting_cpu_time {
                cpu_last_time = Instant::now();

                // Should we move to an interrupt?
                let attending_interrupt = if let Some(interrupt) = self.memory.next_pending_interrupt() {
                    self.core.attend_interrupt(interrupt, &mut self.memory)
                } else {
                    false
                };

                let waiting_cpu_cycles = if attending_interrupt {
                    20
                } else {
                    self.core.run_step(&mut self.memory) * 4
                };

                waiting_cpu_time = (waiting_cpu_cycles as u128) * self.core.current_clk_period();
            }

            // Update display (assuming too much about time synchronization??? Review this timing)
            if display_last_time.elapsed().as_nanos() >= SLOW_CLK_PERIOD {
                display_last_time = Instant::now();

                self.display.update(&self.memory);
            }
        }
    }

}

