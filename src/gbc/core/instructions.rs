use super::Core;
use super::register_file::{Reg8, Reg16, Flag};

pub struct InstructionInfo(u8, u8);

impl Core {

    pub fn nop(&self) -> InstructionInfo {
        unimplemented!()
    }

    pub fn ld_r16_imm16(&self, opcode: u8) -> InstructionInfo {
        unimplemented!()
    }

    pub fn ld_r16mem_a(&self, opcode: u8) -> InstructionInfo {
        unimplemented!()
    }

    pub fn ld_a_r16mem(&self, opcode: u8) -> InstructionInfo {
        unimplemented!()
    }

    pub fn ld_imm16_sp(&self) -> InstructionInfo {
        unimplemented!()
    }

    pub fn inc_r16(&self, opcode: u8) -> InstructionInfo {
        unimplemented!()
    }

    pub fn dec_r16(&self, opcode: u8) -> InstructionInfo {
        unimplemented!()
    }

    pub fn add_hl_r16(&self, opcode: u8) -> InstructionInfo {
        unimplemented!()
    }

    pub fn inc_r8(&self, opcode: u8) -> InstructionInfo {
        unimplemented!()
    }

    pub fn dec_r8(&self, opcode: u8) -> InstructionInfo {
        unimplemented!()
    }

    pub fn ld_r8_imm8(&self, opcode: u8) -> InstructionInfo {
        unimplemented!()
    }

    pub fn rlca(&self) -> InstructionInfo {
        unimplemented!()
    }

    pub fn rrca(&self) -> InstructionInfo {
        unimplemented!()
    }

    pub fn rla(&self) -> InstructionInfo {
        unimplemented!()
    }

    pub fn rra(&self) -> InstructionInfo {
        unimplemented!()
    }

    pub fn daa(&self) -> InstructionInfo {
        unimplemented!()
    }

    pub fn cpl(&self) -> InstructionInfo {
        unimplemented!()
    }

    pub fn scf(&self) -> InstructionInfo {
        unimplemented!()
    }

    pub fn ccf(&self) -> InstructionInfo {
        unimplemented!()
    }

    pub fn jr_imm8(&self) -> InstructionInfo {
        unimplemented!()
    }

    pub fn jr_cond_imm8(&self, opcode: u8) -> InstructionInfo {
        unimplemented!()
    }

    pub fn stop(&self) -> InstructionInfo {
        unimplemented!()
    }

    pub fn ld_r8_r8(&self, opcode: u8) -> InstructionInfo {
        unimplemented!()
    }

    pub fn halt(&self) -> InstructionInfo {
        unimplemented!()
    }

    pub fn add_a_r8(&self, opcode: u8) -> InstructionInfo {
        unimplemented!()
    }

    pub fn adc_a_r8(&self, opcode: u8) -> InstructionInfo {
        unimplemented!()
    }

    pub fn sub_a_r8(&self, opcode: u8) -> InstructionInfo {
        unimplemented!()
    }

    pub fn sbc_a_r8(&self, opcode: u8) -> InstructionInfo {
        unimplemented!()
    }

    pub fn and_a_r8(&self, opcode: u8) -> InstructionInfo {
        unimplemented!()
    }

    pub fn xor_a_r8(&self, opcode: u8) -> InstructionInfo {
        unimplemented!()
    }

    pub fn or_a_r8(&self, opcode: u8) -> InstructionInfo {
        unimplemented!()
    }

    pub fn cp_a_r8(&self, opcode: u8) -> InstructionInfo {
        unimplemented!()
    }

    pub fn add_a_imm8(&self) -> InstructionInfo {
        unimplemented!()
    }

    pub fn adc_a_imm8(&self) -> InstructionInfo {
        unimplemented!()
    }

    pub fn sub_a_imm8(&self) -> InstructionInfo {
        unimplemented!()
    }

    pub fn sbc_a_imm8(&self) -> InstructionInfo {
        unimplemented!()
    }

    pub fn and_a_imm8(&self) -> InstructionInfo {
        unimplemented!()
    }

    pub fn xor_a_imm8(&self) -> InstructionInfo {
        unimplemented!()
    }

    pub fn or_a_imm8(&self) -> InstructionInfo {
        unimplemented!()
    }

    pub fn cp_a_imm8(&self) -> InstructionInfo {
        unimplemented!()
    }

    pub fn ret_cond(&self, opcode: u8) -> InstructionInfo {
        unimplemented!()
    }

    pub fn ret(&self) -> InstructionInfo {
        unimplemented!()
    }

    pub fn reti(&self) -> InstructionInfo {
        unimplemented!()
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

    pub fn prefix(&self) -> InstructionInfo {
        unimplemented!()
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

    pub fn di(&self) -> InstructionInfo {
        unimplemented!()
    }

    pub fn ei(&self) -> InstructionInfo {
        unimplemented!()
    }

    pub fn rlc_r8(&self, opcode: u8) {
        unimplemented!()
    }

    pub fn rrc_r8(&self, opcode: u8) {
        unimplemented!()
    }

    pub fn rl_r8(&self, opcode: u8) {
        unimplemented!()
    }

    pub fn rr_r8(&self, opcode: u8) {
        unimplemented!()
    }

    pub fn sla_r8(&self, opcode: u8) {
        unimplemented!()
    }

    pub fn sra_r8(&self, opcode: u8) {
        unimplemented!()
    }

    pub fn swap_r8(&self, opcode: u8) {
        unimplemented!()
    }

    pub fn srl_r8(&self, opcode: u8) {
        unimplemented!()
    }

    pub fn bit_b3_r8(&self, opcode: u8) {
        unimplemented!()
    }

    pub fn res_b3_r8(&self, opcode: u8) {
        unimplemented!()
    }

    pub fn set_b3_r8(&self, opcode: u8) {
        unimplemented!()
    }

}

