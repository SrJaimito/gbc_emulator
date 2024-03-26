use super::Core;
use super::register_file::{Reg8, Reg16, Flag, map_r8};

pub struct InstructionInfo(pub u8, pub u8); // (size, cycles)

impl<'a> Core<'a> {

    pub fn nop(&self) -> InstructionInfo {
        InstructionInfo(1, 1)
    }

    pub fn ld_r16_imm16(&mut self, opcode: u8) -> InstructionInfo {
        let lsb = self.mem.read(self.pc + 1) as u16;
        let msb = self.mem.read(self.pc + 2) as u16;

        let imm = (msb << 8) | lsb;

        match (opcode >> 4) & 0x03 {
            0x00 => self.reg.dwrite(Reg16::BC, imm),
            0x01 => self.reg.dwrite(Reg16::DE, imm),
            0x02 => self.reg.dwrite(Reg16::HL, imm),
            0x03 => self.sp = imm,
            _ => panic!("Error ld_r16_imm16")
        }

        InstructionInfo(3, 3)
    }

    pub fn ld_r16mem_a(&mut self, opcode: u8) -> InstructionInfo {
        let a = self.reg.read(Reg8::A);

        match (opcode >> 4) & 0x03 {
            0x00 => {
                let addr = self.reg.dread(Reg16::BC);
                self.mem.write(addr, a);
            },
            0x01 => {
                let addr = self.reg.dread(Reg16::DE);
                self.mem.write(addr, a);
            },
            0x02 => {
                let addr = self.reg.dread(Reg16::HL);
                self.mem.write(addr, a);
                self.reg.dwrite(Reg16::HL, addr + 1);
            },
            0x03 => {
                let addr = self.reg.dread(Reg16::HL);
                self.mem.write(addr, a);
                self.reg.dwrite(Reg16::HL, addr - 1);
            }
            _ => panic!("Error ld_r16mem_a")
        }

        InstructionInfo(1, 2)
    }

    pub fn ld_a_r16mem(&mut self, opcode: u8) -> InstructionInfo {
        match (opcode >> 4) & 0x03 {
            0x00 => {
                let addr = self.reg.dread(Reg16::BC);
                let a = self.mem.read(addr);

                self.reg.write(Reg8::A, a);
            },
            0x01 => {
                let addr = self.reg.dread(Reg16::DE);
                let a = self.mem.read(addr);

                self.reg.write(Reg8::A, a);
            },
            0x02 => {
                let addr = self.reg.dread(Reg16::HL);
                let a = self.mem.read(addr);

                self.reg.write(Reg8::A, a);
                self.reg.dwrite(Reg16::HL, addr + 1);
            },
            0x03 => {
                let addr = self.reg.dread(Reg16::HL);
                let a = self.mem.read(addr);

                self.reg.write(Reg8::A, a);
                self.reg.dwrite(Reg16::HL, addr - 1);
            }
            _ => panic!("Error ld_r16mem_a")
        }

        InstructionInfo(1, 2)
    }

    pub fn ld_imm16_sp(&mut self) -> InstructionInfo {
        let imm_lsb = self.mem.read(self.pc + 1) as u16;
        let imm_msb = self.mem.read(self.pc + 2) as u16;

        let addr = (imm_msb << 8) | imm_lsb;

        let sp_lsb = (self.pc & 0xFF) as u8;
        let sp_msb = (self.pc >> 8) as u8;

        self.mem.write(addr, sp_lsb);
        self.mem.write(addr + 1, sp_msb);

        InstructionInfo(3, 5)
    }

    pub fn inc_r16(&mut self, opcode: u8) -> InstructionInfo {
        match (opcode >> 4) & 0x03 {
            0x00 => {
                let r16 = self.reg.dread(Reg16::BC);
                let (result, _) = r16.overflowing_add(1u16);

                self.reg.dwrite(Reg16::BC, result);
            },
            0x01 => {
                let r16 = self.reg.dread(Reg16::DE);
                let (result, _) = r16.overflowing_add(1u16);

                self.reg.dwrite(Reg16::DE, result);
            },
            0x02 => {
                let r16 = self.reg.dread(Reg16::HL);
                let (result, _) = r16.overflowing_add(1u16);

                self.reg.dwrite(Reg16::HL, result);
            },
            0x03 => {
                let (result, _) = self.sp.overflowing_add(1u16);

                self.sp = result;
            },
            _ => panic!("Error inc_r16")
        }

        InstructionInfo(1, 2)
    }

    pub fn dec_r16(&mut self, opcode: u8) -> InstructionInfo {
        match (opcode >> 4) & 0x03 {
            0x00 => {
                let r16 = self.reg.dread(Reg16::BC);
                let (result, _) = r16.overflowing_sub(1u16);

                self.reg.dwrite(Reg16::BC, result);
            },
            0x01 => {
                let r16 = self.reg.dread(Reg16::DE);
                let (result, _) = r16.overflowing_sub(1u16);

                self.reg.dwrite(Reg16::DE, result);
            },
            0x02 => {
                let r16 = self.reg.dread(Reg16::HL);
                let (result, _) = r16.overflowing_sub(1u16);

                self.reg.dwrite(Reg16::HL, result);
            },
            0x03 => {
                let (result, _) = self.sp.overflowing_sub(1u16);

                self.sp = result;
            },
            _ => panic!("Error dec_r16")
        }

        InstructionInfo(1, 2)
    }

    pub fn add_hl_r16(&mut self, opcode: u8) -> InstructionInfo {
        let hl = self.reg.dread(Reg16::HL);
        let r16 = match (opcode >> 4) & 0x03 {
            0x00 => self.reg.dread(Reg16::BC),
            0x01 => self.reg.dread(Reg16::DE),
            0x02 => hl,
            0x03 => self.pc,
            _ => panic!("Error add_hl_r16")
        };

        let (result, cy) = hl.overflowing_add(r16);
        let h = ((hl & 0x0FFF) + (r16 & 0x0FFF)) & 0x1000 == 0x1000;

        self.reg.dwrite(Reg16::HL, result);

        self.reg.write_flag(Flag::N, false);
        self.reg.write_flag(Flag::H, h);
        self.reg.write_flag(Flag::CY, cy);

        InstructionInfo(1, 2)
    }

