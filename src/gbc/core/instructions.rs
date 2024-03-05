use super::Core;
use super::register_file::{SingleReg, DoubleReg, map_3bit_field};

pub struct InstructionInfo(u8, u8);

impl Core {

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

}

