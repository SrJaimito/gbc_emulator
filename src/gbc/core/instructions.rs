use super::Core;
use super::register_file::{SingleReg, DoubleReg, map_3bit_field, Flag};

pub struct InstructionInfo(u8, u8);

impl Core {

    ///////////////////////////////////
    //                               //
    //  8-Bit Transfer Instructions  //
    //                               //
    ///////////////////////////////////

    fn ld_r8_r8(&mut self, opcode: u8) -> InstructionInfo {
        let src_reg = match map_3bit_field(opcode & 0x07) {
            Some(reg) => reg,
            None => panic!("Error src_reg ld_r8_r8")
        };

        let dst_reg = match map_3bit_field((opcode >> 3) & 0x07) {
            Some(reg) => reg,
            None => panic!("Error dst_reg ld_r8_r8")
        };

        let value = self.reg.read_reg(src_reg);

        self.reg.write_reg(dst_reg, value);

        InstructionInfo(1, 4)
    }

    fn ld_r8_n8(&mut self, opcode: u8) -> InstructionInfo {
        let dst_reg = match map_3bit_field((opcode >> 3) & 0x07) {
            Some(reg) => reg,
            None => panic!("Error dst_reg ld_r8_n8")
        };

        let value = self.mem.read(self.pc + 1);

        self.reg.write_reg(dst_reg, value);

        InstructionInfo(2, 8)
    }

    fn ld_r8_hl(&mut self, opcode: u8) -> InstructionInfo {
        let dst_reg = match map_3bit_field((opcode >> 3) & 0x07) {
            Some(reg) => reg,
            None => panic!("Error dst_reg ld_r8_hl")
        };

        let addr = self.reg.read_dreg(DoubleReg::HL);
        let value = self.mem.read(addr);

        self.reg.write_reg(dst_reg, value);

        InstructionInfo(1, 8)
    }

    fn ld_hl_r8(&mut self, opcode: u8) -> InstructionInfo {
        let src_reg = match map_3bit_field((opcode >> 3) & 0x07) {
            Some(reg) => reg,
            None => panic!("Error src_reg ld_hl_r8")
        };

        let value = self.reg.read_reg(src_reg);
        let addr = self.reg.read_dreg(DoubleReg::HL);

        self.mem.write(addr, value);

        InstructionInfo(1, 8)
    }

    fn ld_hl_n8(&mut self) -> InstructionInfo {
        let value = self.mem.read(self.pc + 1);
        let addr = self.reg.read_dreg(DoubleReg::HL);

        self.mem.write(addr, value);

        InstructionInfo(2, 12)
    }

    fn ld_a_bc(&mut self) -> InstructionInfo {
        let addr = self.reg.read_dreg(DoubleReg::BC);
        let value = self.mem.read(addr);

        self.reg.write_reg(SingleReg::A, value);

        InstructionInfo(1, 8)
    }

    fn ld_a_de(&mut self) -> InstructionInfo {
        let addr = self.reg.read_dreg(DoubleReg::DE);
        let value = self.mem.read(addr);

        self.reg.write_reg(SingleReg::A, value);

        InstructionInfo(1, 8)
    }

    fn ld_a_c(&mut self) -> InstructionInfo {
        let offset = self.reg.read_reg(SingleReg::C);
        let addr = 0xFF00 + (offset as u16);

        let value = self.mem.read(addr);
        self.reg.write_reg(SingleReg::A, value);

        InstructionInfo(1, 8)
    }

    fn ld_c_a(&mut self) ->  InstructionInfo {
        let offset = self.reg.read_reg(SingleReg::C);
        let addr = 0xFF00 + (offset as u16);

        let value = self.reg.read_reg(SingleReg::A);
        self.mem.write(addr, value);

        InstructionInfo(1, 8)
    }