    pub fn inc_r8(&mut self, opcode: u8) -> InstructionInfo {
        let bit_field = (opcode >> 3) & 0x07;

        let (z, h, cycles) = if bit_field == 0x06 {
            let addr = self.reg.dread(Reg16::HL);
            let r8 = self.mem.read(addr);

            let (result, _) = r8.overflowing_add(1u8);
            let z = result == 0;
            let h = ((r8 & 0x0F) + 0x01) & 0x10 == 0x10;

            self.mem.write(addr, result);

            (z, h, 3)

        } else {
            let target_reg = map_r8(bit_field);
            let r8 = self.reg.read(target_reg);

            let (result, _) = r8.overflowing_add(1u8);
            let z = result == 0;
            let h = ((r8 & 0x0F) + 0x01) & 0x10 == 0x10;

            self.reg.write(target_reg, result);

            (z, h, 1)
        };

        self.reg.write_flag(Flag::Z, z);
        self.reg.write_flag(Flag::N, false);
        self.reg.write_flag(Flag::H, h);

        InstructionInfo(1, cycles)
    }

    pub fn dec_r8(&mut self, opcode: u8) -> InstructionInfo {
        let bit_field = (opcode >> 3) & 0x07;

        let (z, h, cycles) = if bit_field == 0x06 {
            let addr = self.reg.dread(Reg16::HL);
            let r8 = self.mem.read(addr);

            let (result, _) = r8.overflowing_sub(1u8);
            let z = result == 0;
            let h = ((r8 & 0x0F) + 0x01) & 0x10 == 0x10;

            self.mem.write(addr, result);

            (z, h, 3)

        } else {
            let target_reg = map_r8(bit_field);
            let r8 = self.reg.read(target_reg);

            let (result, _) = r8.overflowing_sub(1u8);
            let z = result == 0;
            let h = ((r8 & 0x0F) + 0x01) & 0x10 == 0x10;

            self.reg.write(target_reg, result);

            (z, h, 1)
        };

        self.reg.write_flag(Flag::Z, z);
        self.reg.write_flag(Flag::N, true);
        self.reg.write_flag(Flag::H, h);

        InstructionInfo(1, cycles)
    }

    pub fn ld_r8_imm8(&mut self, opcode: u8) -> InstructionInfo {
        let bit_field = (opcode >> 3) & 0x07;
        let imm = self.mem.read(self.pc + 1);

        let cycles = if bit_field == 0x06 {
            let addr = self.reg.dread(Reg16::HL);
            self.mem.write(addr, imm);

            3

        } else {
            let target_reg = map_r8(bit_field);
            self.reg.write(target_reg, imm);

            2
        };

        InstructionInfo(2, cycles)
    }

    pub fn rlca(&mut self) -> InstructionInfo {
        let a = self.reg.read(Reg8::A);
        let a7 = a >> 7;

        let result = (a << 1) | a7;
        let cy = a7 == 0x01;

        self.reg.write(Reg8::A, result);

        self.reg.write_flag(Flag::Z, false);
        self.reg.write_flag(Flag::N, false);
        self.reg.write_flag(Flag::H, false);
        self.reg.write_flag(Flag::CY, cy);

        InstructionInfo(1, 1)
    }

    pub fn rrca(&mut self) -> InstructionInfo {
        let a = self.reg.read(Reg8::A);
        let a0 = a & 0x01;

        let result = (a0 << 7) | (a & 0x7F);
        let cy = a0 == 0x01;

        self.reg.write(Reg8::A, result);

        self.reg.write_flag(Flag::Z, false);
        self.reg.write_flag(Flag::N, false);
        self.reg.write_flag(Flag::H, false);
        self.reg.write_flag(Flag::CY, cy);

        InstructionInfo(1, 1)
    }

    pub fn rla(&mut self) -> InstructionInfo {
        let a = self.reg.read(Reg8::A);
        let a0 = if self.reg.read_flag(Flag::CY) { 1u8 } else { 0u8 };

        let result = (a << 1) | a0;
        let cy = (a >> 7) == 0x01;

        self.reg.write(Reg8::A, result);

        self.reg.write_flag(Flag::Z, false);
        self.reg.write_flag(Flag::N, false);
        self.reg.write_flag(Flag::H, false);
        self.reg.write_flag(Flag::CY, cy);

        InstructionInfo(1, 1)
    }

    pub fn rra(&mut self) -> InstructionInfo {
        let a = self.reg.read(Reg8::A);
        let a7 = if self.reg.read_flag(Flag::CY) { 0x80u8 } else { 0u8 };

        let result = a7 | (a >> 1);
        let cy = (a & 0x01) == 0x01;

        self.reg.write(Reg8::A, result);

        self.reg.write_flag(Flag::Z, false);
        self.reg.write_flag(Flag::N, false);
        self.reg.write_flag(Flag::H, false);
        self.reg.write_flag(Flag::CY, cy);

        InstructionInfo(1, 1)
    }

    pub fn daa(&mut self) -> InstructionInfo {
        let a = self.reg.read(Reg8::A);
        let old_h = self.reg.read_flag(Flag::H);
        let old_cy = self.reg.read_flag(Flag::CY);
        
        let units = a & 0x0F;
        let tens = a >> 4;

        let mut correction = 0u8;
        let mut cy = false;

        if old_h || (units > 9) {
           correction |= 0x06; 
        }

        if old_cy || (tens > 9) {
            correction |= 0x60;
            cy = true;
        }

        let (result, _) = if self.reg.read_flag(Flag::N) {
            a.overflowing_sub(correction)
        } else {
            a.overflowing_add(correction)
        };

        let z = result == 0;

        self.reg.write(Reg8::A, result);

        self.reg.write_flag(Flag::Z, z);
        self.reg.write_flag(Flag::H, false);
        self.reg.write_flag(Flag::CY, cy);

        InstructionInfo(1, 1)
    }

