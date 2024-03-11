use super::Core;
use super::register_file::{SingleReg, DoubleReg, map_3bit_field, Flag};

pub struct InstructionInfo(u8, u8);

impl Core {

    fn prefix(&self) -> InstructionInfo {
        InstructionInfo(1, 4)
    }

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
            _ => panic!("Error src_reg pop_qq")
        }

        self.sp += 2;

        InstructionInfo(1, 12)
    }

    fn ldhl_sp_e(&mut self) -> InstructionInfo {
        let sp = self.sp as i32;
        let e = (self.mem.read(self.pc + 1) as i8) as i32;
        
        let result = sp + e;
        let h = (((sp & 0x0FFF) + (e & 0x0FFF)) & 0x1000) == 0x1000;
        let cy = (((sp & 0xFFFF) + (e & 0xFFFF)) & 0x10000) == 0x10000;

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

    ///////////////////////////////////////////////////////////
    //                                                       //
    //  8-Bit Arithmetic and Logical Operation Instructions  //
    //                                                       //
    ///////////////////////////////////////////////////////////

    fn add_a_r8(&mut self, opcode: u8) -> InstructionInfo {
        let src_reg = match map_3bit_field(opcode & 0x07) {
            Some(reg) => reg,
            None => panic!("Error src_reg add_a_r8")
        };

        let r8 = self.reg.read_reg(src_reg);
        let a = self.reg.read_reg(SingleReg::A);

        let (result, cy) = a.overflowing_add(r8);
        
        self.reg.write_reg(SingleReg::A, result);

        let z = result == 0;
        let h = (((a & 0x0F) + (r8 & 0x0F)) & 0x10) == 0x10;

        self.reg.set_flag(Flag::Z, z);
        self.reg.set_flag(Flag::N, false);
        self.reg.set_flag(Flag::H, h);
        self.reg.set_flag(Flag::CY, cy);

        InstructionInfo(1, 4)
    }

    fn add_a_n8(&mut self) -> InstructionInfo {
        let n8 = self.mem.read(self.pc + 1);
        let a = self.reg.read_reg(SingleReg::A);

        let (result, cy) = a.overflowing_add(n8);

        self.reg.write_reg(SingleReg::A, result);

        let z = result == 0;
        let h = (((a & 0x0F) + (n8 & 0x0F)) & 0x10) == 0x10;

        self.reg.set_flag(Flag::Z, z);
        self.reg.set_flag(Flag::N, false);
        self.reg.set_flag(Flag::H, h);
        self.reg.set_flag(Flag::CY, cy);

        InstructionInfo(2, 8)
    }

    fn add_a_hl(&mut self) -> InstructionInfo {
        let addr = self.reg.read_dreg(DoubleReg::HL);
        let n8 = self.mem.read(addr);
        let a = self.reg.read_reg(SingleReg::A);

        let (result, cy) = a.overflowing_add(n8);

        self.reg.write_reg(SingleReg::A, result);

        let z = result == 0;
        let h = (((a & 0x0F) + (n8 & 0x0F)) & 0x10) == 0x10;

        self.reg.set_flag(Flag::Z, z);
        self.reg.set_flag(Flag::N, false);
        self.reg.set_flag(Flag::H, h);
        self.reg.set_flag(Flag::CY, cy);

        InstructionInfo(1, 8)
    }

    fn adc_a_r8(&mut self, opcode: u8) -> InstructionInfo {
        let src_reg = match map_3bit_field(opcode & 0x07) {
            Some(reg) => reg,
            None => panic!("Error src_reg adc_a_r8")
        };

        let a = self.reg.read_reg(SingleReg::A);
        let r8 = self.reg.read_reg(src_reg);
        let carry = if self.reg.get_flag(Flag::CY) { 1u8 } else { 0u8 };

        let (result, cy1) = a.overflowing_add(r8);
        let (result_carry, cy2) = result.overflowing_add(carry);

        self.reg.write_reg(SingleReg::A, result_carry);

        let z = result_carry == 0;
        let h = (((a & 0x0F) + (r8 & 0x0F) + carry) & 0x10) == 0x10;

        self.reg.set_flag(Flag::Z, z);
        self.reg.set_flag(Flag::N, false);
        self.reg.set_flag(Flag::H, h);
        self.reg.set_flag(Flag::CY, cy1 || cy2);

        InstructionInfo(1, 4)
    }

    fn adc_a_n8(&mut self) -> InstructionInfo {
        let a = self.reg.read_reg(SingleReg::A);
        let n8 = self.mem.read(self.pc + 1);
        let carry = if self.reg.get_flag(Flag::CY) { 1u8 } else { 0u8 };

        let (result, cy1) = a.overflowing_add(n8);
        let (result_carry, cy2) = result.overflowing_add(carry);

        self.reg.write_reg(SingleReg::A, result_carry);

        let z = result_carry == 0;
        let h = (((a & 0x0F) + (n8 & 0x0F) + carry) & 0x10) == 0x10;

        self.reg.set_flag(Flag::Z, z);
        self.reg.set_flag(Flag::N, false);
        self.reg.set_flag(Flag::H, h);
        self.reg.set_flag(Flag::CY, cy1 || cy2);

        InstructionInfo(2, 8)
    }

    fn adc_a_hl(&mut self) -> InstructionInfo {
        let addr = self.reg.read_dreg(DoubleReg::HL);
        let n8 = self.mem.read(addr);

        let a = self.reg.read_reg(SingleReg::A);
        let carry = if self.reg.get_flag(Flag::CY) { 1u8 } else { 0u8 };

        let (result, cy1) = a.overflowing_add(n8);
        let (result_carry, cy2) = result.overflowing_add(carry);

        self.reg.write_reg(SingleReg::A, result_carry);

        let z = result_carry == 0;
        let h = (((a & 0x0F) + (n8 & 0x0F) + carry) & 0x10) == 0x10;

        self.reg.set_flag(Flag::Z, z);
        self.reg.set_flag(Flag::N, false);
        self.reg.set_flag(Flag::H, h);
        self.reg.set_flag(Flag::CY, cy1 || cy2);

        InstructionInfo(1, 8)
    }

    fn sub_a_r8(&mut self, opcode: u8) -> InstructionInfo {
        let src_reg = match map_3bit_field(opcode & 0x07) {
            Some(reg) => reg,
            None => panic!("Error src_reg sub_a_r8")
        };

        let r8 = self.reg.read_reg(src_reg);
        let a = self.reg.read_reg(SingleReg::A);

        let (result, cy) = a.overflowing_sub(r8);
        
        self.reg.write_reg(SingleReg::A, result);

        let z = result == 0;

        let (h_sub, _) = (a & 0x0F).overflowing_sub(r8 & 0x0F);
        let h = (h_sub & 0x10) == 0x10;

        self.reg.set_flag(Flag::Z, z);
        self.reg.set_flag(Flag::N, true);
        self.reg.set_flag(Flag::H, h);
        self.reg.set_flag(Flag::CY, cy);

        InstructionInfo(1, 4)
    }

    fn sub_a_n8(&mut self) -> InstructionInfo {
        let n8 = self.mem.read(self.pc + 1);
        let a = self.reg.read_reg(SingleReg::A);

        let (result, cy) = a.overflowing_sub(n8);
        
        self.reg.write_reg(SingleReg::A, result);

        let z = result == 0;

        let (h_sub, _) = (a & 0x0F).overflowing_sub(n8 & 0x0F);
        let h = (h_sub & 0x10) == 0x10;

        self.reg.set_flag(Flag::Z, z);
        self.reg.set_flag(Flag::N, true);
        self.reg.set_flag(Flag::H, h);
        self.reg.set_flag(Flag::CY, cy);

        InstructionInfo(2, 8)
    }

    fn sub_a_hl(&mut self) -> InstructionInfo {
        let addr = self.reg.read_dreg(DoubleReg::HL);
        let n8 = self.mem.read(addr);
        let a = self.reg.read_reg(SingleReg::A);

        let (result, cy) = a.overflowing_sub(n8);
        
        self.reg.write_reg(SingleReg::A, result);

        let z = result == 0;

        let (h_sub, _) = (a & 0x0F).overflowing_sub(n8 & 0x0F);
        let h = (h_sub & 0x10) == 0x10;

        self.reg.set_flag(Flag::Z, z);
        self.reg.set_flag(Flag::N, true);
        self.reg.set_flag(Flag::H, h);
        self.reg.set_flag(Flag::CY, cy);

        InstructionInfo(1, 8)
    }

    fn sbc_a_r8(&mut self, opcode: u8) -> InstructionInfo {
        let src_reg = match map_3bit_field(opcode & 0x07) {
            Some(reg) => reg,
            None => panic!("Error src_reg sbc_a_r8")
        };

        let r8 = self.reg.read_reg(src_reg);
        let a = self.reg.read_reg(SingleReg::A);
        let carry = if self.reg.get_flag(Flag::CY) { 1u8 } else { 0u8 };

        let (result, cy1) = a.overflowing_sub(r8);
        let (result_carry, cy2) = result.overflowing_sub(carry);
        
        self.reg.write_reg(SingleReg::A, result_carry);

        let z = result_carry == 0;

        let (h_sub, _) = (a & 0x0F).overflowing_sub(r8 & 0x0F);
        let (h_sub_carry, _) = h_sub.overflowing_sub(carry);
        let h = (h_sub_carry & 0x10) == 0x10;

        self.reg.set_flag(Flag::Z, z);
        self.reg.set_flag(Flag::N, true);
        self.reg.set_flag(Flag::H, h);
        self.reg.set_flag(Flag::CY, cy1 || cy2);

        InstructionInfo(1, 4)
    }

    fn sbc_a_n8(&mut self) -> InstructionInfo {
        let n8 = self.mem.read(self.pc + 1);
        let a = self.reg.read_reg(SingleReg::A);
        let carry = if self.reg.get_flag(Flag::CY) { 1u8 } else { 0u8 };

        let (result, cy1) = a.overflowing_sub(n8);
        let (result_carry, cy2) = result.overflowing_sub(carry);
        
        self.reg.write_reg(SingleReg::A, result_carry);

        let z = result_carry == 0;

        let (h_sub, _) = (a & 0x0F).overflowing_sub(n8 & 0x0F);
        let (h_sub_carry, _) = h_sub.overflowing_sub(carry);
        let h = (h_sub_carry & 0x10) == 0x10;

        self.reg.set_flag(Flag::Z, z);
        self.reg.set_flag(Flag::N, true);
        self.reg.set_flag(Flag::H, h);
        self.reg.set_flag(Flag::CY, cy1 || cy2);

        InstructionInfo(2, 8)
    }

    fn sbc_a_hl(&mut self) -> InstructionInfo {
        let addr = self.reg.read_dreg(DoubleReg::HL);
        let n8 = self.mem.read(addr);
        let a = self.reg.read_reg(SingleReg::A);
        let carry = if self.reg.get_flag(Flag::CY) { 1u8 } else { 0u8 };

        let (result, cy1) = a.overflowing_sub(n8);
        let (result_carry, cy2) = result.overflowing_sub(carry);
        
        self.reg.write_reg(SingleReg::A, result_carry);

        let z = result_carry == 0;

        let (h_sub, _) = (a & 0x0F).overflowing_sub(n8 & 0x0F);
        let (h_sub_carry, _) = h_sub.overflowing_sub(carry);
        let h = (h_sub_carry & 0x10) == 0x10;

        self.reg.set_flag(Flag::Z, z);
        self.reg.set_flag(Flag::N, true);
        self.reg.set_flag(Flag::H, h);
        self.reg.set_flag(Flag::CY, cy1 || cy2);

        InstructionInfo(2, 8)
    }

    fn and_a_r8(&mut self, opcode: u8) -> InstructionInfo {
        let src_reg = match map_3bit_field(opcode & 0x07) {
            Some(reg) => reg,
            None => panic!("Error src_reg and_a_r8")
        };

        let a = self.reg.read_reg(SingleReg::A);
        let r8 = self.reg.read_reg(src_reg);

        let result = a & r8;

        let z = result == 0;

        self.reg.set_flag(Flag::Z, z);
        self.reg.set_flag(Flag::N, false);
        self.reg.set_flag(Flag::H, true);
        self.reg.set_flag(Flag::CY, false);

        InstructionInfo(1, 4)
    }

    fn and_a_n8(&mut self) -> InstructionInfo {
        let a = self.reg.read_reg(SingleReg::A);
        let n8 = self.mem.read(self.pc + 1);

        let result = a & n8;

        let z = result == 0;

        self.reg.set_flag(Flag::Z, z);
        self.reg.set_flag(Flag::N, false);
        self.reg.set_flag(Flag::H, true);
        self.reg.set_flag(Flag::CY, false);

        InstructionInfo(2, 8)
    }

    fn and_a_hl(&mut self) -> InstructionInfo {
        let addr = self.reg.read_dreg(DoubleReg::HL);
        let a = self.reg.read_reg(SingleReg::A);
        let n8 = self.mem.read(addr);

        let result = a & n8;

        let z = result == 0;

        self.reg.set_flag(Flag::Z, z);
        self.reg.set_flag(Flag::N, false);
        self.reg.set_flag(Flag::H, true);
        self.reg.set_flag(Flag::CY, false);

        InstructionInfo(1, 8)
    }

    fn or_a_r8(&mut self, opcode: u8) -> InstructionInfo {
        let src_reg = match map_3bit_field(opcode & 0x07) {
            Some(reg) => reg,
            None => panic!("Error src_reg and_a_r8")
        };

        let a = self.reg.read_reg(SingleReg::A);
        let r8 = self.reg.read_reg(src_reg);

        let result = a | r8;

        let z = result == 0;

        self.reg.set_flag(Flag::Z, z);
        self.reg.set_flag(Flag::N, false);
        self.reg.set_flag(Flag::H, false);
        self.reg.set_flag(Flag::CY, false);

        InstructionInfo(1, 4)
    }

    fn or_a_n8(&mut self) -> InstructionInfo {
        let a = self.reg.read_reg(SingleReg::A);
        let n8 = self.mem.read(self.pc + 1);

        let result = a | n8;

        let z = result == 0;

        self.reg.set_flag(Flag::Z, z);
        self.reg.set_flag(Flag::N, false);
        self.reg.set_flag(Flag::H, false);
        self.reg.set_flag(Flag::CY, false);

        InstructionInfo(2, 8)
    }

    fn or_a_hl(&mut self) -> InstructionInfo {
        let addr = self.reg.read_dreg(DoubleReg::HL);
        let a = self.reg.read_reg(SingleReg::A);
        let n8 = self.mem.read(addr);

        let result = a | n8;

        let z = result == 0;

        self.reg.set_flag(Flag::Z, z);
        self.reg.set_flag(Flag::N, false);
        self.reg.set_flag(Flag::H, false);
        self.reg.set_flag(Flag::CY, false);

        InstructionInfo(1, 8)
    }

    fn xor_a_r8(&mut self, opcode: u8) -> InstructionInfo {
        let src_reg = match map_3bit_field(opcode & 0x07) {
            Some(reg) => reg,
            None => panic!("Error src_reg and_a_r8")
        };

        let a = self.reg.read_reg(SingleReg::A);
        let r8 = self.reg.read_reg(src_reg);

        let result = a ^ r8;

        let z = result == 0;

        self.reg.set_flag(Flag::Z, z);
        self.reg.set_flag(Flag::N, false);
        self.reg.set_flag(Flag::H, false);
        self.reg.set_flag(Flag::CY, false);

        InstructionInfo(1, 4)
    }

    fn xor_a_n8(&mut self) -> InstructionInfo {
        let a = self.reg.read_reg(SingleReg::A);
        let n8 = self.mem.read(self.pc + 1);

        let result = a ^ n8;

        let z = result == 0;

        self.reg.set_flag(Flag::Z, z);
        self.reg.set_flag(Flag::N, false);
        self.reg.set_flag(Flag::H, false);
        self.reg.set_flag(Flag::CY, false);

        InstructionInfo(2, 8)
    }

    fn xor_a_hl(&mut self) -> InstructionInfo {
        let addr = self.reg.read_dreg(DoubleReg::HL);
        let a = self.reg.read_reg(SingleReg::A);
        let n8 = self.mem.read(addr);

        let result = a ^ n8;

        let z = result == 0;

        self.reg.set_flag(Flag::Z, z);
        self.reg.set_flag(Flag::N, false);
        self.reg.set_flag(Flag::H, false);
        self.reg.set_flag(Flag::CY, false);

        InstructionInfo(1, 8)
    }

    fn cp_a_r8(&mut self, opcode: u8) -> InstructionInfo {
        let src_reg = match map_3bit_field(opcode & 0x07) {
            Some(reg) => reg,
            None => panic!("Error src_reg sub_a_r8")
        };

        let r8 = self.reg.read_reg(src_reg);
        let a = self.reg.read_reg(SingleReg::A);

        let (result, cy) = a.overflowing_sub(r8);

        let z = result == 0;

        let (h_sub, _) = (a & 0x0F).overflowing_sub(r8 & 0x0F);
        let h = (h_sub & 0x10) == 0x10;

        self.reg.set_flag(Flag::Z, z);
        self.reg.set_flag(Flag::N, true);
        self.reg.set_flag(Flag::H, h);
        self.reg.set_flag(Flag::CY, cy);

        InstructionInfo(1, 4)
    }

    fn cp_a_n8(&mut self) -> InstructionInfo {
        let n8 = self.mem.read(self.pc + 1);
        let a = self.reg.read_reg(SingleReg::A);

        let (result, cy) = a.overflowing_sub(n8);

        let z = result == 0;

        let (h_sub, _) = (a & 0x0F).overflowing_sub(n8 & 0x0F);
        let h = (h_sub & 0x10) == 0x10;

        self.reg.set_flag(Flag::Z, z);
        self.reg.set_flag(Flag::N, true);
        self.reg.set_flag(Flag::H, h);
        self.reg.set_flag(Flag::CY, cy);

        InstructionInfo(2, 8)
    }

    fn cp_a_hl(&mut self) -> InstructionInfo {
        let addr = self.reg.read_dreg(DoubleReg::HL);
        let n8 = self.mem.read(addr);
        let a = self.reg.read_reg(SingleReg::A);

        let (result, cy) = a.overflowing_sub(n8);

        let z = result == 0;

        let (h_sub, _) = (a & 0x0F).overflowing_sub(n8 & 0x0F);
        let h = (h_sub & 0x10) == 0x10;

        self.reg.set_flag(Flag::Z, z);
        self.reg.set_flag(Flag::N, true);
        self.reg.set_flag(Flag::H, h);
        self.reg.set_flag(Flag::CY, cy);

        InstructionInfo(1, 8)
    }

    fn inc_r8(&mut self, opcode: u8) -> InstructionInfo {
        let target_reg = match map_3bit_field((opcode >> 3) & 0x07) {
            Some(reg) => reg,
            None => panic!("Error target_reg inc_r8")
        };

        let r8 = self.reg.read_reg(target_reg);

        let (result, _) = r8.overflowing_add(1u8);

        let z = result == 0;
        let h = (((r8 & 0x0F) + 1u8) & 0x10) == 0x10;

        self.reg.write_reg(target_reg, result);

        self.reg.set_flag(Flag::Z, z);
        self.reg.set_flag(Flag::N, false);
        self.reg.set_flag(Flag::H, h);

        InstructionInfo(1, 4)
    }

    fn inc_hl(&mut self) -> InstructionInfo {
        let addr = self.reg.read_dreg(DoubleReg::HL);
        let n8 = self.mem.read(addr);

        let (result, _) = n8.overflowing_add(1u8);

        let z = result == 0;
        let h = (((n8 & 0x0F) + 1u8) & 0x10) == 0x10;

        self.mem.write(addr, result);

        self.reg.set_flag(Flag::Z, z);
        self.reg.set_flag(Flag::N, false);
        self.reg.set_flag(Flag::H, h);

        InstructionInfo(1, 12)
    }

    fn dec_r8(&mut self, opcode: u8) -> InstructionInfo {
        let target_reg = match map_3bit_field((opcode >> 3) & 0x07) {
            Some(reg) => reg,
            None => panic!("Error target_reg dec_r8")
        };

        let r8 = self.reg.read_reg(target_reg);

        let (result, _) = r8.overflowing_sub(1u8);

        let z = result == 0;
        let h = (((r8 & 0x0F) - 1u8) & 0x10) == 0x10;

        self.reg.write_reg(target_reg, result);

        self.reg.set_flag(Flag::Z, z);
        self.reg.set_flag(Flag::N, true);
        self.reg.set_flag(Flag::H, h);

        InstructionInfo(1, 4)
    }

    fn dec_hl(&mut self) -> InstructionInfo {
        let addr = self.reg.read_dreg(DoubleReg::HL);
        let n8 = self.mem.read(addr);

        let (result, _) = n8.overflowing_sub(1u8);

        let z = result == 0;
        let h = (((n8 & 0x0F) - 1u8) & 0x10) == 0x10;

        self.mem.write(addr, result);

        self.reg.set_flag(Flag::Z, z);
        self.reg.set_flag(Flag::N, true);
        self.reg.set_flag(Flag::H, h);

        InstructionInfo(1, 12)
    }

    ////////////////////////////////////////////////
    //                                            //
    //  16-Bit Arithmetic Operation Instructions  //
    //                                            //
    ////////////////////////////////////////////////

    fn add_hl_ss(&mut self, opcode: u8) -> InstructionInfo {
        let ss = (opcode >> 4) & 0x03;

        let r16 = if ss == 0x03 {
            self.pc
        } else {
            let src_reg = match ss {
                0x00 => DoubleReg::BC,
                0x01 => DoubleReg::DE,
                0x02 => DoubleReg::HL,
                _ => panic!("Error src_reg add_hl_ss")
            };

            self.reg.read_dreg(src_reg)
        };

        let hl = self.reg.read_dreg(DoubleReg::HL);

        let (result, cy) = hl.overflowing_add(r16);

        let (h_add, _) = (hl & 0x0FFF).overflowing_add(r16 & 0x0FFF);
        let h = (h_add & 0x1000) == 0x1000;

        self.reg.write_dreg(DoubleReg::HL, result);

        self.reg.set_flag(Flag::N, false);
        self.reg.set_flag(Flag::H, h);
        self.reg.set_flag(Flag::CY, cy);

        InstructionInfo(1, 8)
    }

    fn add_sp_e(&mut self) -> InstructionInfo {
        let sp = self.sp as i32;
        let e = self.mem.read(self.pc + 1) as i32;

        let result = sp + e;

        let h = (((sp & 0x0FFF) + (e & 0x0FFF)) & 0x1000) == 0x1000;
        let cy = (((sp & 0xFFFF) + (e & 0xFFFF)) & 0x10000) == 0x10000;

        self.sp = result as u16;

        self.reg.set_flag(Flag::Z, false);
        self.reg.set_flag(Flag::N, false);
        self.reg.set_flag(Flag::H, h);
        self.reg.set_flag(Flag::CY, cy);

        InstructionInfo(2, 16)
    }

    fn inc_ss(&mut self, opcode: u8) -> InstructionInfo {
        let ss = (opcode >> 4) & 0x03;

        if ss == 0x03 {
            let (result, _) = self.pc.overflowing_add(1u16);
            self.pc = result;
        } else {
            let target_reg = match ss {
                0x00 => DoubleReg::BC,
                0x01 => DoubleReg::DE,
                0x02 => DoubleReg::HL,
                _ => panic!("Error target_reg inc_ss")
            };

            let (result, _) = self.reg.read_dreg(target_reg).overflowing_add(1u16);
            self.reg.write_dreg(target_reg, result);
        }

        InstructionInfo(1, 8)
    }

    fn dec_ss(&mut self, opcode: u8) -> InstructionInfo {
        let ss = (opcode >> 4) & 0x03;

        if ss == 0x03 {
            let (result, _) = self.pc.overflowing_sub(1u16);
            self.pc = result;
        } else {
            let target_reg = match ss {
                0x00 => DoubleReg::BC,
                0x01 => DoubleReg::DE,
                0x02 => DoubleReg::HL,
                _ => panic!("Error target_reg inc_ss")
            };

            let (result, _) = self.reg.read_dreg(target_reg).overflowing_sub(1u16);
            self.reg.write_dreg(target_reg, result);
        }

        InstructionInfo(1, 8)
    }

    /////////////////////////////////
    //                             //
    //  Rotate Shift Instructions  //
    //                             //
    /////////////////////////////////

    fn rlca(&mut self) -> InstructionInfo {
        let a = self.reg.read_reg(SingleReg::A);

        let result = (a << 1) | (a >> 7);

        let cy = if (a >> 7) == 0x01 {
            true
        } else {
            false
        };

        self.reg.write_reg(SingleReg::A, result);

        self.reg.set_flag(Flag::Z, false);
        self.reg.set_flag(Flag::N, false);
        self.reg.set_flag(Flag::H, false);
        self.reg.set_flag(Flag::CY, cy);

        InstructionInfo(1, 4)
    }

    fn rla(&mut self) -> InstructionInfo {
        let a = self.reg.read_reg(SingleReg::A);
        let a0 = if self.reg.get_flag(Flag::CY) { 1u8 } else { 0u8 };

        let result = (a << 1) | a0;

        let cy = if (a >> 7) == 0x01 {
            true
        } else {
            false
        };

        self.reg.write_reg(SingleReg::A, result);

        self.reg.set_flag(Flag::Z, false);
        self.reg.set_flag(Flag::N, false);
        self.reg.set_flag(Flag::H, false);
        self.reg.set_flag(Flag::CY, cy);

        InstructionInfo(1, 4)
    }

    fn rrca(&mut self) -> InstructionInfo {
        let a = self.reg.read_reg(SingleReg::A);

        let result = (a >> 1) | (a << 7);

        let cy = if (a & 0x01) == 0x01 {
            true
        } else {
            false
        };

        self.reg.write_reg(SingleReg::A, result);

        self.reg.set_flag(Flag::Z, false);
        self.reg.set_flag(Flag::N, false);
        self.reg.set_flag(Flag::H, false);
        self.reg.set_flag(Flag::CY, cy);

        InstructionInfo(1, 4)
    }

    fn rra(&mut self) -> InstructionInfo {
        let a = self.reg.read_reg(SingleReg::A);
        let a7 = if self.reg.get_flag(Flag::CY) { 0x80u8 } else { 0u8 };

        let result = (a >> 1) | a7;

        let cy = if (a & 0x01) == 0x01 {
            true
        } else {
            false
        };

        self.reg.write_reg(SingleReg::A, result);

        self.reg.set_flag(Flag::Z, false);
        self.reg.set_flag(Flag::N, false);
        self.reg.set_flag(Flag::H, false);
        self.reg.set_flag(Flag::CY, cy);

        InstructionInfo(1, 4)
    }

    fn rlc(&mut self) -> InstructionInfo {
        let source = self.mem.read(self.pc + 1);
        
        let mut src_reg = SingleReg::A;
        let mut addr = 0u16;

        let m = if source != 0x06 {
            src_reg = match map_3bit_field(source) {
                Some(reg) => reg,
                None => panic!("Error src_reg rlc")
            };

            self.reg.read_reg(src_reg)

        } else {
            addr = self.reg.read_dreg(DoubleReg::HL);
            self.mem.read(addr)
        };

        let result = (m << 1) | (m >> 7);

        let z = result == 0;
        let cy = (m >> 7) == 0x01;

        self.reg.set_flag(Flag::Z, z);
        self.reg.set_flag(Flag::N, false);
        self.reg.set_flag(Flag::H, false);
        self.reg.set_flag(Flag::CY, cy);

        if source != 0x06 {
            self.reg.write_reg(src_reg, result);
            InstructionInfo(2, 8)
        } else {
            self.mem.write(addr, result);
            InstructionInfo(2, 16)
        }
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn adc_a_r8() {
        let mut core = Core::new();

        core.reg.write_reg(SingleReg::A, 0xE1);
        core.reg.write_reg(SingleReg::E, 0x0F);
        core.reg.set_flag(Flag::CY, true);

        let opcode: u8 = 0x03;

        core.adc_a_r8(opcode);

        assert_eq!(core.reg.read_reg(SingleReg::A), 0xF1);
        assert_eq!(core.reg.get_flag(Flag::Z), false);
        assert_eq!(core.reg.get_flag(Flag::H), true);
        assert_eq!(core.reg.get_flag(Flag::CY), false);

        core.reg.write_reg(SingleReg::A, 0xE1);
        core.reg.write_reg(SingleReg::E, 0x3B);
        core.reg.set_flag(Flag::CY, true);

        core.adc_a_r8(opcode);

        assert_eq!(core.reg.read_reg(SingleReg::A), 0x1D);
        assert_eq!(core.reg.get_flag(Flag::Z), false);
        assert_eq!(core.reg.get_flag(Flag::H), false);
        assert_eq!(core.reg.get_flag(Flag::CY), true);

        core.reg.write_reg(SingleReg::A, 0xE1);
        core.reg.write_reg(SingleReg::E, 0x1E);
        core.reg.set_flag(Flag::CY, true);

        core.adc_a_r8(opcode);

        assert_eq!(core.reg.read_reg(SingleReg::A), 0x00);
        assert_eq!(core.reg.get_flag(Flag::Z), true);
        assert_eq!(core.reg.get_flag(Flag::H), true);
        assert_eq!(core.reg.get_flag(Flag::CY), true);
    }

    #[test]
    fn sub_a_r8() {
        let mut core = Core::new();

        core.reg.write_reg(SingleReg::A, 0x3E);
        core.reg.write_reg(SingleReg::E, 0x3E);

        let opcode: u8 = 0x03;

        core.sub_a_r8(opcode);

        assert_eq!(core.reg.read_reg(SingleReg::A), 0x00);
        assert_eq!(core.reg.get_flag(Flag::Z), true);
        assert_eq!(core.reg.get_flag(Flag::N), true);
        assert_eq!(core.reg.get_flag(Flag::H), false);
        assert_eq!(core.reg.get_flag(Flag::CY), false);

        core.reg.write_reg(SingleReg::A, 0x3E);
        core.reg.write_reg(SingleReg::E, 0x0F);

        core.sub_a_r8(opcode);

        assert_eq!(core.reg.read_reg(SingleReg::A), 0x2F);
        assert_eq!(core.reg.get_flag(Flag::Z), false);
        assert_eq!(core.reg.get_flag(Flag::N), true);
        assert_eq!(core.reg.get_flag(Flag::H), true);
        assert_eq!(core.reg.get_flag(Flag::CY), false);

        core.reg.write_reg(SingleReg::A, 0x3E);
        core.reg.write_reg(SingleReg::E, 0x40);

        core.sub_a_r8(opcode);

        assert_eq!(core.reg.read_reg(SingleReg::A), 0xFE);
        assert_eq!(core.reg.get_flag(Flag::Z), false);
        assert_eq!(core.reg.get_flag(Flag::N), true);
        assert_eq!(core.reg.get_flag(Flag::H), false);
        assert_eq!(core.reg.get_flag(Flag::CY), true);
    }

    #[test]
    fn sbc_a_r8() {
        let mut core = Core::new();

        core.reg.write_reg(SingleReg::A, 0x3B);
        core.reg.write_reg(SingleReg::H, 0x2A);
        core.reg.set_flag(Flag::CY, true);

        let opcode: u8 = 0x04;

        core.sbc_a_r8(opcode);

        assert_eq!(core.reg.read_reg(SingleReg::A), 0x10);
        assert_eq!(core.reg.get_flag(Flag::Z), false);
        assert_eq!(core.reg.get_flag(Flag::N), true);
        assert_eq!(core.reg.get_flag(Flag::H), false);
        assert_eq!(core.reg.get_flag(Flag::CY), false);

        core.reg.write_reg(SingleReg::A, 0x3B);
        core.reg.write_reg(SingleReg::H, 0x3A);
        core.reg.set_flag(Flag::CY, true);

        core.sbc_a_r8(opcode);

        assert_eq!(core.reg.read_reg(SingleReg::A), 0x00);
        assert_eq!(core.reg.get_flag(Flag::Z), true);
        assert_eq!(core.reg.get_flag(Flag::N), true);
        assert_eq!(core.reg.get_flag(Flag::H), false);
        assert_eq!(core.reg.get_flag(Flag::CY), false);

        core.reg.write_reg(SingleReg::A, 0x3B);
        core.reg.write_reg(SingleReg::H, 0x4F);
        core.reg.set_flag(Flag::CY, true);

        core.sbc_a_r8(opcode);

        assert_eq!(core.reg.read_reg(SingleReg::A), 0xEB);
        assert_eq!(core.reg.get_flag(Flag::Z), false);
        assert_eq!(core.reg.get_flag(Flag::N), true);
        assert_eq!(core.reg.get_flag(Flag::H), true);
        assert_eq!(core.reg.get_flag(Flag::CY), true);
    }

    #[test]
    fn add_hl_ss() {
        let mut core = Core::new();

        core.reg.write_dreg(DoubleReg::HL, 0x8A23);
        core.reg.write_dreg(DoubleReg::BC, 0x0605);

        let mut opcode: u8 = 0x00;

        core.add_hl_ss(opcode);

        assert_eq!(core.reg.read_dreg(DoubleReg::HL), 0x9028);
        assert_eq!(core.reg.get_flag(Flag::H), true);
        assert_eq!(core.reg.get_flag(Flag::CY), false);

        core.reg.write_dreg(DoubleReg::HL, 0x8A23);

        opcode = 0x20;

        core.add_hl_ss(opcode);

        assert_eq!(core.reg.read_dreg(DoubleReg::HL), 0x1446);
        assert_eq!(core.reg.get_flag(Flag::H), true);
        assert_eq!(core.reg.get_flag(Flag::CY), true);
    }

    #[test]
    fn rlca() {
        let mut core = Core::new();

        core.reg.write_reg(SingleReg::A, 0x85);
        core.reg.set_flag(Flag::CY, false);

        core.rlca();

        assert_eq!(core.reg.read_reg(SingleReg::A), 0x0B);
        assert_eq!(core.reg.get_flag(Flag::CY), true);
    }

}