    fn ld_a_n(&mut self) -> InstructionInfo {
        let offset = self.mem.read(self.pc + 1);
        let addr = 0xFF00 + (offset as u16);

        let value = self.mem.read(addr);
        self.reg.write_reg(SingleReg::A, value);

        InstructionInfo(2, 12)
    }

    fn ld_n_a(&mut self) ->  InstructionInfo {
        let offset = self.mem.read(self.pc + 1);
        let addr = 0xFF00 + (offset as u16);

        let value = self.reg.read_reg(SingleReg::A);
        self.mem.write(addr, value);

        InstructionInfo(2, 12)
    }

    fn ld_a_nn(&mut self) -> InstructionInfo {
        let lsb = self.mem.read(self.pc + 1) as u16;
        let msb = self.mem.read(self.pc + 2) as u16;

        let addr = (msb << 8) | lsb;

        let value = self.mem.read(addr);
        self.reg.write_reg(SingleReg::A, value);

        InstructionInfo(3, 16)
    }

    fn ld_nn_a(&mut self) -> InstructionInfo {
        let lsb = self.mem.read(self.pc + 1) as u16;
        let msb = self.mem.read(self.pc + 2) as u16;

        let addr = (msb << 8) | lsb;

        let value = self.reg.read_reg(SingleReg::A);
        self.mem.write(addr, value);

        InstructionInfo(3, 16)
    }

    fn ld_a_hli(&mut self) -> InstructionInfo {
        let addr = self.reg.read_dreg(DoubleReg::HL);
        let value = self.mem.read(addr);

        self.reg.write_reg(SingleReg::A, value);
        self.reg.write_dreg(DoubleReg::HL, addr + 1);

        InstructionInfo(1, 8)
    }

    fn ld_a_hld(&mut self) -> InstructionInfo {
        let addr = self.reg.read_dreg(DoubleReg::HL);
        let value = self.mem.read(addr);

        self.reg.write_reg(SingleReg::A, value);
        self.reg.write_dreg(DoubleReg::HL, addr - 1);

        InstructionInfo(1, 8)
    }

    fn ld_bc_a(&mut self) -> InstructionInfo {
        let addr = self.reg.read_dreg(DoubleReg::BC);
        let value = self.reg.read_reg(SingleReg::A);

        self.mem.write(addr, value);

        InstructionInfo(1, 8)
    }

    fn ld_de_a(&mut self) -> InstructionInfo {
        let addr = self.reg.read_dreg(DoubleReg::DE);
        let value = self.reg.read_reg(SingleReg::A);

        self.mem.write(addr, value);

        InstructionInfo(1, 8)
    }

    fn ld_hli_a(&mut self) -> InstructionInfo {
        let addr = self.reg.read_dreg(DoubleReg::HL);
        let value = self.reg.read_reg(SingleReg::A);

        self.mem.write(addr, value);
        self.reg.write_dreg(DoubleReg::HL, addr + 1);

        InstructionInfo(1, 8)
    }

    fn ld_hld_a(&mut self) -> InstructionInfo {
        let addr = self.reg.read_dreg(DoubleReg::HL);
        let value = self.reg.read_reg(SingleReg::A);

        self.mem.write(addr, value);
        self.reg.write_dreg(DoubleReg::HL, addr - 1);

        InstructionInfo(1, 8)
    }

    ////////////////////////////////////
    //                                //
    //  16-Bit Transfer Instructions  //
    //                                //
    ////////////////////////////////////

    fn ld_r16_n16(&mut self, opcode: u8) -> InstructionInfo {
        let lsb = self.mem.read(self.pc + 1) as u16;
        let msb = self.mem.read(self.pc + 2) as u16;

        let value = (msb << 8) | lsb;

        let masked_opcode = (opcode >> 4) & 0x03;

        if masked_opcode == 0x03 {
            self.sp = value;
        } else {
            let dst_reg = match masked_opcode {
                0x00 => DoubleReg::BC,
                0x01 => DoubleReg::DE,
                0x02 => DoubleReg::HL,
                _ => panic!("Error dst_reg ld_r16_n16")
            };

            self.reg.write_dreg(dst_reg, value);
        }

        InstructionInfo(3, 12)
    }

