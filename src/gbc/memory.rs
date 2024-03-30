use super::core::interrupt::Interrupt;

// Memory map

const MEMORY_START: usize = 0x0000;
const MEMORY_END: usize = 0xFFFF;
const MEMORY_SIZE: usize = MEMORY_END - MEMORY_START + 1;

const CARTRIDGE_START: usize = MEMORY_START;
const CARTRIDGE_END: usize = 0x7FFF;
const CARTRIDGE_SIZE: usize = CARTRIDGE_END - CARTRIDGE_START + 1;

const VRAM_START: usize = 0x8000;
const VRAM_END: usize = 0x9FFF;
const VRAM_SIZE: usize = VRAM_END - VRAM_START + 1;

const VRAM_CHAR_DATA_START: usize = VRAM_START;
const VRAM_CHAR_DATA_END: usize = 0x97FF;
const VRAM_CHAR_DATA_SIZE: usize = VRAM_CHAR_DATA_END - VRAM_CHAR_DATA_START + 1;

const VRAM_BG_DATA_1_START: usize = VRAM_CHAR_DATA_END + 1;
const VRAM_BG_DATA_1_END: usize = 0x9BFF;
const VRAM_BG_DATA_1_SIZE: usize = VRAM_BG_DATA_1_END - VRAM_BG_DATA_1_START + 1;

const VRAM_BG_DATA_2_START: usize = VRAM_BG_DATA_1_END + 1;
const VRAM_BG_DATA_2_END: usize = VRAM_END;
const VRAM_BG_DATA_2_SIZE: usize = VRAM_BG_DATA_2_END - VRAM_BG_DATA_2_START + 1;

const EXT_WRAM_START: usize = 0xA000;
const EXT_WRAM_END: usize = 0xBFFF;
const EXT_WRAM_SIZE: usize = EXT_WRAM_END - EXT_WRAM_START + 1;

const WRAM_START: usize = 0xC000;
const WRAM_END: usize = 0xDFFF;
const WRAM_SIZE: usize = WRAM_END - WRAM_START + 1;

const FIXED_WRAM_START: usize = WRAM_START;
const FIXED_WRAM_END: usize = 0xCFFF;
const FIXED_WRAM_SIZE: usize = FIXED_WRAM_END - FIXED_WRAM_START + 1;

const SW_WRAM_START: usize = FIXED_WRAM_END + 1;
const SW_WRAM_END: usize = WRAM_END;
const SW_WRAM_SIZE: usize = SW_WRAM_END - SW_WRAM_START + 1;

const ECHO_WRAM_START: usize = 0xE000;
const ECHO_WRAM_END: usize = 0xFDFF;
const ECHO_WRAM_SIZE: usize = ECHO_WRAM_END - ECHO_WRAM_START + 1;

const OAM_START: usize = 0xFE00;
const OAM_END: usize = 0xFEBF;
const OAM_SIZE: usize = OAM_END - OAM_START + 1;

const NOT_USABLE_START: usize = 0xFEA0;
const NOT_USABLE_END: usize = 0xFEFF;
const NOT_USABLE_SIZE: usize = NOT_USABLE_END - NOT_USABLE_START + 1;

const OTHER_START: usize = 0xFF00;
const OTHER_END: usize = MEMORY_END;
const OTHER_SIZE: usize = OTHER_END - OTHER_START + 1;

// Memory mapped registers

const IF_ADDR: usize = 0xFF0F;
const VBK_ADDR: usize = 0xFF4F;
const SVBK_ADDR: usize = 0xFF70;
const IE_ADDR: usize = 0xFFFF;

pub struct Memory {
    fixed_memory: [u8; MEMORY_SIZE],

    vram_banks: [[u8; VRAM_SIZE]; 2],
    active_vram_bank: usize,

    sw_wram_banks: [[u8; SW_WRAM_SIZE]; 7],
    active_sw_wram_bank: usize
}

