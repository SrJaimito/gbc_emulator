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

        let value = self.reg.get_single_reg(src_reg);

        self.reg.set_single_reg(dst_reg, value);

        InstructionInfo(1, 4)
    }

    fn ld_r8_n8(&mut self, opcode: u8) -> InstructionInfo {
        let dst_reg = match map_3bit_field((opcode >> 3) & 0x07) {
            Some(reg) => reg,
            None => panic!("Error dst_reg ld_r8_n8")
        };

        let value = self.mem.read(self.pc + 1);

        self.reg.set_single_reg(dst_reg, value);

        InstructionInfo(2, 8)
    }

    fn ld_r8_hl(&mut self, opcode: u8) -> InstructionInfo {
        let dst_reg = match map_3bit_field((opcode >> 3) & 0x07) {
            Some(reg) => reg,
            None => panic!("Error dst_reg ld_r8_hl")
        };

        let addr = self.reg.get_double_reg(DoubleReg::HL);
        let value = self.mem.read(addr);

        self.reg.set_single_reg(dst_reg, value);

        InstructionInfo(1, 8)
    }

    fn ld_hl_r8(&mut self, opcode: u8) -> InstructionInfo {
        let src_reg = match map_3bit_field((opcode >> 3) & 0x07) {
            Some(reg) => reg,
            None => panic!("Error src_reg ld_hl_r8")
        };

        let value = self.reg.get_single_reg(src_reg);
        let addr = self.reg.get_double_reg(DoubleReg::HL);

        self.mem.write(addr, value);

        InstructionInfo(1, 8)
    }

    fn ld_hl_n8(&mut self) -> InstructionInfo {
        let value = self.mem.read(self.pc + 1);
        let addr = self.reg.get_double_reg(DoubleReg::HL);

        self.mem.write(addr, value);

        InstructionInfo(2, 12)
    }

    fn ld_a_bc(&mut self) -> InstructionInfo {
        let addr = self.reg.get_double_reg(DoubleReg::BC);
        let value = self.mem.read(addr);

        self.reg.set_single_reg(SingleReg::A, value);

        InstructionInfo(1, 8)
    }

    fn ld_a_de(&mut self) -> InstructionInfo {
        let addr = self.reg.get_double_reg(DoubleReg::DE);
        let value = self.mem.read(addr);

        self.reg.set_single_reg(SingleReg::A, value);

        InstructionInfo(1, 8)
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn ld_r8_r8() {
        let mut core = Core::new();
        core.reg.set_single_reg(SingleReg::A, 0x0A);
        core.reg.set_single_reg(SingleReg::B, 0x0B);

        let opcode = 0b01111000;

        core.ld_r8_r8(opcode);

        assert_eq!(core.reg.get_single_reg(SingleReg::A), 0x0B);
    }

    #[test]
    fn ld_r8_n8() {
        let mut core = Core::new();
        core.reg.set_single_reg(SingleReg::C, 0x0C);
        core.pc = 0x100;
        core.mem.write(core.pc, 0x0A);
        core.mem.write(core.pc + 1, 0x0B);

        let opcode = 0x0E;

        core.ld_r8_n8(opcode);

        assert_eq!(core.reg.get_single_reg(SingleReg::C), 0x0B);
    }

    #[test]
    fn ld_r8_hl() {
    }

    #[test]
    fn ld_hl_r8() {
    }

    #[test]
    fn ld_hl_n8() {
    }
}

