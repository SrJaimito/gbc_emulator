pub struct Memory {
    memory: [u8; 0x10000]
}

impl Memory {

    pub fn new() -> Self {
        Self {
            memory: [0; 0x10000]
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        if addr >= 0xE000 && addr <= 0xFDFF {
            // Echo RAM
            self.memory[(addr - 0x2000) as usize]

        } else if addr >= 0xFEA0 && addr <= 0xFEFF {
            // Not usable RAM
            let nibble = (addr & 0x00F0) as u8;
            nibble | (nibble >> 8)

        } else {
            // Normal behavior
            self.memory[addr as usize]
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        if addr >= 0xE000 && addr <= 0xFDFF {
            // Echo RAM
            self.memory[(addr - 0x2000) as usize] = value;

        } else {
            // Normal behavior
            self.memory[addr as usize] = value;
        }
    }

}

