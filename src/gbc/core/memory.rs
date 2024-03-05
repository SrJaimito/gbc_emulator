pub struct Memory {
    memory: [u8; 0x10000]
}

impl Memory {

    pub fn new() -> Self {
        Self {
            memory: [0; 0x10000]
        }
    }

    pub fn get_at(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    pub fn set_at(&self, addr: u16, value: u8) {
        self.memory[addr as usize] = value;
    }

}

