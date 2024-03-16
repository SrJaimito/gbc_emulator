#[derive(Copy, Clone)]
pub enum Reg8 {
    A, F,
    B, C,
    D, E,
    H, L
}

#[derive(Copy, Clone)]
pub enum Reg16 {
    AF,
    BC,
    DE,
    HL
}

#[derive(Copy, Clone)]
pub enum Flag {
    Z, N, H, CY
}

pub struct RegisterFile {
    a: u8, f: u8,
    b: u8, c: u8,
    d: u8, e: u8,
    h: u8, l: u8
}

impl RegisterFile {

    pub fn new() -> Self {
        Self {
            a: 0, f: 0,
            b: 0, c: 0,
            d: 0, e: 0,
            h: 0, l: 0
        }
    }

    pub fn read(&self, reg: Reg8) -> u8 {
        match reg {
            Reg8::A => self.a,
            Reg8::F => self.f,
            Reg8::B => self.b,
            Reg8::C => self.c,
            Reg8::D => self.d,
            Reg8::E => self.e,
            Reg8::H => self.h,
            Reg8::L => self.l
        }
    }

    pub fn write(&mut self, reg: Reg8, value: u8) {
        match reg {
            Reg8::A => self.a = value,
            Reg8::F => self.f = value,
            Reg8::B => self.b = value,
            Reg8::C => self.c = value,
            Reg8::D => self.d = value,
            Reg8::E => self.e = value,
            Reg8::H => self.h = value,
            Reg8::L => self.l = value
        }
    }

    pub fn dread(&self, reg: Reg16) -> u16 {
        match reg {
            Reg16::AF => {
                ((self.a as u16) << 8) | (self.f as u16)
            },

            Reg16::BC => {
                ((self.b as u16) << 8) | (self.c as u16)
            },

            Reg16::DE => {
                ((self.d as u16) << 8) | (self.e as u16)
            },

            Reg16::HL => {
                ((self.h as u16) << 8) | (self.l as u16)
            }
        }
    }

    pub fn dwrite(&mut self, reg: Reg16, value: u16) {
        let lsb = (value & 0xFF) as u8;
        let msb= ((value >> 8) & 0xFF) as u8;

        match reg {
            Reg16::AF => {
                self.a = msb;
                self.f = lsb;
            },

            Reg16::BC => {
                self.b = msb;
                self.c = lsb;
            },
            
            Reg16::DE => {
                self.d = msb;
                self.e = lsb;
            },

            Reg16::HL => {
                self.h = msb;
                self.l = lsb;
            }
        }
    }

    pub fn read_flag(&mut self, flag: Flag) -> bool {
        let offset = match flag {
            Flag::Z => 7,
            Flag::N => 6,
            Flag::H => 5,
            Flag::CY => 4
        };

        if ((self.f >> offset) & 0x01) == 0x01 {
            true
        } else {
            false
        }
    }

    pub fn write_flag(&mut self, flag: Flag, value: bool) {
        let offset = match flag {
            Flag::Z => 7,
            Flag::N => 6,
            Flag::H => 5,
            Flag::CY => 4
        };

        let mask = 0x01 << offset;

        if value {
            self.f |= mask;
        } else {
            self.f &= !mask;
        }
    }

}