    pub fn cpl(&mut self) -> InstructionInfo {
        let result = !self.reg.read(Reg8::A);

        self.reg.write(Reg8::A, result);

        self.reg.write_flag(Flag::N, false);
        self.reg.write_flag(Flag::H, false);

        InstructionInfo(1, 1)
    }

    pub fn scf(&mut self) -> InstructionInfo {
        self.reg.write_flag(Flag::N, false);
        self.reg.write_flag(Flag::H, false);
        self.reg.write_flag(Flag::CY, true);

        InstructionInfo(1, 1)
    }

    pub fn ccf(&mut self) -> InstructionInfo {
        let cy = !self.reg.read_flag(Flag::CY);

        self.reg.write_flag(Flag::N, false);
        self.reg.write_flag(Flag::H, false);
        self.reg.write_flag(Flag::CY, cy);

        InstructionInfo(1, 1)
    }

    pub fn jr_imm8(&mut self) -> InstructionInfo {
        let pc = self.pc as i32;
        let imm = self.mem.read(self.pc + 1) as i32;

        let result = pc + imm;

        self.pc = result as u16;

        InstructionInfo(2, 3)
    }

    pub fn jr_cond_imm8(&mut self, opcode: u8) -> InstructionInfo {
        match (opcode >> 3) & 0x03 {
            0x00 => {
                if !self.reg.read_flag(Flag::Z) {
                    return self.jr_imm8();
                }
            },
            0x01 => {
                if self.reg.read_flag(Flag::Z) {
                    return self.jr_imm8();
                }
            },
            0x02 => {
                if !self.reg.read_flag(Flag::CY) {
                    return self.jr_imm8();
                }
            },
            0x03 => {
                if self.reg.read_flag(Flag::CY) {
                    return self.jr_imm8();
                }
            },
            _ => panic!("Error jr_cond_imm8")
        };

        InstructionInfo(2, 2)
    }

    pub fn stop(&self) -> InstructionInfo {
        InstructionInfo(2, 0)
    }

    pub fn ld_r8_r8(&mut self, opcode: u8) -> InstructionInfo {
        let src_bit_field = opcode & 0x07;
        let dst_bit_field = (opcode >> 3) & 0x07;

        if src_bit_field == 0x06 {
            if dst_bit_field == 0x06 {
                return self.halt();
            }

            let addr = self.reg.dread(Reg16::HL);
            let value = self.mem.read(addr);

            let dst_reg = map_r8(dst_bit_field);

            self.reg.write(dst_reg, value);

            InstructionInfo(1, 2)
        } else {
            let src_reg = map_r8(src_bit_field);
            let value = self.reg.read(src_reg);

            if dst_bit_field == 0x06 {
                let addr = self.reg.dread(Reg16::HL);
                self.mem.write(addr, value);

                InstructionInfo(1, 2)
            } else {
                let dst_reg = map_r8(dst_bit_field);
                self.reg.write(dst_reg, value);

                InstructionInfo(1, 1)
            }
        }
    }

    pub fn halt(&self) -> InstructionInfo {
        unimplemented!()
    }

    pub fn add_a_r8(&mut self, opcode: u8) -> InstructionInfo {
        let src_bit_field = opcode & 0x07;

        let (value, cycles) = if src_bit_field == 0x06 {
            let addr = self.reg.dread(Reg16::HL);
            (self.mem.read(addr), 2)
        } else {
            let src_reg = map_r8(src_bit_field);
            (self.reg.read(src_reg), 1)
        };

        let a = self.reg.read(Reg8::A);

        let (result, cy) = a.overflowing_add(value);

        let z = result == 0;
        let h = ((a & 0x0F) + (value & 0x0F)) & 0x10 == 0x10;

        self.reg.write(Reg8::A, result);

        self.reg.write_flag(Flag::Z, z);
        self.reg.write_flag(Flag::N, false);
        self.reg.write_flag(Flag::H, h);
        self.reg.write_flag(Flag::CY, cy);

        InstructionInfo(1, cycles)
    }

    pub fn adc_a_r8(&mut self, opcode: u8) -> InstructionInfo {
        let src_bit_field = opcode & 0x07;

        let (value, cycles) = if src_bit_field == 0x06 {
            let addr = self.reg.dread(Reg16::HL);
            (self.mem.read(addr), 2)
        } else {
            let src_reg = map_r8(src_bit_field);
            (self.reg.read(src_reg), 1)
        };

        let a = self.reg.read(Reg8::A);
        let carry = if self.reg.read_flag(Flag::CY) { 1u8 } else { 0u8 };

        let (result, cy) = a.overflowing_add(value);
        let (result_carry, cy_carry) = result.overflowing_add(carry);

        let z = result_carry == 0;
        let h = ((a & 0x0F) + (value & 0x0F) + carry) & 0x10 == 0x10;

        self.reg.write(Reg8::A, result_carry);

        self.reg.write_flag(Flag::Z, z);
        self.reg.write_flag(Flag::N, false);
        self.reg.write_flag(Flag::H, h);
        self.reg.write_flag(Flag::CY, cy || cy_carry);

        InstructionInfo(1, cycles)
    }

    pub fn sub_a_r8(&mut self, opcode: u8) -> InstructionInfo {
        let src_bit_field = opcode & 0x07;

        let (value, cycles) = if src_bit_field == 0x06 {
            let addr = self.reg.dread(Reg16::HL);
            (self.mem.read(addr), 2)
        } else {
            let src_reg = map_r8(src_bit_field);
            (self.reg.read(src_reg), 1)
        };

        let a = self.reg.read(Reg8::A);

        let (result, cy) = a.overflowing_sub(value);

        let z = result == 0;
        let h = ((a & 0x0F) - (value & 0x0F)) & 0x10 == 0x10;

        self.reg.write(Reg8::A, result);

        self.reg.write_flag(Flag::Z, z);
        self.reg.write_flag(Flag::N, true);
        self.reg.write_flag(Flag::H, h);
        self.reg.write_flag(Flag::CY, cy);

        InstructionInfo(1, cycles)
    }

