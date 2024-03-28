const VRAM_START: usize = 0x8000;
const VRAM_END: usize = 0x9FFF;
const VRAM_SIZE: usize = 8 * 1024;

const WRAM_SW_START: usize = 0xC000 + 4 * 1024;
const WRAM_SW_END: usize = 0xDFFF;
const WRAM_SW_SIZE: usize = 4 * 1024;

const VBK_ADDR: usize = 0xFF4F;
const SVBK_ADDR: usize = 0xFF70;

pub struct Memory {
    memory: [u8; 0x10000],

    vram_extra_bank: [u8; VRAM_SIZE],
    wram_extra_banks: [[u8; WRAM_SW_SIZE]; 7]
}

impl Memory {

    pub fn new() -> Self {
        Self {
            memory: [0; 0x10000],
            vram_extra_bank: [0; VRAM_SIZE],
            wram_extra_banks: [[0; WRAM_SW_SIZE]; 7]
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        let addr = addr as usize;

        // Echo RAM
        if addr >= 0xE000 && addr <= 0xFDFF {
            return self.memory[addr - 0x2000];
        }

        // Not usable RAM
        if addr >= 0xFEA0 && addr <= 0xFEFF {
            let nibble = (addr & 0x00F0) as u8;
            return nibble | (nibble >> 8);
        }

        // VRAM bank selection
        if addr == VBK_ADDR {
            return 0xFE | (self.memory[VBK_ADDR] & 0x01);
        }

        // Normal behavior
        self.memory[addr]
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        let addr = addr as usize;

        // Echo RAM
        if addr >= 0xE000 && addr <= 0xFDFF {
            self.memory[addr - 0x2000] = value;
            return;
        }

        // VRAM bank selection
        if addr == VBK_ADDR {
            // Switch VRAM banks if requested
            if (self.memory[VBK_ADDR] & 0x01) != (value & 0x01) {
                for i in 0..VRAM_SIZE {
                    let aux = self.memory[VRAM_START + i];
                    self.memory[VRAM_START + i] = self.vram_extra_bank[i];
                    self.vram_extra_bank[i] = aux;
                }
            }
        }

        // WRAM bank selection
        if addr == SVBK_ADDR {
            // Switch WRAM banks
            let mut old_bank = self.memory[SVBK_ADDR] & 0x07;
            old_bank = if old_bank != 0 { old_bank - 1 } else { 0 };

            let mut new_bank = value & 0x07;
            new_bank = if old_bank != 0 { new_bank - 1 } else { 0 };

            for i in 0..WRAM_SW_SIZE {
                self.wram_extra_banks[old_bank as usize][i] = self.memory[WRAM_SW_START + i];
                self.memory[WRAM_SW_START + i] = self.wram_extra_banks[new_bank as usize][i];
            }
        }

        self.memory[addr] = value;
    }

}