    fn ld_sp_hl(&mut self) -> InstructionInfo {
        let value = self.reg.read_dreg(DoubleReg::HL);
        self.sp = value;

        InstructionInfo(1, 8)
    }

    fn push_qq(&mut self, opcode: u8) -> InstructionInfo {
        match (opcode >> 4) & 0x03 {
            0x00 => {
                self.mem.write(self.sp - 1, self.reg.read_reg(SingleReg::B));
                self.mem.write(self.sp - 2, self.reg.read_reg(SingleReg::C));
            },
            0x01 => {
                self.mem.write(self.sp - 1, self.reg.read_reg(SingleReg::D));
                self.mem.write(self.sp - 2, self.reg.read_reg(SingleReg::E));
            },
            0x02 => {
                self.mem.write(self.sp - 1, self.reg.read_reg(SingleReg::H));
                self.mem.write(self.sp - 2, self.reg.read_reg(SingleReg::L));
            },
            0x03 => {
                self.mem.write(self.sp - 1, self.reg.read_reg(SingleReg::A));
                self.mem.write(self.sp - 2, self.reg.read_reg(SingleReg::F));
            },
            _ => panic!("Error src_reg push_qq")
        }

        self.sp -= 2;

        InstructionInfo(1, 16)
    }

    fn pop_qq(&mut self, opcode: u8) -> InstructionInfo {
        match (opcode >> 4) & 0x03 {
            0x00 => {
                self.mem.write(self.sp, self.reg.read_reg(SingleReg::C));
                self.mem.write(self.sp + 1, self.reg.read_reg(SingleReg::B));
            },
            0x01 => {
                self.mem.write(self.sp, self.reg.read_reg(SingleReg::E));
                self.mem.write(self.sp + 1, self.reg.read_reg(SingleReg::D));
            },
            0x02 => {
                self.mem.write(self.sp, self.reg.read_reg(SingleReg::L));
                self.mem.write(self.sp + 1, self.reg.read_reg(SingleReg::H));
            },
            0x03 => {
                self.mem.write(self.sp, self.reg.read_reg(SingleReg::F));
                self.mem.write(self.sp + 1, self.reg.read_reg(SingleReg::A));
            },
            _ => panic!("Error src_reg push_qq")
        }

        self.sp += 2;

        InstructionInfo(1, 12)
    }

    fn ldhl_sp_e(&mut self) -> InstructionInfo {
        let sp = self.sp as i32;
        let e = (self.mem.read(self.pc + 1) as i8) as i32);
        
        let result = sp + e;
        let carry = sp ^ e ^ result;

        let h = if (carry & 0x0000_1000) != 0 {
            true
        } else {
            false
        };

        let cy = if (carry & 0x0001_0000) != 0 {
            true
        } else {
            false
        };

        self.reg.write_dreg(DoubleReg::HL, result as u16);

        self.reg.set_flag(Flag::Z, false);
        self.reg.set_flag(Flag::N, false);
        self.reg.set_flag(Flag::H, h);
        self.reg.set_flag(Flag::CY, cy);

        InstructionInfo(2, 12)
    }

    fn ld_nn_sp(&mut self) -> InstructionInfo {
        let addr_lsb = self.mem.read(self.pc + 1) as u16;
        let addr_msb = self.mem.read(self.pc + 2) as u16;

        let addr = (addr_msb << 8) | addr_lsb;

        let sp_lsb = self.sp & 0x0F;
        let sp_msb = (self.sp >> 8) & 0x0F;

        self.mem.write(addr, sp_lsb as u8);
        self.mem.write(addr + 1, sp_msb as u8);

        InstructionInfo(3, 20)
    }

}