    pub fn sbc_a_r8(&mut self, opcode: u8) -> InstructionInfo {
        let src_bit_field = opcode & 0x07;

        let (value, cycles) = if src_bit_field == 0x06 {
            let addr = self.reg.dread(Reg16::HL);
            (self.mem.read(addr), 2)
        } else {
            let src_reg = map_r8(src_bit_field);
            (self.reg.read(src_reg), 1)
        };

        let a = self.reg.read(Reg8::A);
        let carry = if self.reg.read_flag(Flag::CY) { 1u8 } else { 0u8 };

        let (result, cy) = a.overflowing_sub(value);
        let (result_carry, cy_carry) = result.overflowing_sub(carry);

        let z = result_carry == 0;
        let h = ((a & 0x0F) - (value & 0x0F) - carry) & 0x10 == 0x10;

        self.reg.write(Reg8::A, result_carry);

        self.reg.write_flag(Flag::Z, z);
        self.reg.write_flag(Flag::N, true);
        self.reg.write_flag(Flag::H, h);
        self.reg.write_flag(Flag::CY, cy || cy_carry);

        InstructionInfo(1, cycles)
    }

    pub fn and_a_r8(&mut self, opcode: u8) -> InstructionInfo {
        let src_bit_field = opcode & 0x07;

        let (value, cycles) = if src_bit_field == 0x06 {
            let addr = self.reg.dread(Reg16::HL);
            (self.mem.read(addr), 2)
        } else {
            let src_reg = map_r8(src_bit_field);
            (self.reg.read(src_reg), 1)
        };

        let a = self.reg.read(Reg8::A);

        let result = a & value;

        let z = result == 0;

        self.reg.write(Reg8::A, result);

        self.reg.write_flag(Flag::Z, z);
        self.reg.write_flag(Flag::N, false);
        self.reg.write_flag(Flag::H, true);
        self.reg.write_flag(Flag::CY, false);

        InstructionInfo(1, cycles)
    }

    pub fn xor_a_r8(&mut self, opcode: u8) -> InstructionInfo {
        let src_bit_field = opcode & 0x07;

        let (value, cycles) = if src_bit_field == 0x06 {
            let addr = self.reg.dread(Reg16::HL);
            (self.mem.read(addr), 2)
        } else {
            let src_reg = map_r8(src_bit_field);
            (self.reg.read(src_reg), 1)
        };

        let a = self.reg.read(Reg8::A);

        let result = a ^ value;

        let z = result == 0;

        self.reg.write(Reg8::A, result);

        self.reg.write_flag(Flag::Z, z);
        self.reg.write_flag(Flag::N, false);
        self.reg.write_flag(Flag::H, false);
        self.reg.write_flag(Flag::CY, false);

        InstructionInfo(1, cycles)
    }

    pub fn or_a_r8(&mut self, opcode: u8) -> InstructionInfo {
        let src_bit_field = opcode & 0x07;

        let (value, cycles) = if src_bit_field == 0x06 {
            let addr = self.reg.dread(Reg16::HL);
            (self.mem.read(addr), 2)
        } else {
            let src_reg = map_r8(src_bit_field);
            (self.reg.read(src_reg), 1)
        };

        let a = self.reg.read(Reg8::A);

        let result = a | value;

        let z = result == 0;

        self.reg.write(Reg8::A, result);

        self.reg.write_flag(Flag::Z, z);
        self.reg.write_flag(Flag::N, false);
        self.reg.write_flag(Flag::H, false);
        self.reg.write_flag(Flag::CY, false);

        InstructionInfo(1, cycles)
    }

    pub fn cp_a_r8(&mut self, opcode: u8) -> InstructionInfo {
        let src_bit_field = opcode & 0x07;

        let (value, cycles) = if src_bit_field == 0x06 {
            let addr = self.reg.dread(Reg16::HL);
            (self.mem.read(addr), 2)
        } else {
            let src_reg = map_r8(src_bit_field);
            (self.reg.read(src_reg), 1)
        };

        let a = self.reg.read(Reg8::A);

        let (result, cy) = a.overflowing_sub(value);

        let z = result == 0;
        let h = ((a & 0x0F) - (value & 0x0F)) & 0x10 == 0x10;

        self.reg.write_flag(Flag::Z, z);
        self.reg.write_flag(Flag::N, true);
        self.reg.write_flag(Flag::H, h);
        self.reg.write_flag(Flag::CY, cy);

        InstructionInfo(1, cycles)
    }

    pub fn add_a_imm8(&mut self) -> InstructionInfo {
        let a = self.reg.read(Reg8::A);
        let value = self.mem.read(self.pc + 1);

        let (result, cy) = a.overflowing_add(value);

        let z = result == 0;
        let h = ((a & 0x0F) + (value & 0x0F)) & 0x10 == 0x10;

        self.reg.write(Reg8::A, result);

        self.reg.write_flag(Flag::Z, z);
        self.reg.write_flag(Flag::N, false);
        self.reg.write_flag(Flag::H, h);
        self.reg.write_flag(Flag::CY, cy);

        InstructionInfo(2, 2)
    }

    pub fn adc_a_imm8(&mut self) -> InstructionInfo {
        let a = self.reg.read(Reg8::A);
        let value = self.mem.read(self.pc + 1);
        let carry = if self.reg.read_flag(Flag::CY) { 1u8 } else { 0u8 };

        let (result, cy) = a.overflowing_add(value);
        let (result_carry, cy_carry) = result.overflowing_add(carry);

        let z = result_carry == 0;
        let h = ((a & 0x0F) + (value & 0x0F) + carry) & 0x10 == 0x10;

        self.reg.write(Reg8::A, result_carry);

        self.reg.write_flag(Flag::Z, z);
        self.reg.write_flag(Flag::N, false);
        self.reg.write_flag(Flag::H, h);
        self.reg.write_flag(Flag::CY, cy || cy_carry);

        InstructionInfo(2, 2)
    }

