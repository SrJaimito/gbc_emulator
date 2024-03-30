use super::Core;

use super::Memory;

// Vector table
const INT_VBLANK_ADDR: u16 = 0x0040;
const INT_LCD_ADDR: u16 = 0x0048;
const INT_TIMER_ADDR: u16 = 0x0050;
const INT_SERIAL_ADDR: u16 = 0x0058;
const INT_JOYPAD_ADDR: u16 = 0x0060;

// Interrupt types
#[derive(Copy, Clone)]
pub enum Interrupt {
    VBlank,
    Lcd,
    Timer,
    Serial,
    Joypad
}

impl Core {

    pub fn attend_interrupt(&mut self, interrupt: Interrupt, memory: &mut Memory) -> bool {
        if self.ime_enabled {
            self.ime_enabled = false;
            self.ime_enable_request = 0;

            let pc_lsb = self.pc & 0x00FF;
            let pc_msb = self.pc >> 8;

            memory.write(self.sp - 1, pc_msb as u8);
            memory.write(self.sp - 2, pc_lsb as u8);
            self.sp -= 2;

            self.pc = match interrupt {
                Interrupt::VBlank => INT_VBLANK_ADDR,
                Interrupt::Lcd => INT_LCD_ADDR,
                Interrupt::Timer => INT_TIMER_ADDR,
                Interrupt::Serial => INT_SERIAL_ADDR,
                Interrupt::Joypad => INT_JOYPAD_ADDR
            };

            true
        } else {
            false
        }
    }

}

