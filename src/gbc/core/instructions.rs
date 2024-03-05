use super::Core;
use super::register_file::{SingleReg, DoubleReg};

pub struct InstructionInfo(u8, u8);

impl Core {

    fn ld_r8_r8(&self, opcode: u8) -> InstructionInfo {
        let src_reg = match opcode & 0x07 {
            0x07 => SingleReg::A,
            0x00 => SingleReg::B,
            0x01 => SingleReg::C,
            0x02 => SingleReg::D,
            0x03 => SingleReg::E,
            0x04 => SingleReg::H,
            0x05 => SingleReg::L,
            _ => panic!("Error src_reg ld_r8_r8")
        };

        let dst_reg = match (opcode >> 3) & 0x07 {
            0x07 => SingleReg::A,
            0x00 => SingleReg::B,
            0x01 => SingleReg::C,
            0x02 => SingleReg::D,
            0x03 => SingleReg::E,
            0x04 => SingleReg::H,
            0x05 => SingleReg::L,
            _ => panic!("Error dst_reg ld_r8_r8")
        };

        let value = self.reg.get_single_reg(src_reg);

        self.reg.set_single_reg(dst_reg, value);

        InstructionInfo(1, 4)
    }

    fn ld_r8_n8(&self, opcode: u8) -> InstructionInfo {
        let dst_reg = match (opcode >> 3) & 0x07 {
            0x07 => SingleReg::A,
            0x00 => SingleReg::B,
            0x01 => SingleReg::C,
            0x02 => SingleReg::D,
            0x03 => SingleReg::E,
            0x04 => SingleReg::H,
            0x05 => SingleReg::L,
            _ => panic!("Error dst_reg ld_r8_n8")
        };

        let value = self.mem.get_at(self.pc + 1);

        self.reg.set_single_reg(dst_reg, value);

        InstructionInfo(2, 8)
    }

    fn ld_r8_hl(&self, opcode: u8) -> InstructionInfo {
        let dst_reg = match (opcode >> 3) & 0x07 {
            0x07 => SingleReg::A,
            0x00 => SingleReg::B,
            0x01 => SingleReg::C,
            0x02 => SingleReg::D,
            0x03 => SingleReg::E,
            0x04 => SingleReg::H,
            0x05 => SingleReg::L,
            _ => panic!("Error dst_reg ld_r8_hl")
        };

        let addr = self.reg.get_double_reg(DoubleReg::HL);
        let value = self.mem.get_at(addr);

        self.reg.set_single_reg(dst_reg, value);

        InstructionInfo(1, 8)
    }

    fn ld_hl_r8(&self, opcode: u8) -> InstructionInfo {
        let src_reg = match (opcode >> 3) & 0x07 {
            0x07 => SingleReg::A,
            0x00 => SingleReg::B,
            0x01 => SingleReg::C,
            0x02 => SingleReg::D,
            0x03 => SingleReg::E,
            0x04 => SingleReg::H,
            0x05 => SingleReg::L,
            _ => panic!("Error src_reg ld_hl_r8")
        };

        let value = self.reg.get_single_reg(src_reg);
        let addr = self.reg.get_double_reg(DoubleReg::HL);

        self.mem.set_at(addr, value);

        InstructionInfo(1, 8)
    }

    fn ld_hl_n8(&self) -> InstructionInfo {
        let value = self.mem.get_at(self.pc + 1);
        let addr = self.reg.get_double_reg(DoubleReg::HL);

        self.mem.set_at(addr, value);

        InstructionInfo(2, 12)
    }

}