    pub fn sub_a_imm8(&mut self) -> InstructionInfo {
        let a = self.reg.read(Reg8::A);
        let value = self.mem.read(self.pc + 1);

        let (result, cy) = a.overflowing_sub(value);

        let z = result == 0;
        let h = ((a & 0x0F) - (value & 0x0F)) & 0x10 == 0x10;

        self.reg.write(Reg8::A, result);

        self.reg.write_flag(Flag::Z, z);
        self.reg.write_flag(Flag::N, true);
        self.reg.write_flag(Flag::H, h);
        self.reg.write_flag(Flag::CY, cy);

        InstructionInfo(2, 2)
    }

    pub fn sbc_a_imm8(&mut self) -> InstructionInfo {
        let a = self.reg.read(Reg8::A);
        let value = self.mem.read(self.pc + 1);
        let carry = if self.reg.read_flag(Flag::CY) { 1u8 } else { 0u8 };

        let (result, cy) = a.overflowing_sub(value);
        let (result_carry, cy_carry) = result.overflowing_sub(carry);

        let z = result_carry == 0;
        let h = ((a & 0x0F) - (value & 0x0F) - carry) & 0x10 == 0x10;

        self.reg.write(Reg8::A, result_carry);

        self.reg.write_flag(Flag::Z, z);
        self.reg.write_flag(Flag::N, true);
        self.reg.write_flag(Flag::H, h);
        self.reg.write_flag(Flag::CY, cy || cy_carry);

        InstructionInfo(2, 2)
    }

    pub fn and_a_imm8(&mut self) -> InstructionInfo {
        let a = self.reg.read(Reg8::A);
        let value = self.mem.read(self.pc + 1);

        let result = a & value;

        let z = result == 0;

        self.reg.write(Reg8::A, result);

        self.reg.write_flag(Flag::Z, z);
        self.reg.write_flag(Flag::N, false);
        self.reg.write_flag(Flag::H, true);
        self.reg.write_flag(Flag::CY, false);

        InstructionInfo(2, 2)
    }

    pub fn xor_a_imm8(&mut self) -> InstructionInfo {
        let a = self.reg.read(Reg8::A);
        let value = self.mem.read(self.pc + 1);

        let result = a ^ value;

        let z = result == 0;

        self.reg.write(Reg8::A, result);

        self.reg.write_flag(Flag::Z, z);
        self.reg.write_flag(Flag::N, false);
        self.reg.write_flag(Flag::H, false);
        self.reg.write_flag(Flag::CY, false);

        InstructionInfo(2, 2)
    }

    pub fn or_a_imm8(&mut self) -> InstructionInfo {
        let a = self.reg.read(Reg8::A);
        let value = self.mem.read(self.pc + 1);

        let result = a | value;

        let z = result == 0;

        self.reg.write(Reg8::A, result);

        self.reg.write_flag(Flag::Z, z);
        self.reg.write_flag(Flag::N, false);
        self.reg.write_flag(Flag::H, false);
        self.reg.write_flag(Flag::CY, false);

        InstructionInfo(2, 2)
    }

    pub fn cp_a_imm8(&mut self) -> InstructionInfo {
        let a = self.reg.read(Reg8::A);
        let value = self.mem.read(self.pc + 1);

        let (result, cy) = a.overflowing_sub(value);

        let z = result == 0;
        let h = ((a & 0x0F) - (value & 0x0F)) & 0x10 == 0x10;

        self.reg.write_flag(Flag::Z, z);
        self.reg.write_flag(Flag::N, true);
        self.reg.write_flag(Flag::H, h);
        self.reg.write_flag(Flag::CY, cy);

        InstructionInfo(2, 2)
    }

    pub fn ret_cond(&mut self, opcode: u8) -> InstructionInfo {
        let cycles = match (opcode >> 3) & 0x03 {
            0x00 => {
                if !self.reg.read_flag(Flag::Z) {
                    self.ret();
                    5
                } else {
                    2
                }
            },
            0x01 => {
                if self.reg.read_flag(Flag::Z) {
                    self.ret();
                    5
                } else {
                    2
                }
            },
            0x02 => {
                if !self.reg.read_flag(Flag::CY) {
                    self.ret();
                    5
                } else {
                    2
                }
            },
            0x03 => {
                if self.reg.read_flag(Flag::CY) {
                    self.ret();
                    5
                } else {
                    2
                }
            },
            _ => panic!("Error ret_cond")
        };

        InstructionInfo(1, cycles)
    }

    pub fn ret(&mut self) -> InstructionInfo {
        let pc_lsb = self.mem.read(self.sp) as u16;
        let pc_msb = self.mem.read(self.sp + 1) as u16;

        self.pc = (pc_msb << 8) | pc_lsb;
        self.sp += 2;

        InstructionInfo(1, 4)
    }

    pub fn reti(&mut self) -> InstructionInfo {
        // IME is set right after this instruction
        self.ime_enable_request = 1;

        self.ret()
    }

    pub fn jp_cond_imm16(&mut self, opcode: u8) -> InstructionInfo {
        match (opcode >> 3) & 0x03 {
            0x00 => {
                if !self.reg.read_flag(Flag::Z) {
                    return self.jp_imm16();
                }
            },
            0x01 => {
                if self.reg.read_flag(Flag::Z) {
                    return self.jp_imm16();
                }
            },
            0x02 => {
                if !self.reg.read_flag(Flag::CY) {
                    return self.jp_imm16();
                }
            },
            0x03 => {
                if self.reg.read_flag(Flag::CY) {
                    return self.jp_imm16();
                }
            },
            _ => panic!("Error jp_cond_imm16")
        }

        InstructionInfo(3, 3)
    }

    pub fn jp_imm16(&mut self) -> InstructionInfo {
        let lsb = self.mem.read(self.pc + 1) as u16;
        let msb = self.mem.read(self.pc + 2) as u16;

        self.pc = (msb << 8) | lsb;

        InstructionInfo(0, 4)
    }

    pub fn jp_hl(&mut self) -> InstructionInfo {
        self.pc = self.reg.dread(Reg16::HL);

        InstructionInfo(0, 1)
    }

