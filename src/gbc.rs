use std::time::{Instant};

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

// Clock periods (ns)
const SLOW_CLK_PERIOD: u128 = 238;
const FAST_CLK_PERIOD: u128 = 119;

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
        let mut fast_clock_timer = Instant::now();
        let mut waiting_cpu_cycles = 0u8;
        let mut waiting_display_cycles = 0u8;

        'main_loop: loop {
            // Check input events
            for event in self.sdl_event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'main_loop,
                    _ => {}
                }
            }

            // Fast clock cycle timing
            if fast_clock_timer.elapsed().as_nanos() >= FAST_CLK_PERIOD {
                fast_clock_timer = Instant::now();

                if waiting_cpu_cycles == 0 {
                    // Should we move to an interrupt?
                    let attending_interrupt = if let Some(interrupt) = self.memory.next_pending_interrupt() {
                        self.core.attend_interrupt(interrupt, &mut self.memory)
                    } else {
                        false
                    };

                    waiting_cpu_cycles = if attending_interrupt {
                        20
                    } else {
                        self.core.run_step(&mut self.memory) * 4
                    };
                }

                // Update display each two fast cock cycles
                if waiting_display_cycles == 0 {
                    self.display.update(&self.memory);

                    waiting_display_cycles = 2;
                }

                waiting_cpu_cycles -= 1;
                waiting_display_cycles -= 1;
            }
        }
    }

}

