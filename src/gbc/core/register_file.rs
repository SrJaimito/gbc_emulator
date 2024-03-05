pub enum SingleReg {
    A, F,
    B, C,
    D, E,
    H, L
}

pub enum DoubleReg {
    AF,
    BC,
    DE,
    HL
}

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

    pub fn get_single_reg(&self, reg: SingleReg) -> u8 {
        match reg {
            SingleReg::A => self.a,
            SingleReg::F => self.f,
            SingleReg::B => self.b,
            SingleReg::C => self.c,
            SingleReg::D => self.d,
            SingleReg::E => self.e,
            SingleReg::H => self.h,
            SingleReg::L => self.l
        }
    }

    pub fn set_single_reg(&mut self, reg: SingleReg, value: u8) {
        match reg {
            SingleReg::A => self.a = value,
            SingleReg::F => self.f = value,
            SingleReg::B => self.b = value,
            SingleReg::C => self.c = value,
            SingleReg::D => self.d = value,
            SingleReg::E => self.e = value,
            SingleReg::H => self.h = value,
            SingleReg::L => self.l = value
        }
    }

    pub fn get_double_reg(&self, reg: DoubleReg) -> u16 {
        match reg {
            DoubleReg::AF => {
                ((self.a as u16) << 8) | (self.f as u16)
            },

            DoubleReg::BC => {
                ((self.b as u16) << 8) | (self.c as u16)
            },

            DoubleReg::DE => {
                ((self.d as u16) << 8) | (self.e as u16)
            },

            DoubleReg::HL => {
                ((self.h as u16) << 8) | (self.l as u16)
            }
        }
    }

    pub fn set_double_reg(&mut self, reg: DoubleReg, value: u16) {
        let lsb = (value & 0xFF) as u8;
        let msb= ((value >> 8) & 0xFF) as u8;

        match reg {
            DoubleReg::AF => {
                self.a = msb;
                self.f = lsb;
            },

            DoubleReg::BC => {
                self.b = msb;
                self.c = lsb;
            },
            
            DoubleReg::DE => {
                self.d = msb;
                self.e = lsb;
            },

            DoubleReg::HL => {
                self.h = msb;
                self.l = lsb;
            }
        }
    }

    pub fn set_flag(&mut self, flag: Flag, value: bool) {
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

    pub fn get_flag(&mut self, flag: Flag) -> bool {
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

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn get_set_single_registers() {
        let mut register_file = RegisterFile::new();

        register_file.a = 0x01;
        register_file.f = 0x02;
        register_file.b = 0x03;
        register_file.c = 0x04;
        register_file.d = 0x05;
        register_file.e = 0x06;
        register_file.h = 0x07;
        register_file.l = 0x08;

        assert_eq!(register_file.get_single_reg(SingleReg::A), 0x01);
        assert_eq!(register_file.get_single_reg(SingleReg::F), 0x02);
        assert_eq!(register_file.get_single_reg(SingleReg::B), 0x03);
        assert_eq!(register_file.get_single_reg(SingleReg::C), 0x04);
        assert_eq!(register_file.get_single_reg(SingleReg::D), 0x05);
        assert_eq!(register_file.get_single_reg(SingleReg::E), 0x06);
        assert_eq!(register_file.get_single_reg(SingleReg::H), 0x07);
        assert_eq!(register_file.get_single_reg(SingleReg::L), 0x08);

        register_file.set_single_reg(SingleReg::A, 0x09);
        register_file.set_single_reg(SingleReg::F, 0x0A);
        register_file.set_single_reg(SingleReg::B, 0x0B);
        register_file.set_single_reg(SingleReg::C, 0x0C);
        register_file.set_single_reg(SingleReg::D, 0x0D);
        register_file.set_single_reg(SingleReg::E, 0x0E);
        register_file.set_single_reg(SingleReg::H, 0x0F);
        register_file.set_single_reg(SingleReg::L, 0x10);

        assert_eq!(register_file.a, 0x09);
        assert_eq!(register_file.f, 0x0A);
        assert_eq!(register_file.b, 0x0B);
        assert_eq!(register_file.c, 0x0C);
        assert_eq!(register_file.d, 0x0D);
        assert_eq!(register_file.e, 0x0E);
        assert_eq!(register_file.h, 0x0F);
        assert_eq!(register_file.l, 0x10);
    }

    #[test]
    fn get_set_double_registers() {
        let mut register_file = RegisterFile::new();

        register_file.a = 0x01;
        register_file.f = 0x02;
        register_file.b = 0x03;
        register_file.c = 0x04;
        register_file.d = 0x05;
        register_file.e = 0x06;
        register_file.h = 0x07;
        register_file.l = 0x08;

        assert_eq!(register_file.get_double_reg(DoubleReg::AF), 0x0102);
        assert_eq!(register_file.get_double_reg(DoubleReg::BC), 0x0304);
        assert_eq!(register_file.get_double_reg(DoubleReg::DE), 0x0506);
        assert_eq!(register_file.get_double_reg(DoubleReg::HL), 0x0708);

        register_file.set_double_reg(DoubleReg::AF, 0x090A);
        register_file.set_double_reg(DoubleReg::BC, 0x0B0C);
        register_file.set_double_reg(DoubleReg::DE, 0x0D0E);
        register_file.set_double_reg(DoubleReg::HL, 0x0F10);

        assert_eq!(register_file.a, 0x09);
        assert_eq!(register_file.f, 0x0A);
        assert_eq!(register_file.b, 0x0B);
        assert_eq!(register_file.c, 0x0C);
        assert_eq!(register_file.d, 0x0D);
        assert_eq!(register_file.e, 0x0E);
        assert_eq!(register_file.h, 0x0F);
        assert_eq!(register_file.l, 0x10);
    }

    #[test]
    fn set_get_flag() {
        let mut register_file = RegisterFile::new();

        register_file.f = 0xA0;

        assert_eq!(register_file.get_flag(Flag::Z), true);
        assert_eq!(register_file.get_flag(Flag::N), false);
        assert_eq!(register_file.get_flag(Flag::H), true);
        assert_eq!(register_file.get_flag(Flag::CY), false);

        register_file.set_flag(Flag::Z, false);
        register_file.set_flag(Flag::N, true);
        register_file.set_flag(Flag::H, false);
        register_file.set_flag(Flag::CY, true);

        assert_eq!(register_file.f, 0x50);
    }

}