    pub fn call_cond_imm16(&mut self, opcode: u8) -> InstructionInfo {
        match (opcode >> 3) & 0x03 {
            0x00 => {
                if !self.reg.read_flag(Flag::Z) {
                    return self.call_imm16();
                }
            },
            0x01 => {
                if self.reg.read_flag(Flag::Z) {
                    return self.call_imm16();
                }
            },
            0x02 => {
                if !self.reg.read_flag(Flag::CY) {
                    return self.call_imm16();
                }
            },
            0x03 => {
                if self.reg.read_flag(Flag::CY) {
                    return self.call_imm16();
                }
            },
            _ => panic!("Error call_cond_imm16")
        }

        InstructionInfo(3, 3)
    }

    pub fn call_imm16(&mut self) -> InstructionInfo {
        let return_addr = self.pc + 3;

        let return_addr_lsb = return_addr & 0x00FF;
        let return_addr_msb = return_addr >> 8;

        self.mem.write(self.sp - 1, return_addr_lsb as u8);
        self.mem.write(self.sp - 2, return_addr_msb as u8);

        self.sp -= 2;

        let jump_addr_lsb = self.mem.read(self.pc + 1) as u16;
        let jump_addr_msb = self.mem.read(self.pc + 2) as u16;

        let jump_addr = (jump_addr_msb << 8) | jump_addr_lsb;

        self.pc = jump_addr;

        InstructionInfo(0, 6)
    }

    pub fn rst_tgt3(&mut self, opcode: u8) -> InstructionInfo {
        let jump_addr: u16 = match (opcode >> 3) & 0x07 {
            0x00 => 0x0000,
            0x01 => 0x0008,
            0x02 => 0x0010,
            0x03 => 0x0018,
            0x04 => 0x0020,
            0x05 => 0x0028,
            0x06 => 0x0030,
            0x07 => 0x0038,
            _ => panic!("Error rst_tgt3")
        };

        self.mem.write(self.sp - 1, (self.pc >> 8) as u8);
        self.mem.write(self.sp - 2, (self.pc & 0x00FF) as u8);
        self.sp -= 2;

        self.pc = jump_addr;

        InstructionInfo(0, 4)
    }

    pub fn pop_r16stk(&mut self, opcode: u8) -> InstructionInfo {
        let dst_reg = match (opcode >> 4) & 0x03 {
            0x00 => Reg16::BC,
            0x01 => Reg16::DE,
            0x02 => Reg16::HL,
            0x03 => Reg16::AF,
            _ => panic!("Error pop_r16stk")
        };

        let lsb = self.mem.read(self.sp) as u16;
        let msb = self.mem.read(self.sp + 1) as u16;

        let value = (msb << 8) | lsb;

        self.reg.dwrite(dst_reg, value);
        self.sp += 2;

        InstructionInfo(1, 3)
    }

    pub fn push_r16stk(&mut self, opcode: u8) -> InstructionInfo {
        let src_reg = match (opcode >> 4) & 0x03 {
            0x00 => Reg16::BC,
            0x01 => Reg16::DE,
            0x02 => Reg16::HL,
            0x03 => Reg16::AF,
            _ => panic!("Error push_r16stk")
        };

        let value = self.reg.dread(src_reg);

        let lsb = value & 0x00FF;
        let msb = value >> 8;

        self.mem.write(self.sp - 1, msb as u8);
        self.mem.write(self.sp - 2, lsb as u8);
        self.sp -= 2;

        InstructionInfo(1, 4)
    }

    pub fn prefix(&mut self) -> InstructionInfo {
        self.prefix_enabled = true;

        InstructionInfo(1, 1)
    }

    pub fn ldh_c_a(&mut self) -> InstructionInfo {
        let a = self.reg.read(Reg8::A);
        let c = self.reg.read(Reg8::C) as u16;
        let addr = 0xFF00 + c;

        self.mem.write(addr, a);

        InstructionInfo(1, 2)
    }

    pub fn ldh_imm8_a(&mut self) -> InstructionInfo {
        let a = self.reg.read(Reg8::A);
        let imm = self.mem.read(self.pc + 1) as u16;
        let addr = 0xFF00 + imm;

        self.mem.write(addr, a);

        InstructionInfo(2, 3)
    }

    pub fn ld_imm16_a(&mut self) -> InstructionInfo {
        let addr_lsb = self.mem.read(self.pc + 1) as u16;
        let addr_msb = self.mem.read(self.pc + 2) as u16;

        let addr = (addr_msb << 8) | addr_lsb;

        let a = self.reg.read(Reg8::A);

        self.mem.write(addr, a);

        InstructionInfo(3, 4)
    }

    pub fn ldh_a_c(&mut self) -> InstructionInfo {
        let c = self.reg.read(Reg8::C) as u16;
        let addr = 0xFF00 + c;
        let value = self.mem.read(addr);

        self.reg.write(Reg8::A, value);

        InstructionInfo(1, 2)
    }

    pub fn ldh_a_imm8(&mut self) -> InstructionInfo {
        let imm = self.mem.read(self.pc + 1) as u16;
        let addr = 0xFF00 + imm;
        let value = self.mem.read(addr);

        self.reg.write(Reg8::A, value);

        InstructionInfo(2, 3)
    }

    pub fn ld_a_imm16(&mut self) -> InstructionInfo {
        let addr_lsb = self.mem.read(self.pc + 1) as u16;
        let addr_msb = self.mem.read(self.pc + 2) as u16;

        let addr = (addr_msb << 8) | addr_lsb;

        let value = self.mem.read(addr);

        self.reg.write(Reg8::A, value);

        InstructionInfo(3, 4)
    }

    pub fn add_sp_imm8(&mut self) -> InstructionInfo {
        let sp = self.sp as i32;
        let imm = self.mem.read(self.pc + 1) as i32;

        let result = sp + imm;

        let h = ((sp & 0xF) + (imm & 0xF)) & 0x10 == 0x10;
        let cy = ((sp & 0xFF) + (imm & 0xFF)) & 0x100 == 0x100;

        self.sp = result as u16;

        self.reg.write_flag(Flag::Z, false);
        self.reg.write_flag(Flag::N, false);
        self.reg.write_flag(Flag::H, h);
        self.reg.write_flag(Flag::CY, cy);

        InstructionInfo(2, 4)
    }

