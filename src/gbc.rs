mod core;
mod memory;

use core::Core;
use memory::Memory;

pub struct GameBoyColor {
    core: Core,
    memory: Memory
}

impl GameBoyColor {

    pub fn new() -> Self {
        Self {
            core: Core::new(),
            memory: Memory::new()
        }
    }

    pub fn run(&mut self) {
        self.core.run_step(&mut self.memory);
    }

}