impl Memory {

    pub fn new() -> Self {
        Self {
            fixed_memory: [0; MEMORY_SIZE],

            vram_banks: [[0; VRAM_SIZE]; 2],
            active_vram_bank: 0,

            sw_wram_banks: [[0; SW_WRAM_SIZE]; 7],
            active_sw_wram_bank: 0
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        let addr = addr as usize;

        // VRAM
        if addr >= VRAM_START && addr <= VRAM_END {
            return self.vram_banks[self.active_vram_bank][addr - VRAM_START];
        }

        // Switchable WRAM
        if addr >= SW_WRAM_START && addr <= SW_WRAM_END {
            return self.sw_wram_banks[self.active_sw_wram_bank][addr - SW_WRAM_START];
        }

        // Echo RAM
        if addr >= ECHO_WRAM_START && addr <= ECHO_WRAM_END {
            return self.read((addr - (ECHO_WRAM_START - WRAM_START)) as u16);
        }

        // Not usable
        if addr >= NOT_USABLE_START && addr <= NOT_USABLE_END {
            let nibble = (addr & 0x00F0) as u8;
            return nibble | (nibble >> 4);
        }

        // Memory mapped registers
        if addr >= OTHER_START {
            // VRAM bank selection
            if addr == VBK_ADDR {
                return 0xFE | (self.fixed_memory[VBK_ADDR] & 0x01);
            }
        }

        // Normal behavior
        self.fixed_memory[addr]
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        let addr = addr as usize;

        // VRAM
        if addr >= VRAM_START && addr <= VRAM_END {
            self.vram_banks[self.active_vram_bank][addr - VRAM_START] = value;
        }

        // Switchable WRAM
        if addr >= SW_WRAM_START && addr <= SW_WRAM_END {
            self.sw_wram_banks[self.active_sw_wram_bank][addr - SW_WRAM_START] = value;
        }

        // Echo RAM
        if addr >= ECHO_WRAM_START && addr <= ECHO_WRAM_END {
            return self.write((addr - (ECHO_WRAM_START - WRAM_START)) as u16, value);
        }

        if addr >= OTHER_START {
            // VRAM bank selection
            if addr == VBK_ADDR {
                self.active_vram_bank = (value & 0x01) as usize;
            }

            // WRAM bank selection
            if addr == SVBK_ADDR {
                let selected_bank = (value & 0x07) as usize;

                self.active_sw_wram_bank = if selected_bank != 0 {
                    selected_bank - 1
                } else {
                    0
                };
            }
        }

        self.fixed_memory[addr] = value;
    }

    pub fn next_pending_interrupt(&self) -> Option<Interrupt> {
        let pending_interrupts = self.fixed_memory[IF_ADDR];
        let enabled_interrupts = self.fixed_memory[IE_ADDR];

        for i in 0..=4 {
            let pending_mask = pending_interrupts & (0x01 << i);
            let enabled_mask = enabled_interrupts & (0x01 << i);
             
            if pending_mask != 0 && enabled_mask != 0 {
                match i {
                    0 => {
                        return Some(Interrupt::VBlank);
                    },
                    1 => {
                        return Some(Interrupt::Lcd);
                    },
                    2 => {
                        return Some(Interrupt::Timer);
                    },
                    3 => {
                        return Some(Interrupt::Serial);
                    },
                    4 => {
                        return Some(Interrupt::Joypad);
                    },
                    _ => {}
                }
            }
        }

        None
    }

    pub fn notify_interrupt(&mut self, interrupt: Interrupt) {
        let bit: u8 = match interrupt {
            Interrupt::VBlank => 0,
            Interrupt::Lcd => 1,
            Interrupt::Timer => 2,
            Interrupt::Serial => 3,
            Interrupt::Joypad => 4
        };

        self.fixed_memory[IF_ADDR] |= 0x01 << bit;
    }

}