    pub fn ld_hl_sp_imm8(&mut self) -> InstructionInfo {
        let sp = self.sp as i32;
        let imm = self.mem.read(self.pc + 1) as i32;

        let result = sp + imm;

        let h = ((sp & 0xF) + (imm & 0xF)) & 0x10 == 0x10;
        let cy = ((sp & 0xFF) + (imm & 0xFF)) & 0x100 == 0x100;

        self.reg.dwrite(Reg16::HL, result as u16);

        self.reg.write_flag(Flag::Z, false);
        self.reg.write_flag(Flag::N, false);
        self.reg.write_flag(Flag::H, h);
        self.reg.write_flag(Flag::CY, cy);

        InstructionInfo(2, 3)
    }

    pub fn ld_sp_hl(&mut self) -> InstructionInfo {
        let hl = self.reg.dread(Reg16::HL);

        self.sp = hl;

        InstructionInfo(1, 2)
    }

    pub fn di(&mut self) -> InstructionInfo {
        // IME has to be cleared right now
        self.ime_enabled = false;
        self.ime_enable_request = 0;

        InstructionInfo(1, 1)
    }

    pub fn ei(&mut self) -> InstructionInfo {
        // IME is set after the instruction following this one
        self.ime_enable_request = 2;

        InstructionInfo(1, 1)
    }

    pub fn rlc_r8(&mut self, opcode: u8) -> InstructionInfo {
        let r8_bit_field = opcode & 0x07;

        let cycles = if r8_bit_field == 0x06 {
            let addr = self.reg.dread(Reg16::HL);
            let b = self.mem.read(addr);

            let b7 = b >> 7;

            let result = (b << 1) | b7;
            let z = result == 0;
            let cy = b7 == 0x01;

            self.mem.write(addr, result);

            self.reg.write_flag(Flag::Z, z);
            self.reg.write_flag(Flag::CY, cy);

            4

        } else {
            let target_reg = map_r8(r8_bit_field);
            let b = self.reg.read(target_reg);

            let b7 = b >> 7;

            let result = (b << 1) | b7;
            let z = result == 0;
            let cy = b7 == 0x01;

            self.reg.write(target_reg, result);

            self.reg.write_flag(Flag::Z, z);
            self.reg.write_flag(Flag::CY, cy);

            2
        };

        self.reg.write_flag(Flag::N, false);
        self.reg.write_flag(Flag::H, false);

        self.prefix_enabled = false;

        InstructionInfo(1, cycles)
    }

    pub fn rrc_r8(&mut self, opcode: u8) -> InstructionInfo {
        let r8_bit_field = opcode & 0x07;

        let cycles = if r8_bit_field == 0x06 {
            let addr = self.reg.dread(Reg16::HL);
            let b = self.mem.read(addr);

            let b0 = b & 0x01;

            let result = (b0 << 7) | (b >> 1);
            let z = result == 0;
            let cy = b0 == 0x01;

            self.mem.write(addr, result);

            self.reg.write_flag(Flag::Z, z);
            self.reg.write_flag(Flag::CY, cy);

            4

        } else {
            let target_reg = map_r8(r8_bit_field);
            let b = self.reg.read(target_reg);

            let b0 = b & 0x01;

            let result = (b0 << 7) | (b >> 1);
            let z = result == 0;
            let cy = b0 == 0x01;

            self.reg.write(target_reg, result);

            self.reg.write_flag(Flag::Z, z);
            self.reg.write_flag(Flag::CY, cy);

            2
        };

        self.reg.write_flag(Flag::N, false);
        self.reg.write_flag(Flag::H, false);

        self.prefix_enabled = false;

        InstructionInfo(1, cycles)
    }

    pub fn rl_r8(&mut self, opcode: u8) -> InstructionInfo {
        let r8_bit_field = opcode & 0x07;

        let mut cy = if self.reg.read_flag(Flag::CY) {
            1u8
        } else {
            0u8
        };


        let cycles = if r8_bit_field == 0x06 {
            let addr = self.reg.dread(Reg16::HL);
            let b = self.mem.read(addr);
            
            let result = (b << 1) | cy;
            let z = result == 0;
            cy = b >> 7;

            self.mem.write(addr, result);

            self.reg.write_flag(Flag::Z, z);

            4

        } else {
            let target_reg = map_r8(r8_bit_field);
            let b = self.reg.read(target_reg);

            let result = (b << 1) | cy;
            let z = result == 0;
            cy = b >> 7;

            self.reg.write(target_reg, result);

            self.reg.write_flag(Flag::Z, z);

            2
        };

        self.reg.write_flag(Flag::N, false);
        self.reg.write_flag(Flag::H, false);
        self.reg.write_flag(Flag::CY, cy == 0x01);

        self.prefix_enabled = false;

        InstructionInfo(1, cycles)
    }

    pub fn rr_r8(&mut self, opcode: u8) -> InstructionInfo {
        let r8_bit_field = opcode & 0x07;

        let mut cy = if self.reg.read_flag(Flag::CY) {
            0x80u8
        } else {
            0u8
        };


        let cycles = if r8_bit_field == 0x06 {
            let addr = self.reg.dread(Reg16::HL);
            let b = self.mem.read(addr);
            
            let result = cy | (b >> 1);
            let z = result == 0;
            cy = b & 0x01;

            self.mem.write(addr, result);

            self.reg.write_flag(Flag::Z, z);

            4

        } else {
            let target_reg = map_r8(r8_bit_field);
            let b = self.reg.read(target_reg);

            let result = cy | (b >> 1);
            let z = result == 0;
            cy = b & 0x01;

            self.reg.write(target_reg, result);

            self.reg.write_flag(Flag::Z, z);

            2
        };

        self.reg.write_flag(Flag::N, false);
        self.reg.write_flag(Flag::H, false);
        self.reg.write_flag(Flag::CY, cy == 0x01);

        self.prefix_enabled = false;

        InstructionInfo(1, cycles)
    }

