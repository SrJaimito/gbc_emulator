mod register_file;
mod instructions;
mod memory;

use register_file::RegisterFile;
use memory::Memory;

struct Core {
    reg: RegisterFile,

    pc: u16,
    sp: u16,

    mem: Memory
}

impl Core {

    pub fn new() -> Self {
        Self {
            reg: RegisterFile::new(),
            pc: 0,
            sp: 0
        }
    }

    pub fn execute(&self, opcode: u8) {
        unimplemented!();
    }

}

