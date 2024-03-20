mod register_file;
mod instructions;
mod memory;

use register_file::RegisterFile;
use memory::Memory;
use instructions::InstructionInfo;

struct Core {
    reg: RegisterFile,

    pc: u16,
    sp: u16,

    prefix_enabled: bool,

    ime_enabled: bool,
    ime_enable_request: u8,

    mem: Memory
}

impl Core {

    pub fn new() -> Self {
        Self {
            reg: RegisterFile::new(),
            pc: 0,
            sp: 0,
            prefix_enabled: false,
            ime_enabled: true,
            ime_enable_request: 0,
            mem: Memory::new()
        }
    }

    pub fn execute(&mut self, opcode: u8) -> InstructionInfo {
        if self.prefix_enabled {
            // Test bits 7-6
            match opcode >> 6 {
                0x00 => {
                    // Test bits 5-3
                    match (opcode >> 3) & 0x07 {
                        0x00 => self.rlc_r8(opcode),
                        0x01 => self.rrc_r8(opcode),
                        0x02 => self.rl_r8(opcode),
                        0x03 => self.rr_r8(opcode),
                        0x04 => self.sla_r8(opcode),
                        0x05 => self.sra_r8(opcode),
                        0x06 => self.swap_r8(opcode),
                        0x07 => self.srl_r8(opcode),
                        _ => panic!("Error decoding prefix instruction (1)")
                    }
                },

                0x01 => self.bit_b3_r8(opcode),
                0x02 => self.res_b3_r8(opcode),
                0x03 => self.set_b3_r8(opcode),
                _ => panic!("Error decoding prefix instruction (2)")
            }
        } else {
            // Match block (https://gbdev.io/pandocs/CPU_Instruction_set.html)
            match opcode >> 6 {
                // Block 0
                0x00 => {

                    // Test bits 2-0
                    match opcode & 0x07 {
                        0x00 => {
                            // Test bit 5
                            if ((opcode >> 5) & 0x01) == 0x00 {
                                // Test bits 4-3
                                match (opcode >> 3) & 0x03 {
                                    0x00 => self.nop(),
                                    0x01 => self.ld_imm16_sp(),
                                    0x02 => self.stop(),
                                    0x03 => self.jr_imm8(),
                                    _ => panic!("Error decoding instruction (1)")
                                }
                            } else {
                                self.jr_cond_imm8(opcode)
                            }
                        },
                        
                        0x01 => {
                            // Test bit 3
                            if (opcode >> 3) == 0x00 {
                                self.ld_r16_imm16(opcode)
                            } else {
                                self.add_hl_r16(opcode)
                            }
                        },

                        0x02 => {
                            // Test bit 3
                            if (opcode >> 3) == 0x00 {
                                self.ld_r16mem_a(opcode)
                            } else {
                                self.ld_a_r16mem(opcode)
                            }
                        },

                        0x03 => {
                            // Test bit 3
                            if (opcode >> 3) == 0x00 {
                                self.inc_r16(opcode)
                            } else {
                                self.dec_r16(opcode)
                            }
                        },

                        0x04 => self.inc_r8(opcode),

                        0x05 => self.dec_r8(opcode),

                        0x06 => self.ld_r8_imm8(opcode),

                        0x07 => {
                            // Test bits 5-3
                            match (opcode >> 3) & 0x07 {
                                0x00 => self.rlca(),
                                0x01 => self.rrca(),
                                0x02 => self.rla(),
                                0x03 => self.rra(),
                                0x04 => self.daa(),
                                0x05 => self.cpl(),
                                0x06 => self.scf(),
                                0x07 => self.ccf(),
                                _ => panic!("Error decoding instruction (2)")
                            }
                        },

                        _ => panic!("Error decoding instruction (3)")
                    }

                },

                // Block 1
                0x01 => {
                    if opcode == 0x76 {
                        self.halt()
                    } else {
                        self.ld_r8_r8(opcode)
                    }
                },

                // Block 2
                0x02 => {
                    // Test bits 5-3
                    match (opcode >> 3) & 0x07 {
                        0x00 => self.add_a_r8(opcode),
                        0x01 => self.adc_a_r8(opcode),
                        0x02 => self.sub_a_r8(opcode),
                        0x03 => self.sbc_a_r8(opcode),
                        0x04 => self.and_a_r8(opcode),
                        0x05 => self.xor_a_r8(opcode),
                        0x06 => self.or_a_r8(opcode),
                        0x07 => self.cp_a_r8(opcode),
                        _ => panic!("Error decoding instruction (4)")
                    }
                },

                // Block 3
                0x03 => {
                    // Test bits 2-0
                    match opcode & 0x07 {
                        0x00 => {
                            // Test bit 5
                            if (opcode >> 5) & 0x01 == 0x00 {
                                self.ret_cond(opcode)
                            } else {
                                // Test bits 4-3
                                match (opcode >> 3) & 0x3 {
                                    0x00 => self.ldh_imm8_a(),
                                    0x01 => self.add_sp_imm8(),
                                    0x02 => self.ldh_a_imm8(),
                                    0x03 => self.ld_hl_sp_imm8(),
                                    _ => panic!("Error decoding instruction (5)")
                                }
                            }
                        },

                        0x01 => {
                            // Test bit 3
                            if (opcode >> 3) & 0x01 == 0x00 {
                                self.pop_r16stk(opcode)
                            } else {
                                // Test bits 5-4
                                match (opcode >> 4) & 0x03 {
                                    0x00 => self.ret(),
                                    0x01 => self.reti(),
                                    0x02 => self.jp_hl(),
                                    0x03 => self.ld_sp_hl(),
                                    _ => panic!("Error decoding instruction (6)")
                                }
                            }
                        },

                        0x02 => {
                            // Test bit 5
                            if (opcode >> 5) & 0x01 == 0x00 {
                                self.jp_cond_imm16(opcode)
                            } else {
                                // Test bits 4-3
                                match (opcode >> 3) & 0x03 {
                                    0x00 => self.ldh_c_a(),
                                    0x01 => self.ld_imm16_a(),
                                    0x02 => self.ldh_a_c(),
                                    0x03 => self.ld_a_imm16(),
                                    _ => panic!("Error decoding instruction (7)")
                                }
                            }
                        },

                        0x03 => {
                            match opcode {
                                0xC3 => self.jp_imm16(),
                                0xCB => self.prefix(),
                                0xF3 => self.di(),
                                0xFB => self.ei(),
                                _ => panic!("Error decoding instruction (8)")
                            }
                        },

                        0x04 => self.call_cond_imm16(opcode),

                        0x05 => {
                           // Test bit 3
                           if (opcode >> 3) & 0x01 == 0x00 {
                               self.push_r16stk(opcode)
                           } else {
                               self.call_imm16()
                           }
                        },

                        0x06 => {
                            // Test bits 5-3
                            match (opcode >> 3) & 0x07 {
                                0x00 => self.add_a_imm8(),
                                0x01 => self.adc_a_imm8(),
                                0x02 => self.sub_a_imm8(),
                                0x03 => self.sbc_a_imm8(),
                                0x04 => self.and_a_imm8(),
                                0x05 => self.xor_a_imm8(),
                                0x06 => self.or_a_imm8(),
                                0x07 => self.cp_a_imm8(),
                                _ => panic!("Error decoding instruction (9)")
                            }
                        },

                        0x07 => self.rst_tgt3(opcode),

                        _ => panic!("Error decoding instruction (10)")
                   }
                },

                _ => panic!("Error matching opcode block")
            }
        }
    }

    pub fn update_ime(&mut self) {
        // Update IME if needed
        if self.ime_enable_request != 0 {
            self.ime_enable_request -= 1;

            if self.ime_enable_request == 0 {
                self.ime_enabled = true;
            }
        }
    }

}

