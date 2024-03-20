use super::Core;
use super::register_file::{Reg8, Reg16, Flag, map_r8, map_r16stk};

pub struct InstructionInfo(u8, u8); // (size, cycles)

impl Core {

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
        let pc = self.pc as i32;
        let imm = self.mem.read(self.pc + 1) as i32;

        let cycles = match (opcode >> 3) & 0x03 {
            0x00 => {
                if !self.reg.read_flag(Flag::Z) {
                    self.pc = (pc + imm) as u16;
                    3
                } else {
                    2
                }
            },
            0x01 => {
                if self.reg.read_flag(Flag::Z) {
                    self.pc = (pc + imm) as u16;
                    3
                } else {
                    2
                }
            },
            0x02 => {
                if !self.reg.read_flag(Flag::CY) {
                    self.pc = (pc + imm) as u16;
                    3
                } else {
                    2
                }
            },
            0x03 => {
                if self.reg.read_flag(Flag::CY) {
                    self.pc = (pc + imm) as u16;
                    3
                } else {
                    2
                }
            },
            _ => panic!("Error jr_cond_imm8")
        };

        InstructionInfo(2, cycles)
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

    pub fn jp_cond_imm16(&self, opcode: u8) -> InstructionInfo {
        unimplemented!()
    }

    pub fn jp_imm16(&self) -> InstructionInfo {
        unimplemented!()
    }

    pub fn jp_hl(&self) -> InstructionInfo {
        unimplemented!()
    }

    pub fn call_cond_imm16(&self, opcode: u8) -> InstructionInfo {
        unimplemented!()
    }

    pub fn call_imm16(&self) -> InstructionInfo {
        unimplemented!()
    }

    pub fn rst_tgt3(&self, opcode: u8) -> InstructionInfo {
        unimplemented!()
    }

    pub fn pop_r16stk(&self, opcode: u8) -> InstructionInfo {
        unimplemented!()
    }

    pub fn push_r16stk(&self, opcode: u8) -> InstructionInfo {
        unimplemented!()
    }

    pub fn prefix(&mut self) -> InstructionInfo {
        self.prefix_enabled = true;

        InstructionInfo(1, 1)
    }

    pub fn ldh_c_a(&self) -> InstructionInfo {
        unimplemented!()
    }

    pub fn ldh_imm8_a(&self) -> InstructionInfo {
        unimplemented!()
    }

    pub fn ld_imm16_a(&self) -> InstructionInfo {
        unimplemented!()
    }

    pub fn ldh_a_c(&self) -> InstructionInfo {
        unimplemented!()
    }

    pub fn ldh_a_imm8(&self) -> InstructionInfo {
        unimplemented!()
    }

    pub fn ld_a_imm16(&self) -> InstructionInfo {
        unimplemented!()
    }

    pub fn add_sp_imm8(&self) -> InstructionInfo {
        unimplemented!()
    }

    pub fn ld_hl_sp_imm8(&self) -> InstructionInfo {
        unimplemented!()
    }

    pub fn ld_sp_hl(&self) -> InstructionInfo {
        unimplemented!()
    }

    pub fn di(&mut self) -> InstructionInfo {
        self.ime_enabled = false;
        self.ime_enable_request = 0;

        InstructionInfo(1, 1)
    }

    pub fn ei(&mut self) -> InstructionInfo {
        // IME is set after the instruction following this one
        self.ime_enable_request = 2;

        InstructionInfo(1, 1)
    }

    pub fn rlc_r8(&self, opcode: u8) -> InstructionInfo {
        unimplemented!()
    }

    pub fn rrc_r8(&self, opcode: u8) -> InstructionInfo {
        unimplemented!()
    }

    pub fn rl_r8(&self, opcode: u8) -> InstructionInfo {
        unimplemented!()
    }

    pub fn rr_r8(&self, opcode: u8) -> InstructionInfo {
        unimplemented!()
    }

    pub fn sla_r8(&self, opcode: u8) -> InstructionInfo {
        unimplemented!()
    }

    pub fn sra_r8(&self, opcode: u8) -> InstructionInfo {
        unimplemented!()
    }

    pub fn swap_r8(&self, opcode: u8) -> InstructionInfo {
        unimplemented!()
    }

    pub fn srl_r8(&self, opcode: u8) -> InstructionInfo {
        unimplemented!()
    }

    pub fn bit_b3_r8(&self, opcode: u8) -> InstructionInfo {
        unimplemented!()
    }

    pub fn res_b3_r8(&self, opcode: u8) -> InstructionInfo {
        unimplemented!()
    }

    pub fn set_b3_r8(&self, opcode: u8) -> InstructionInfo {
        unimplemented!()
    }

}