    pub fn sla_r8(&mut self, opcode: u8) -> InstructionInfo {
        let bit_field = opcode & 0x07;

        let (z, cy, cycles) = if bit_field == 0x06 {
            let addr = self.reg.dread(Reg16::HL);
            let b = self.mem.read(addr);

            let result = b << 1;

            let z = result == 0;
            let cy = (b >> 7) == 0x01;

            self.mem.write(addr, result);

            (z, cy, 4)

        } else {
            let target_reg = map_r8(bit_field);
            let b = self.reg.read(target_reg);

            let result = b << 1;

            let z = result == 0;
            let cy = (b >> 7) == 0x01;

            self.reg.write(target_reg, result);

            (z, cy, 2)
        };

        self.reg.write_flag(Flag::Z, z);
        self.reg.write_flag(Flag::N, false);
        self.reg.write_flag(Flag::H, false);
        self.reg.write_flag(Flag::CY, cy);

        self.prefix_enabled = false;

        InstructionInfo(1, cycles)
    }

    pub fn sra_r8(&mut self, opcode: u8) -> InstructionInfo {
        let bit_field = opcode & 0x07;

        let (z, cy, cycles) = if bit_field == 0x06 {
            let addr = self.reg.dread(Reg16::HL);
            let b = self.mem.read(addr);

            let result = (b & 0x80) | (b >> 1);

            let z = result == 0;
            let cy = (b & 0x01) == 0x01;

            self.mem.write(addr, result);

            (z, cy, 4)

        } else {
            let target_reg = map_r8(bit_field);
            let b = self.reg.read(target_reg);

            let result = (b & 0x80) | (b >> 1);

            let z = result == 0;
            let cy = (b & 0x01) == 0x01;

            self.reg.write(target_reg, result);

            (z, cy, 2)
        };

        self.reg.write_flag(Flag::Z, z);
        self.reg.write_flag(Flag::N, false);
        self.reg.write_flag(Flag::H, false);
        self.reg.write_flag(Flag::CY, cy);

        self.prefix_enabled = false;

        InstructionInfo(1, cycles)
    }

    pub fn swap_r8(&mut self, opcode: u8) -> InstructionInfo {
        let bit_field = opcode & 0x07;

        let (z, cycles) = if bit_field == 0x06 {
            let addr = self.reg.dread(Reg16::HL);
            let value = self.mem.read(addr);

            let lsb = value & 0x0F;
            let msb = value & 0xF0;

            let result = (lsb << 4) | (msb >> 4);
            let z = result == 0;

            self.mem.write(addr, result);

            (z, 4)

        } else {
            let target_reg = map_r8(bit_field);
            let value = self.reg.read(target_reg);

            let lsb = value & 0x0F;
            let msb = value & 0xF0;

            let result = (lsb << 4) | (msb >> 4);
            let z = result == 0;

            self.reg.write(target_reg, result);

            (z, 2)
        };

        self.reg.write_flag(Flag::Z, z);
        self.reg.write_flag(Flag::N, false);
        self.reg.write_flag(Flag::H, false);
        self.reg.write_flag(Flag::CY, false);

        self.prefix_enabled = false;

        InstructionInfo(1, cycles)
    }

    pub fn srl_r8(&mut self, opcode: u8) -> InstructionInfo {
        let bit_field = opcode & 0x07;

        let (z, cy, cycles) = if bit_field == 0x06 {
            let addr = self.reg.dread(Reg16::HL);
            let b = self.mem.read(addr);

            let result = b >> 1;

            let z = result == 0;
            let cy = (b & 0x01) == 0x01;

            self.mem.write(addr, result);

            (z, cy, 4)

        } else {
            let target_reg = map_r8(bit_field);
            let b = self.reg.read(target_reg);

            let result = b >> 1;

            let z = result == 0;
            let cy = (b & 0x01) == 0x01;

            self.reg.write(target_reg, result);

            (z, cy, 2)
        };

        self.reg.write_flag(Flag::Z, z);
        self.reg.write_flag(Flag::N, false);
        self.reg.write_flag(Flag::H, false);
        self.reg.write_flag(Flag::CY, cy);

        self.prefix_enabled = false;

        InstructionInfo(1, cycles)
    }

    pub fn bit_b3_r8(&mut self, opcode: u8) -> InstructionInfo {
        let bit_index = (opcode >> 3) & 0x07;
        let operand = opcode & 0x07;

        let (value, cycles) = if operand == 0x06 {
            let addr = self.reg.dread(Reg16::HL);

            (self.mem.read(addr), 3)

        } else {
            let target_reg = map_r8(operand);

            (self.reg.read(target_reg), 2)
        };

        let z = (value >> bit_index) & 0x01 == 0x01;

        self.reg.write_flag(Flag::Z, z);
        self.reg.write_flag(Flag::N, false);
        self.reg.write_flag(Flag::H, true);

        self.prefix_enabled = false;

        InstructionInfo(1, cycles)
    }

    pub fn res_b3_r8(&mut self, opcode: u8) -> InstructionInfo {
        let bit_index = (opcode >> 3) & 0x07;
        let operand = opcode & 0x07;

        let cycles = if operand == 0x06 {
            let addr = self.reg.dread(Reg16::HL);
            let mut value = self.mem.read(addr);

            value &= !(1u8 << bit_index);

            self.mem.write(addr, value);

            4

        } else {
            let target_reg = map_r8(operand);
            let mut value = self.reg.read(target_reg);

            value &= !(1u8 << bit_index);

            self.reg.write(target_reg, value);

            2
        };

        self.prefix_enabled = false;

        InstructionInfo(1, cycles)
    }

    pub fn set_b3_r8(&mut self, opcode: u8) -> InstructionInfo {
        let bit_index = (opcode >> 3) & 0x07;
        let operand = opcode & 0x07;

        let cycles = if operand == 0x06 {
            let addr = self.reg.dread(Reg16::HL);
            let mut value = self.mem.read(addr);

            value |= 1u8 << bit_index;

            self.mem.write(addr, value);

            4

        } else {
            let target_reg = map_r8(operand);
            let mut value = self.reg.read(target_reg);

            value |= !1u8 << bit_index;

            self.reg.write(target_reg, value);

            2
        };

        self.prefix_enabled = false;

        InstructionInfo(1, cycles)
    }

}

