use super::ram::RAM;

struct CPU {
    a: u8,
    f: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16,
    time: u64,

    ram: RAM,
}

impl CPU {
    fn new(ram: RAM) -> Self {
        Self {
            a: 0,
            f: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            sp: 0,
            pc: 0,
            time: 0,

            ram,
        }
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Instruction8 { opcode, operands } => {
                match opcode {
                    0x00 => todo!(),
                    0x01 => self.ld_bc_d16(operands),
                    0x02 => self.ld_bc_a(operands),
                    0x03 => self.inc_bc(operands),
                    0x04 => self.inc_b(operands),
                    0x05 => self.dec_b(operands),
                    0x06 => self.ld_b_d8(operands),
                    0x07 => self.rlca(operands),
                    0x08 => self.ld_a16_sp(operands),
                    0x09 => self.add_hl_bc(operands),
                    0x0a => self.ld_a_bc(operands),
                    0x0b => self.dec_bc(operands),
                    0x0c => self.inc_c(operands),
                    0x0d => self.dec_c(operands),
                    0x0e => self.ld_c_d8(operands),
                    0x0f => self.rrca(operands),
                    0x10 => todo!(),
                    0x11 => self.ld_de_d16(operands),
                    0x12 => self.ld_de_a(operands),
                    0x13 => self.inc_de(operands),
                    0x14 => self.inc_d(operands),
                    0x15 => self.dec_d(operands),
                    0x16 => self.ld_d_d8(operands),
                    0x17 => self.rla(operands),
                    0x18 => todo!(),
                    0x19 => self.add_hl_de(operands),
                    0x1a => self.ld_a_de(operands),
                    0x1b => self.dec_de(operands),
                    0x1c => self.inc_e(operands),
                    0x1d => self.dec_e(operands),
                    0x1e => self.ld_e_d8(operands),
                    0x1f => self.rra(operands),
                    0x20 => todo!(),
                    0x21 => self.ld_hl_d16(operands),
                    0x22 => self.ld_hl_plus_a(operands),
                    0x23 => self.inc_hl_reg(operands),
                    0x24 => self.inc_h(operands),
                    0x25 => self.dec_h(operands),
                    0x26 => self.ld_h_d8(operands),
                    0x27 => todo!(),
                    0x28 => todo!(),
                    0x29 => self.add_hl_hl(operands),
                    0x2a => self.ld_a_hl_plus(operands),
                    0x2b => self.dec_hl_reg(operands),
                    0x2c => self.inc_l(operands),
                    0x2d => self.dec_l(operands),
                    0x2e => self.ld_l_d8(operands),
                    0x2f => self.cpl(operands),
                    0x30 => todo!(),
                    0x31 => self.ld_sp_d16(operands),
                    0x32 => self.ld_hl_minus_a(operands),
                    0x33 => self.inc_sp(operands),
                    0x34 => self.inc_hl(operands),
                    0x35 => self.dec_hl(operands),
                    0x36 => self.ld_hl_d8(operands),
                    0x37 => self.scf(operands),
                    0x38 => todo!(),
                    0x39 => self.add_hl_sp(operands),
                    0x3a => self.ld_a_hl_minus(operands),
                    0x3b => self.dec_sp(operands),
                    0x3c => self.inc_a(operands),
                    0x3d => self.dec_a(operands),
                    0x3e => self.ld_a_d8(operands),
                    0x3f => self.ccf(operands),
                    0x40 => self.ld_b_b(operands),
                    0x41 => self.ld_b_c(operands),
                    0x42 => self.ld_b_d(operands),
                    0x43 => self.ld_b_e(operands),
                    0x44 => self.ld_b_h(operands),
                    0x45 => self.ld_b_l(operands),
                    0x46 => self.ld_b_hl(operands),
                    0x47 => self.ld_b_a(operands),
                    0x48 => self.ld_c_b(operands),
                    0x49 => self.ld_c_c(operands),
                    0x4a => self.ld_c_d(operands),
                    0x4b => self.ld_c_e(operands),
                    0x4c => self.ld_c_h(operands),
                    0x4d => self.ld_c_l(operands),
                    0x4e => self.ld_c_hl(operands),
                    0x4f => self.ld_c_a(operands),
                    0x50 => self.ld_d_b(operands),
                    0x51 => self.ld_d_c(operands),
                    0x52 => self.ld_d_d(operands),
                    0x53 => self.ld_d_e(operands),
                    0x54 => self.ld_d_h(operands),
                    0x55 => self.ld_d_l(operands),
                    0x56 => self.ld_d_hl(operands),
                    0x57 => self.ld_d_a(operands),
                    0x58 => self.ld_e_b(operands),
                    0x59 => self.ld_e_c(operands),
                    0x5a => self.ld_e_d(operands),
                    0x5b => self.ld_e_e(operands),
                    0x5c => self.ld_e_h(operands),
                    0x5d => self.ld_e_l(operands),
                    0x5e => self.ld_e_hl(operands),
                    0x5f => self.ld_e_a(operands),
                    0x60 => self.ld_h_b(operands),
                    0x61 => self.ld_h_c(operands),
                    0x62 => self.ld_h_d(operands),
                    0x63 => self.ld_h_e(operands),
                    0x64 => self.ld_h_h(operands),
                    0x65 => self.ld_h_l(operands),
                    0x66 => self.ld_h_hl(operands),
                    0x67 => self.ld_h_a(operands),
                    0x68 => self.ld_l_b(operands),
                    0x69 => self.ld_l_c(operands),
                    0x6a => self.ld_l_d(operands),
                    0x6b => self.ld_l_e(operands),
                    0x6c => self.ld_l_h(operands),
                    0x6d => self.ld_l_l(operands),
                    0x6e => self.ld_l_hl(operands),
                    0x6f => self.ld_l_a(operands),
                    0x70 => self.ld_hl_b(operands),
                    0x71 => self.ld_hl_c(operands),
                    0x72 => self.ld_hl_d(operands),
                    0x73 => self.ld_hl_e(operands),
                    0x74 => self.ld_hl_h(operands),
                    0x75 => self.ld_hl_l(operands),
                    0x76 => todo!(),
                    0x77 => self.ld_hl_a(operands),
                    0x78 => self.ld_a_b(operands),
                    0x79 => self.ld_a_c(operands),
                    0x7a => self.ld_a_d(operands),
                    0x7b => self.ld_a_e(operands),
                    0x7c => self.ld_a_h(operands),
                    0x7d => self.ld_a_l(operands),
                    0x7e => self.ld_a_hl(operands),
                    0x7f => self.ld_a_a(operands),
                    0x80 => self.add_a_b(operands),
                    0x81 => self.add_a_c(operands),
                    0x82 => self.add_a_d(operands),
                    0x83 => self.add_a_e(operands),
                    0x84 => self.add_a_h(operands),
                    0x85 => self.add_a_l(operands),
                    0x86 => self.add_a_hl(operands),
                    0x87 => self.add_a_a(operands),
                    0x88 => self.adc_a_b(operands),
                    0x89 => self.adc_a_c(operands),
                    0x8a => self.adc_a_d(operands),
                    0x8b => self.adc_a_e(operands),
                    0x8c => self.adc_a_h(operands),
                    0x8d => self.adc_a_l(operands),
                    0x8e => self.adc_a_hl(operands),
                    0x8f => self.adc_a_a(operands),
                    0x90 => self.sub_b(operands),
                    0x91 => self.sub_c(operands),
                    0x92 => self.sub_d(operands),
                    0x93 => self.sub_e(operands),
                    0x94 => self.sub_h(operands),
                    0x95 => self.sub_l(operands),
                    0x96 => self.sub_hl(operands),
                    0x97 => self.sub_a(operands),
                    0x98 => self.sbc_a_b(operands),
                    0x99 => self.sbc_a_c(operands),
                    0x9a => self.sbc_a_d(operands),
                    0x9b => self.sbc_a_e(operands),
                    0x9c => self.sbc_a_h(operands),
                    0x9d => self.sbc_a_l(operands),
                    0x9e => self.sbc_a_hl(operands),
                    0x9f => self.sbc_a_a(operands),
                    0xa0 => self.and_b(operands),
                    0xa1 => self.and_c(operands),
                    0xa2 => self.and_d(operands),
                    0xa3 => self.and_e(operands),
                    0xa4 => self.and_h(operands),
                    0xa5 => self.and_l(operands),
                    0xa6 => self.and_hl(operands),
                    0xa7 => self.and_a(operands),
                    0xa8 => self.xor_b(operands),
                    0xa9 => self.xor_c(operands),
                    0xaa => self.xor_d(operands),
                    0xab => self.xor_e(operands),
                    0xac => self.xor_h(operands),
                    0xad => self.xor_l(operands),
                    0xae => self.xor_hl(operands),
                    0xaf => self.xor_a(operands),
                    0xb0 => self.or_b(operands),
                    0xb1 => self.or_c(operands),
                    0xb2 => self.or_d(operands),
                    0xb3 => self.or_e(operands),
                    0xb4 => self.or_h(operands),
                    0xb5 => self.or_l(operands),
                    0xb6 => self.or_hl(operands),
                    0xb7 => self.or_a(operands),
                    0xb8 => self.cp_b(operands),
                    0xb9 => self.cp_c(operands),
                    0xba => self.cp_d(operands),
                    0xbb => self.cp_e(operands),
                    0xbc => self.cp_h(operands),
                    0xbd => self.cp_l(operands),
                    0xbe => self.cp_hl(operands),
                    0xbf => self.cp_a(operands),
                    0xc0 => todo!(),
                    0xc1 => self.pop_bc(operands),
                    0xc2 => todo!(),
                    0xc3 => todo!(),
                    0xc4 => todo!(),
                    0xc5 => self.push_bc(operands),
                    0xc6 => self.add_a_d8(operands),
                    0xc7 => todo!(),
                    0xc8 => todo!(),
                    0xc9 => todo!(),
                    0xca => todo!(),
                    0xcb => todo!(),
                    0xcc => todo!(),
                    0xcd => todo!(),
                    0xce => self.adc_a_d8(operands),
                    0xcf => todo!(),
                    0xd0 => todo!(),
                    0xd1 => self.pop_de(operands),
                    0xd2 => todo!(),
                    0xd3 => todo!(),
                    0xd4 => todo!(),
                    0xd5 => self.push_de(operands),
                    0xd6 => self.sub_d8(operands),
                    0xd7 => todo!(),
                    0xd8 => todo!(),
                    0xd9 => todo!(),
                    0xda => todo!(),
                    0xdb => todo!(),
                    0xdc => todo!(),
                    0xdd => todo!(),
                    0xde => self.sbc_a_d8(operands),
                    0xdf => todo!(),
                    0xe0 => self.ldh_a8_a(operands),
                    0xe1 => self.pop_hl(operands),
                    0xe2 => self.ldh_c_a(operands),
                    0xe3 => todo!(),
                    0xe4 => todo!(),
                    0xe5 => self.push_hl(operands),
                    0xe6 => self.and_d8(operands),
                    0xe7 => todo!(),
                    0xe8 => self.add_sp_s8(operands),
                    0xe9 => todo!(),
                    0xea => self.ld_a16_a(operands),
                    0xeb => todo!(),
                    0xec => todo!(),
                    0xed => todo!(),
                    0xee => self.xor_d8(operands),
                    0xef => todo!(),
                    0xf0 => self.ldh_a_a8(operands),
                    0xf1 => self.pop_af(operands),
                    0xf2 => self.ldh_a_c(operands),
                    0xf3 => todo!(),
                    0xf4 => todo!(),
                    0xf5 => self.push_af(operands),
                    0xf6 => self.or_d8(operands),
                    0xf7 => todo!(),
                    0xf8 => self.ld_hl_sp_plus_s8(operands),
                    0xf9 => self.ld_sp_hl(operands),
                    0xfa => self.ld_a_a16(operands),
                    0xfb => todo!(),
                    0xfc => todo!(),
                    0xfd => todo!(),
                    0xfe => self.cp_d8(operands),
                    0xff => todo!(),
                }
            },
            Instruction::Instruction16 { opcode, operands } => {
                match opcode {
                    0xcb00 => self.rlc_b(operands),
                    0xcb01 => self.rlc_c(operands),
                    0xcb02 => self.rlc_d(operands),
                    0xcb03 => self.rlc_e(operands),
                    0xcb04 => self.rlc_h(operands),
                    0xcb05 => self.rlc_l(operands),
                    0xcb06 => self.rlc_hl(operands),
                    0xcb07 => self.rlc_a(operands),
                    0xcb08 => self.rrc_b(operands),
                    0xcb09 => self.rrc_c(operands),
                    0xcb0a => self.rrc_d(operands),
                    0xcb0b => self.rrc_e(operands),
                    0xcb0c => self.rrc_h(operands),
                    0xcb0d => self.rrc_l(operands),
                    0xcb0e => self.rrc_hl(operands),
                    0xcb0f => self.rrc_a(operands),
                    0xcb10 => self.rl_b(operands),
                    0xcb11 => self.rl_c(operands),
                    0xcb12 => self.rl_d(operands),
                    0xcb13 => self.rl_e(operands),
                    0xcb14 => self.rl_h(operands),
                    0xcb15 => self.rl_l(operands),
                    0xcb16 => self.rl_hl(operands),
                    0xcb17 => self.rl_a(operands),
                    0xcb18 => self.rr_b(operands),
                    0xcb19 => self.rr_c(operands),
                    0xcb1a => self.rr_d(operands),
                    0xcb1b => self.rr_e(operands),
                    0xcb1c => self.rr_h(operands),
                    0xcb1d => self.rr_l(operands),
                    0xcb1e => self.rr_hl(operands),
                    0xcb1f => self.rr_a(operands),
                    0xcb20 => self.sla_b(operands),
                    0xcb21 => self.sla_c(operands),
                    0xcb22 => self.sla_d(operands),
                    0xcb23 => self.sla_e(operands),
                    0xcb24 => self.sla_h(operands),
                    0xcb25 => self.sla_l(operands),
                    0xcb26 => self.sla_hl(operands),
                    0xcb27 => self.sla_a(operands),
                    0xcb28 => self.sra_b(operands),
                    0xcb29 => self.sra_c(operands),
                    0xcb2a => self.sra_d(operands),
                    0xcb2b => self.sra_e(operands),
                    0xcb2c => self.sra_h(operands),
                    0xcb2d => self.sra_l(operands),
                    0xcb2e => self.sra_hl(operands),
                    0xcb2f => self.sra_a(operands),
                    0xcb30 => self.swap_b(operands),
                    0xcb31 => self.swap_c(operands),
                    0xcb32 => self.swap_d(operands),
                    0xcb33 => self.swap_e(operands),
                    0xcb34 => self.swap_h(operands),
                    0xcb35 => self.swap_l(operands),
                    0xcb36 => self.swap_hl(operands),
                    0xcb37 => self.swap_a(operands),
                    0xcb38 => self.srl_b(operands),
                    0xcb39 => self.srl_c(operands),
                    0xcb3a => self.srl_d(operands),
                    0xcb3b => self.srl_e(operands),
                    0xcb3c => self.srl_h(operands),
                    0xcb3d => self.srl_l(operands),
                    0xcb3e => self.srl_hl(operands),
                    0xcb3f => self.srl_a(operands),
                    0xcb40 => self.bit_b(operands, 0x01),
                    0xcb41 => self.bit_c(operands, 0x01),
                    0xcb42 => self.bit_d(operands, 0x01),
                    0xcb43 => self.bit_e(operands, 0x01),
                    0xcb44 => self.bit_h(operands, 0x01),
                    0xcb45 => self.bit_l(operands, 0x01),
                    0xcb46 => self.bit_hl(operands, 0x01),
                    0xcb47 => self.bit_a(operands, 0x01),
                    0xcb48 => self.bit_b(operands, 0x02),
                    0xcb49 => self.bit_c(operands, 0x02),
                    0xcb4a => self.bit_d(operands, 0x02),
                    0xcb4b => self.bit_e(operands, 0x02),
                    0xcb4c => self.bit_h(operands, 0x02),
                    0xcb4d => self.bit_l(operands, 0x02),
                    0xcb4e => self.bit_hl(operands, 0x02),
                    0xcb4f => self.bit_a(operands, 0x02),
                    0xcb50 => self.bit_b(operands, 0x04),
                    0xcb51 => self.bit_c(operands, 0x04),
                    0xcb52 => self.bit_d(operands, 0x04),
                    0xcb53 => self.bit_e(operands, 0x04),
                    0xcb54 => self.bit_h(operands, 0x04),
                    0xcb55 => self.bit_l(operands, 0x04),
                    0xcb56 => self.bit_hl(operands, 0x04),
                    0xcb57 => self.bit_a(operands, 0x04),
                    0xcb58 => self.bit_b(operands, 0x08),
                    0xcb59 => self.bit_c(operands, 0x08),
                    0xcb5a => self.bit_d(operands, 0x08),
                    0xcb5b => self.bit_e(operands, 0x08),
                    0xcb5c => self.bit_h(operands, 0x08),
                    0xcb5d => self.bit_l(operands, 0x08),
                    0xcb5e => self.bit_hl(operands, 0x08),
                    0xcb5f => self.bit_a(operands, 0x08),
                    0xcb60 => self.bit_b(operands, 0x10),
                    0xcb61 => self.bit_c(operands, 0x10),
                    0xcb62 => self.bit_d(operands, 0x10),
                    0xcb63 => self.bit_e(operands, 0x10),
                    0xcb64 => self.bit_h(operands, 0x10),
                    0xcb65 => self.bit_l(operands, 0x10),
                    0xcb66 => self.bit_hl(operands, 0x10),
                    0xcb67 => self.bit_a(operands, 0x10),
                    0xcb68 => self.bit_b(operands, 0x20),
                    0xcb69 => self.bit_c(operands, 0x20),
                    0xcb6a => self.bit_d(operands, 0x20),
                    0xcb6b => self.bit_e(operands, 0x20),
                    0xcb6c => self.bit_h(operands, 0x20),
                    0xcb6d => self.bit_l(operands, 0x20),
                    0xcb6e => self.bit_hl(operands, 0x20),
                    0xcb6f => self.bit_a(operands, 0x20),
                    0xcb70 => self.bit_b(operands, 0x40),
                    0xcb71 => self.bit_c(operands, 0x40),
                    0xcb72 => self.bit_d(operands, 0x40),
                    0xcb73 => self.bit_e(operands, 0x40),
                    0xcb74 => self.bit_h(operands, 0x40),
                    0xcb75 => self.bit_l(operands, 0x40),
                    0xcb76 => self.bit_hl(operands, 0x40),
                    0xcb77 => self.bit_a(operands, 0x40),
                    0xcb78 => self.bit_b(operands, 0x80),
                    0xcb79 => self.bit_c(operands, 0x80),
                    0xcb7a => self.bit_d(operands, 0x80),
                    0xcb7b => self.bit_e(operands, 0x80),
                    0xcb7c => self.bit_h(operands, 0x80),
                    0xcb7d => self.bit_l(operands, 0x80),
                    0xcb7e => self.bit_hl(operands, 0x80),
                    0xcb7f => self.bit_a(operands, 0x80),
                    0xcb80 => self.res_b(operands, 0x01),
                    0xcb81 => self.res_c(operands, 0x01),
                    0xcb82 => self.res_d(operands, 0x01),
                    0xcb83 => self.res_e(operands, 0x01),
                    0xcb84 => self.res_h(operands, 0x01),
                    0xcb85 => self.res_l(operands, 0x01),
                    0xcb86 => self.res_hl(operands, 0x01),
                    0xcb87 => self.res_a(operands, 0x01),
                    0xcb88 => self.res_b(operands, 0x02),
                    0xcb89 => self.res_c(operands, 0x02),
                    0xcb8a => self.res_d(operands, 0x02),
                    0xcb8b => self.res_e(operands, 0x02),
                    0xcb8c => self.res_h(operands, 0x02),
                    0xcb8d => self.res_l(operands, 0x02),
                    0xcb8e => self.res_hl(operands, 0x02),
                    0xcb8f => self.res_a(operands, 0x02),
                    0xcb90 => self.res_b(operands, 0x04),
                    0xcb91 => self.res_c(operands, 0x04),
                    0xcb92 => self.res_d(operands, 0x04),
                    0xcb93 => self.res_e(operands, 0x04),
                    0xcb94 => self.res_h(operands, 0x04),
                    0xcb95 => self.res_l(operands, 0x04),
                    0xcb96 => self.res_hl(operands, 0x04),
                    0xcb97 => self.res_a(operands, 0x04),
                    0xcb98 => self.res_b(operands, 0x08),
                    0xcb99 => self.res_c(operands, 0x08),
                    0xcb9a => self.res_d(operands, 0x08),
                    0xcb9b => self.res_e(operands, 0x08),
                    0xcb9c => self.res_h(operands, 0x08),
                    0xcb9d => self.res_l(operands, 0x08),
                    0xcb9e => self.res_hl(operands, 0x08),
                    0xcb9f => self.res_a(operands, 0x08),
                    0xcba0 => self.res_b(operands, 0x10),
                    0xcba1 => self.res_c(operands, 0x10),
                    0xcba2 => self.res_d(operands, 0x10),
                    0xcba3 => self.res_e(operands, 0x10),
                    0xcba4 => self.res_h(operands, 0x10),
                    0xcba5 => self.res_l(operands, 0x10),
                    0xcba6 => self.res_hl(operands, 0x10),
                    0xcba7 => self.res_a(operands, 0x10),
                    0xcba8 => self.res_b(operands, 0x20),
                    0xcba9 => self.res_c(operands, 0x20),
                    0xcbaa => self.res_d(operands, 0x20),
                    0xcbab => self.res_e(operands, 0x20),
                    0xcbac => self.res_h(operands, 0x20),
                    0xcbad => self.res_l(operands, 0x20),
                    0xcbae => self.res_hl(operands, 0x20),
                    0xcbaf => self.res_a(operands, 0x20),
                    0xcbb0 => self.res_b(operands, 0x40),
                    0xcbb1 => self.res_c(operands, 0x40),
                    0xcbb2 => self.res_d(operands, 0x40),
                    0xcbb3 => self.res_e(operands, 0x40),
                    0xcbb4 => self.res_h(operands, 0x40),
                    0xcbb5 => self.res_l(operands, 0x40),
                    0xcbb6 => self.res_hl(operands, 0x40),
                    0xcbb7 => self.res_a(operands, 0x40),
                    0xcbb8 => self.res_b(operands, 0x80),
                    0xcbb9 => self.res_c(operands, 0x80),
                    0xcbba => self.res_d(operands, 0x80),
                    0xcbbb => self.res_e(operands, 0x80),
                    0xcbbc => self.res_h(operands, 0x80),
                    0xcbbd => self.res_l(operands, 0x80),
                    0xcbbe => self.res_hl(operands, 0x80),
                    0xcbbf => self.res_a(operands, 0x80),
                    0xcbc0 => self.set_bit_b(operands, 0x01),
                    0xcbc1 => self.set_bit_c(operands, 0x01),
                    0xcbc2 => self.set_bit_d(operands, 0x01),
                    0xcbc3 => self.set_bit_e(operands, 0x01),
                    0xcbc4 => self.set_bit_h(operands, 0x01),
                    0xcbc5 => self.set_bit_l(operands, 0x01),
                    0xcbc6 => self.set_bit_hl(operands, 0x01),
                    0xcbc7 => self.set_bit_a(operands, 0x01),
                    0xcbc8 => self.set_bit_b(operands, 0x02),
                    0xcbc9 => self.set_bit_c(operands, 0x02),
                    0xcbca => self.set_bit_d(operands, 0x02),
                    0xcbcb => self.set_bit_e(operands, 0x02),
                    0xcbcc => self.set_bit_h(operands, 0x02),
                    0xcbcd => self.set_bit_l(operands, 0x02),
                    0xcbce => self.set_bit_hl(operands, 0x02),
                    0xcbcf => self.set_bit_a(operands, 0x02),
                    0xcbd0 => self.set_bit_b(operands, 0x04),
                    0xcbd1 => self.set_bit_c(operands, 0x04),
                    0xcbd2 => self.set_bit_d(operands, 0x04),
                    0xcbd3 => self.set_bit_e(operands, 0x04),
                    0xcbd4 => self.set_bit_h(operands, 0x04),
                    0xcbd5 => self.set_bit_l(operands, 0x04),
                    0xcbd6 => self.set_bit_hl(operands, 0x04),
                    0xcbd7 => self.set_bit_a(operands, 0x04),
                    0xcbd8 => self.set_bit_b(operands, 0x08),
                    0xcbd9 => self.set_bit_c(operands, 0x08),
                    0xcbda => self.set_bit_d(operands, 0x08),
                    0xcbdb => self.set_bit_e(operands, 0x08),
                    0xcbdc => self.set_bit_h(operands, 0x08),
                    0xcbdd => self.set_bit_l(operands, 0x08),
                    0xcbde => self.set_bit_hl(operands, 0x08),
                    0xcbdf => self.set_bit_a(operands, 0x08),
                    0xcbe0 => self.set_bit_b(operands, 0x10),
                    0xcbe1 => self.set_bit_c(operands, 0x10),
                    0xcbe2 => self.set_bit_d(operands, 0x10),
                    0xcbe3 => self.set_bit_e(operands, 0x10),
                    0xcbe4 => self.set_bit_h(operands, 0x10),
                    0xcbe5 => self.set_bit_l(operands, 0x10),
                    0xcbe6 => self.set_bit_hl(operands, 0x10),
                    0xcbe7 => self.set_bit_a(operands, 0x10),
                    0xcbe8 => self.set_bit_b(operands, 0x20),
                    0xcbe9 => self.set_bit_c(operands, 0x20),
                    0xcbea => self.set_bit_d(operands, 0x20),
                    0xcbeb => self.set_bit_e(operands, 0x20),
                    0xcbec => self.set_bit_h(operands, 0x20),
                    0xcbed => self.set_bit_l(operands, 0x20),
                    0xcbee => self.set_bit_hl(operands, 0x20),
                    0xcbef => self.set_bit_a(operands, 0x20),
                    0xcbf0 => self.set_bit_b(operands, 0x40),
                    0xcbf1 => self.set_bit_c(operands, 0x40),
                    0xcbf2 => self.set_bit_d(operands, 0x40),
                    0xcbf3 => self.set_bit_e(operands, 0x40),
                    0xcbf4 => self.set_bit_h(operands, 0x40),
                    0xcbf5 => self.set_bit_l(operands, 0x40),
                    0xcbf6 => self.set_bit_hl(operands, 0x40),
                    0xcbf7 => self.set_bit_a(operands, 0x40),
                    0xcbf8 => self.set_bit_b(operands, 0x80),
                    0xcbf9 => self.set_bit_c(operands, 0x80),
                    0xcbfa => self.set_bit_d(operands, 0x80),
                    0xcbfb => self.set_bit_e(operands, 0x80),
                    0xcbfc => self.set_bit_h(operands, 0x80),
                    0xcbfd => self.set_bit_l(operands, 0x80),
                    0xcbfe => self.set_bit_hl(operands, 0x80),
                    0xcbff => self.set_bit_a(operands, 0x80),
                    _ => (),
                }
            }
        }
    }

    fn a(&self) -> u8 {
        self.a
    }

    fn f(&self) -> u8 {
        self.f
    }

    fn c_flag(&self) -> u8 {
        if self.f() & 0x10 == 0 {
            0
        } else {
            1
        }
    }

    fn b(&self) -> u8 {
        self.b
    }

    fn c(&self) -> u8 {
        self.c
    }

    fn bc(&self) -> u16 {
        u16::from_be_bytes([self.b, self.c])
    }

    fn d(&self) -> u8 {
        self.d
    }

    fn e(&self) -> u8 {
        self.e
    }

    fn de(&self) -> u16 {
        u16::from_be_bytes([self.d, self.e])
    }

    fn h(&self) -> u8 {
        self.h
    }

    fn l(&self) -> u8 {
        self.l
    }

    fn hl(&self) -> u16 {
        u16::from_be_bytes([self.h, self.l])
    }

    fn sp(&self) -> u16 {
        self.sp
    }

    fn set_a(&mut self, val: u8) {
        self.a = val;
    }

    fn set_f(&mut self, val: u8) {
        self.f = val;
    }

    fn set_z_flag(&mut self, val: bool) {
        match val {
            true => self.set_f(self.f() | 0x80),
            false => self.set_f(self.f() & 0x7f),
        };
    }

    fn set_n_flag(&mut self, val: bool) {
        match val {
            true => self.set_f(self.f() | 0x40),
            false => self.set_f(self.f() & 0xbf),
        };
    }

    fn set_h_flag(&mut self, val: bool) {
        match val {
            true => self.set_f(self.f() | 0x20),
            false => self.set_f(self.f() & 0xdf),
        };
    }

    fn update_h_flag(&mut self, operand1: u8, operand2: u8) {
        if (((operand1 & 0xf) + (operand2 & 0xf)) & 0x10) == 0x10 {
            self.set_h_flag(true);
        } else {
            self.set_h_flag(false);
        }
    }

    fn update_h_flag_with_carry(&mut self, operand1: u8, operand2: u8, operand3: u8) {
        if (((operand1 & 0xf) + (operand2 & 0xf) + (operand3 & 0xf)) & 0x10) == 0x10 {
            self.set_h_flag(true);
        } else {
            self.set_h_flag(false);
        }
    }

    fn update_h_flag_16_bit(&mut self, operand1: u16, operand2: u16) {
        if (((operand1 & 0xfff) + (operand2 & 0xfff)) & 0x1000) == 0x1000 {
            self.set_h_flag(true);
        } else {
            self.set_h_flag(false);
        }
    }

    fn set_c_flag(&mut self, val: bool) {
        match val {
            true => self.set_f(self.f() | 0x10),
            false => self.set_f(self.f() & 0xef),
        };
    }

    fn update_c_flag(&mut self, operand1: u8, operand2: u8) {
        match operand1.checked_add(operand2) {
            Some(_) => self.set_c_flag(false),
            None => self.set_c_flag(true),
        };
    }

    fn update_c_flag_with_carry(&mut self, operand1: u8, operand2: u8, operand3: u8) {
        match operand1.checked_add(operand2) {
            Some(partial_sum) => match partial_sum.checked_add(operand3) {
                Some(_) => self.set_c_flag(false),
                None => self.set_c_flag(true),
            },
            None => self.set_c_flag(true),
        };
    }

    fn update_c_flag_16_bit(&mut self, operand1: u16, operand2: u16) {
        match operand1.checked_add(operand2) {
            Some(_) => self.set_c_flag(false),
            None => self.set_c_flag(true),
        };
    }

    fn set_b(&mut self, val: u8) {
        self.b = val;
    }

    fn set_c(&mut self, val: u8) {
        self.c = val;
    }

    fn set_bc(&mut self, val: u16) {
        let bc = u16::to_be_bytes(val);
        self.b = bc[0];
        self.c = bc[1];
    }

    fn set_d(&mut self, val: u8) {
        self.d = val;
    }

    fn set_e(&mut self, val: u8) {
        self.e = val;
    }

    fn set_de(&mut self, val: u16) {
        let de = u16::to_be_bytes(val);
        self.d = de[0];
        self.e = de[1];
    }

    fn set_h(&mut self, val: u8) {
        self.h = val;
    }

    fn set_l(&mut self, val: u8) {
        self.l = val;
    }

    fn set_hl(&mut self, val: u16) {
        let hl = u16::to_be_bytes(val);
        self.h = hl[0];
        self.l = hl[1];
    }

    fn set_sp(&mut self, val: u16) {
        self.sp = val;
    }

    fn ld_b_b(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;
    }

    fn ld_b_c(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.set_b(self.c());
    }

    fn ld_b_d(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.set_b(self.d());
    }

    fn ld_b_e(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.set_b(self.e());
    }

    fn ld_b_h(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.set_b(self.h());
    }

    fn ld_b_l(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.set_b(self.l());
    }

    fn ld_b_a(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.set_b(self.a());
    }

    fn ld_c_b(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.set_c(self.b());
    }

    fn ld_c_c(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;
    }

    fn ld_c_d(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.set_c(self.d());
    }

    fn ld_c_e(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.set_c(self.e());
    }

    fn ld_c_h(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.set_c(self.h());
    }

    fn ld_c_l(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.set_c(self.l());
    }

    fn ld_c_a(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.set_c(self.a());
    }

    fn ld_d_b(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.set_d(self.b());
    }

    fn ld_d_c(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.set_d(self.c());
    }

    fn ld_d_d(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;
    }

    fn ld_d_e(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.set_d(self.e());
    }

    fn ld_d_h(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.set_d(self.h());
    }

    fn ld_d_l(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.set_d(self.l());
    }

    fn ld_d_a(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.set_d(self.a());
    }

    fn ld_e_b(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.set_e(self.b());
    }

    fn ld_e_c(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.set_e(self.c());
    }

    fn ld_e_d(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.set_e(self.d());
    }

    fn ld_e_e(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;
    }

    fn ld_e_h(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.set_e(self.h());
    }

    fn ld_e_l(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.set_e(self.l());
    }

    fn ld_e_a(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.set_e(self.a());
    }

    fn ld_h_b(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.set_h(self.b());
    }

    fn ld_h_c(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.set_h(self.c());
    }

    fn ld_h_d(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.set_h(self.d());
    }

    fn ld_h_e(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.set_h(self.e());
    }

    fn ld_h_h(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;
    }

    fn ld_h_l(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.set_h(self.l());
    }

    fn ld_h_a(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.set_h(self.a());
    }

    fn ld_l_b(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.set_l(self.b());
    }

    fn ld_l_c(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.set_l(self.c());
    }

    fn ld_l_d(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.set_l(self.d());
    }

    fn ld_l_e(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.set_l(self.e());
    }

    fn ld_l_h(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.set_l(self.h());
    }

    fn ld_l_l(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;
    }

    fn ld_l_a(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.set_l(self.a());
    }

    fn ld_a_b(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.set_a(self.b());
    }

    fn ld_a_c(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.set_a(self.c());
    }

    fn ld_a_d(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.set_a(self.d());
    }

    fn ld_a_e(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.set_a(self.e());
    }

    fn ld_a_h(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.set_a(self.h());
    }

    fn ld_a_l(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.set_a(self.l());
    }

    fn ld_a_a(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;
    }

    fn ld_b_d8(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 2;

        self.set_b(operands[0]);
    }

    fn ld_c_d8(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 2;

        self.set_c(operands[0]);
    }

    fn ld_d_d8(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 2;

        self.set_d(operands[0]);
    }

    fn ld_e_d8(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 2;

        self.set_e(operands[0]);
    }

    fn ld_h_d8(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 2;

        self.set_h(operands[0]);
    }

    fn ld_l_d8(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 2;

        self.set_l(operands[0]);
    }

    fn ld_a_d8(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 2;

        self.set_a(operands[0]);
    }

    fn ld_b_hl(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 1;

        self.set_b(self.ram.load(self.hl()));
    }

    fn ld_c_hl(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 1;

        self.set_c(self.ram.load(self.hl()));
    }

    fn ld_d_hl(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 1;

        self.set_d(self.ram.load(self.hl()));
    }

    fn ld_e_hl(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 1;

        self.set_e(self.ram.load(self.hl()));
    }

    fn ld_h_hl(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 1;

        self.set_h(self.ram.load(self.hl()));
    }

    fn ld_l_hl(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 1;

        self.set_l(self.ram.load(self.hl()));
    }

    fn ld_a_hl(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 1;

        self.set_a(self.ram.load(self.hl()));
    }

    fn ld_hl_b(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 1;

        self.ram.store(self.hl(), self.b());
    }

    fn ld_hl_c(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 1;

        self.ram.store(self.hl(), self.c());
    }

    fn ld_hl_d(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 1;

        self.ram.store(self.hl(), self.d());
    }

    fn ld_hl_e(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 1;

        self.ram.store(self.hl(), self.e());
    }

    fn ld_hl_h(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 1;

        self.ram.store(self.hl(), self.h());
    }

    fn ld_hl_l(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 1;

        self.ram.store(self.hl(), self.l());
    }

    fn ld_hl_a(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 1;

        self.ram.store(self.hl(), self.a());
    }

    fn ld_hl_d8(&mut self, operands: Vec<u8>) {
        self.time += 3;
        self.pc += 2;

        self.ram.store(self.hl(), operands[0]);
    }

    fn ld_a_bc(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 1;

        self.set_a(self.ram.load(self.bc()));
    }

    fn ld_a_de(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 1;

        self.set_a(self.ram.load(self.de()));
    }

    fn ld_bc_a(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 1;

        self.ram.store(self.bc(), self.a());
    }

    fn ld_de_a(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 1;

        self.ram.store(self.de(), self.a());
    }

    fn ld_a_a16(&mut self, operands: Vec<u8>) {
        self.time += 4;
        self.pc += 3;

        self.set_a(self.ram.load(u16::from_le_bytes([operands[0], operands[1]])));
    }

    fn ld_a16_a(&mut self, operands: Vec<u8>) {
        self.time += 4;
        self.pc += 3;

        self.ram.store(u16::from_le_bytes([operands[0], operands[1]]), self.a());
    }

    fn ldh_a_c(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 1;

        self.set_a(self.ram.load(u16::from_be_bytes([0xff, self.c()])));
    }

    fn ldh_c_a(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 1;

        self.ram.store(u16::from_be_bytes([0xff, self.c()]), self.a());
    }

    fn ldh_a_a8(&mut self, operands: Vec<u8>) {
        self.time += 3;
        self.pc += 2;

        self.set_a(self.ram.load(u16::from_be_bytes([0xff, operands[0]])));
    }

    fn ldh_a8_a(&mut self, operands: Vec<u8>) {
        self.time += 3;
        self.pc += 2;

        self.ram.store(u16::from_be_bytes([0xff, operands[0]]), self.a());
    }

    fn ld_a_hl_minus(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 1;

        self.set_a(self.ram.load(self.hl()));
        let new_hl = u16::from_be_bytes([self.h(), self.l()]) - 1;
        let new_hl = u16::to_be_bytes(new_hl);
        self.set_h(new_hl[0]);
        self.set_l(new_hl[1]);
    }

    fn ld_hl_minus_a(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 1;

        self.ram.store(self.hl(), self.a());
        let new_hl = u16::from_be_bytes([self.h(), self.l()]) - 1;
        let new_hl = u16::to_be_bytes(new_hl);
        self.set_h(new_hl[0]);
        self.set_l(new_hl[1]);
    }

    fn ld_a_hl_plus(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 1;

        self.set_a(self.ram.load(self.hl()));
        let new_hl = u16::from_be_bytes([self.h(), self.l()]) + 1;
        let new_hl = u16::to_be_bytes(new_hl);
        self.set_h(new_hl[0]);
        self.set_l(new_hl[1]);
    }

    fn ld_hl_plus_a(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 1;

        self.ram.store(self.hl(), self.a());
        let new_hl = u16::from_be_bytes([self.h(), self.l()]) + 1;
        let new_hl = u16::to_be_bytes(new_hl);
        self.set_h(new_hl[0]);
        self.set_l(new_hl[1]);
    }

    fn ld_bc_d16(&mut self, operands: Vec<u8>) {
        self.time += 3;
        self.pc += 3;

        self.set_c(operands[0]);
        self.set_b(operands[1]);
    }

    fn ld_de_d16(&mut self, operands: Vec<u8>) {
        self.time += 3;
        self.pc += 3;

        self.set_e(operands[0]);
        self.set_d(operands[1]);
    }

    fn ld_hl_d16(&mut self, operands: Vec<u8>) {
        self.time += 3;
        self.pc += 3;

        self.set_l(operands[0]);
        self.set_h(operands[1]);
    }

    fn ld_sp_d16(&mut self, operands: Vec<u8>) {
        self.time += 3;
        self.pc += 3;

        self.set_sp(u16::from_le_bytes([operands[0], operands[1]]));
    }

    fn ld_a16_sp(&mut self, operands: Vec<u8>) {
        self.time += 5;
        self.pc += 3;

        let bytes = u16::to_be_bytes(self.sp());
        let s = bytes[0];
        let p = bytes[1];
        self.ram.store(u16::from_le_bytes([operands[0], operands[1]]), p);
        self.ram.store(u16::from_le_bytes([operands[0], operands[1]]) + 1, s);
    }

    fn ld_sp_hl(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 1;

        self.set_sp(self.hl());
    }

    fn push_bc(&mut self, operands: Vec<u8>) {
        self.time += 4;
        self.pc += 1;

        self.ram.store(self.sp() - 1, self.b());
        self.ram.store(self.sp() - 2, self.c());
        self.set_sp(self.sp() - 2);
    }

    fn push_de(&mut self, operands: Vec<u8>) {
        self.time += 4;
        self.pc += 1;

        self.ram.store(self.sp() - 1, self.d());
        self.ram.store(self.sp() - 2, self.e());
        self.set_sp(self.sp() - 2);
    }

    fn push_hl(&mut self, operands: Vec<u8>) {
        self.time += 4;
        self.pc += 1;

        self.ram.store(self.sp() - 1, self.h());
        self.ram.store(self.sp() - 2, self.l());
        self.set_sp(self.sp() - 2);
    }

    fn push_af(&mut self, operands: Vec<u8>) {
        self.time += 4;
        self.pc += 1;

        self.ram.store(self.sp() - 1, self.a());
        self.ram.store(self.sp() - 2, self.f());
        self.set_sp(self.sp() - 2);
    }

    fn pop_bc(&mut self, operands: Vec<u8>) {
        self.time += 3;
        self.pc += 1;

        self.set_c(self.ram.load(self.sp()));
        self.set_b(self.ram.load(self.sp() + 1));
        self.set_sp(self.sp() + 2);
    }

    fn pop_de(&mut self, operands: Vec<u8>) {
        self.time += 3;
        self.pc += 1;

        self.set_e(self.ram.load(self.sp()));
        self.set_d(self.ram.load(self.sp() + 1));
        self.set_sp(self.sp() + 2);
    }

    fn pop_hl(&mut self, operands: Vec<u8>) {
        self.time += 3;
        self.pc += 1;

        self.set_l(self.ram.load(self.sp()));
        self.set_h(self.ram.load(self.sp() + 1));
        self.set_sp(self.sp() + 2);
    }

    fn pop_af(&mut self, operands: Vec<u8>) {
        self.time += 3;
        self.pc += 1;

        self.set_f(self.ram.load(self.sp()));
        self.set_a(self.ram.load(self.sp() + 1));
        self.set_sp(self.sp() + 2);
    }

    fn ld_hl_sp_plus_s8(&mut self, operands: Vec<u8>) {
        self.time += 3;
        self.pc += 2;

        let sp_offset = operands[0] as i8;
        let sp_offset = sp_offset as i16;
        let new_sp = (self.sp() as i16 + sp_offset) as u16;
        let bytes = u16::to_be_bytes(new_sp);
        self.set_h(bytes[0]);
        self.set_l(bytes[1]);

        self.set_z_flag(false);
        self.set_n_flag(false);
        let sp_low = u16::to_be_bytes(self.sp())[1];
        self.update_h_flag(sp_low, operands[0]);
        self.update_c_flag(sp_low, operands[0]);
    }

    fn add_a_b(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.update_h_flag(self.a(), self.b());
        self.update_c_flag(self.a(), self.b());

        self.set_a(self.a().wrapping_add(self.b()));

        if self.a() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
    }

    fn add_a_c(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.update_h_flag(self.a(), self.c());
        self.update_c_flag(self.a(), self.c());

        self.set_a(self.a().wrapping_add(self.c()));

        if self.a() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
    }

    fn add_a_d(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.update_h_flag(self.a(), self.d());
        self.update_c_flag(self.a(), self.d());

        self.set_a(self.a().wrapping_add(self.d()));

        if self.a() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
    }

    fn add_a_e(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.update_h_flag(self.a(), self.e());
        self.update_c_flag(self.a(), self.e());

        self.set_a(self.a().wrapping_add(self.e()));

        if self.a() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
    }

    fn add_a_h(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.update_h_flag(self.a(), self.h());
        self.update_c_flag(self.a(), self.h());

        self.set_a(self.a().wrapping_add(self.h()));

        if self.a() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
    }

    fn add_a_l(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.update_h_flag(self.a(), self.l());
        self.update_c_flag(self.a(), self.l());

        self.set_a(self.a().wrapping_add(self.l()));

        if self.a() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
    }

    fn add_a_a(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.update_h_flag(self.a(), self.a());
        self.update_c_flag(self.a(), self.a());

        self.set_a(self.a().wrapping_add(self.a()));

        if self.a() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
    }

    fn add_a_hl(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 1;

        self.update_h_flag(self.ram.load(self.hl()), self.a());
        self.update_c_flag(self.ram.load(self.hl()), self.a());

        self.set_a(self.ram.load(self.hl()).wrapping_add(self.a()));

        if self.a() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
    }

    fn add_a_d8(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 2;

        self.update_h_flag(self.a(), operands[0]);
        self.update_c_flag(self.a(), operands[0]);

        self.set_a(self.a().wrapping_add(operands[0]));

        if self.a() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
    }

    fn adc_a_b(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        let carry_in = self.c_flag();
        self.update_h_flag_with_carry(self.a(), self.b(), carry_in);
        self.update_c_flag_with_carry(self.a(), self.b(), carry_in);

        self.set_a(self.a().wrapping_add(self.b()).wrapping_add(carry_in));

        if self.a() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
    }

    fn adc_a_c(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        let carry_in = self.c_flag();
        self.update_h_flag_with_carry(self.a(), self.c(), carry_in);
        self.update_c_flag_with_carry(self.a(), self.c(), carry_in);

        self.set_a(self.a().wrapping_add(self.c()).wrapping_add(carry_in));

        if self.a() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
    }

    fn adc_a_d(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        let carry_in = self.c_flag();
        self.update_h_flag_with_carry(self.a(), self.d(), carry_in);
        self.update_c_flag_with_carry(self.a(), self.d(), carry_in);

        self.set_a(self.a().wrapping_add(self.d()).wrapping_add(carry_in));

        if self.a() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
    }

    fn adc_a_e(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        let carry_in = self.c_flag();
        self.update_h_flag_with_carry(self.a(), self.e(), carry_in);
        self.update_c_flag_with_carry(self.a(), self.e(), carry_in);

        self.set_a(self.a().wrapping_add(self.e()).wrapping_add(carry_in));

        if self.a() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
    }

    fn adc_a_h(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        let carry_in = self.c_flag();
        self.update_h_flag_with_carry(self.a(), self.h(), carry_in);
        self.update_c_flag_with_carry(self.a(), self.h(), carry_in);

        self.set_a(self.a().wrapping_add(self.h()).wrapping_add(carry_in));

        if self.a() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
    }

    fn adc_a_l(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        let carry_in = self.c_flag();
        self.update_h_flag_with_carry(self.a(), self.l(), carry_in);
        self.update_c_flag_with_carry(self.a(), self.l(), carry_in);

        self.set_a(self.a().wrapping_add(self.l()).wrapping_add(carry_in));

        if self.a() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
    }

    fn adc_a_a(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        let carry_in = self.c_flag();
        self.update_h_flag_with_carry(self.a(), self.a(), carry_in);
        self.update_c_flag_with_carry(self.a(), self.a(), carry_in);

        self.set_a(self.a().wrapping_add(self.a()).wrapping_add(carry_in));

        if self.a() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
    }

    fn adc_a_hl(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 1;

        let carry_in = self.c_flag();
        self.update_h_flag_with_carry(self.a(), self.ram.load(self.hl()), carry_in);
        self.update_c_flag_with_carry(self.a(), self.ram.load(self.hl()), carry_in);

        self.set_a(self.a().wrapping_add(self.ram.load(self.hl())).wrapping_add(carry_in));

        if self.a() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
    }

    fn adc_a_d8(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 2;

        let carry_in = self.c_flag();
        self.update_h_flag_with_carry(self.a(), operands[0], carry_in);
        self.update_c_flag_with_carry(self.a(), operands[0], carry_in);

        self.set_a(self.a().wrapping_add(operands[0]).wrapping_add(carry_in));

        if self.a() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
    }

    fn sub_b(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.update_h_flag(self.a(), self.b().wrapping_neg());
        self.update_c_flag(self.a(), self.b().wrapping_neg());

        self.set_a(self.a().wrapping_sub(self.b()));

        if self.a() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(true);
    }

    fn sub_c(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.update_h_flag(self.a(), self.c().wrapping_neg());
        self.update_c_flag(self.a(), self.c().wrapping_neg());

        self.set_a(self.a().wrapping_sub(self.c()));

        if self.a() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(true);
    }

    fn sub_d(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.update_h_flag(self.a(), self.d().wrapping_neg());
        self.update_c_flag(self.a(), self.d().wrapping_neg());

        self.set_a(self.a().wrapping_sub(self.d()));

        if self.a() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(true);
    }

    fn sub_e(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.update_h_flag(self.a(), self.e().wrapping_neg());
        self.update_c_flag(self.a(), self.e().wrapping_neg());

        self.set_a(self.a().wrapping_sub(self.e()));

        if self.a() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(true);
    }

    fn sub_h(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.update_h_flag(self.a(), self.h().wrapping_neg());
        self.update_c_flag(self.a(), self.h().wrapping_neg());

        self.set_a(self.a().wrapping_sub(self.h()));

        if self.a() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(true);
    }

    fn sub_l(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.update_h_flag(self.a(), self.l().wrapping_neg());
        self.update_c_flag(self.a(), self.l().wrapping_neg());

        self.set_a(self.a().wrapping_sub(self.l()));

        if self.a() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(true);
    }

    fn sub_a(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.update_h_flag(self.a(), self.a().wrapping_neg());
        self.update_c_flag(self.a(), self.a().wrapping_neg());

        self.set_a(self.a().wrapping_sub(self.a()));

        if self.a() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(true);
    }

    fn sub_hl(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 1;

        self.update_h_flag(self.a(), self.ram.load(self.hl()).wrapping_neg());
        self.update_c_flag(self.a(), self.ram.load(self.hl()).wrapping_neg());

        self.set_a(self.a().wrapping_sub(self.ram.load(self.hl())));

        if self.a() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(true);
    }

    fn sub_d8(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 2;

        self.update_h_flag(self.a(), operands[0].wrapping_neg());
        self.update_c_flag(self.a(), operands[0].wrapping_neg());

        self.set_a(self.a().wrapping_sub(operands[0]));

        if self.a() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(true);
    }

    fn sbc_a_b(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        let carry_in = self.c_flag();
        self.update_h_flag_with_carry(self.a(), self.b().wrapping_neg(), carry_in.wrapping_neg());
        self.update_c_flag_with_carry(self.a(), self.b().wrapping_neg(), carry_in.wrapping_neg());

        self.set_a(self.a().wrapping_sub(self.b()).wrapping_sub(carry_in));

        if self.a() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(true);
    }

    fn sbc_a_c(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        let carry_in = self.c_flag();
        self.update_h_flag_with_carry(self.a(), self.c().wrapping_neg(), carry_in.wrapping_neg());
        self.update_c_flag_with_carry(self.a(), self.c().wrapping_neg(), carry_in.wrapping_neg());

        self.set_a(self.a().wrapping_sub(self.c()).wrapping_sub(carry_in));

        if self.a() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(true);
    }

    fn sbc_a_d(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        let carry_in = self.c_flag();
        self.update_h_flag_with_carry(self.a(), self.d().wrapping_neg(), carry_in.wrapping_neg());
        self.update_c_flag_with_carry(self.a(), self.d().wrapping_neg(), carry_in.wrapping_neg());

        self.set_a(self.a().wrapping_sub(self.d()).wrapping_sub(carry_in));

        if self.a() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(true);
    }

    fn sbc_a_e(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        let carry_in = self.c_flag();
        self.update_h_flag_with_carry(self.a(), self.e().wrapping_neg(), carry_in.wrapping_neg());
        self.update_c_flag_with_carry(self.a(), self.e().wrapping_neg(), carry_in.wrapping_neg());

        self.set_a(self.a().wrapping_sub(self.e()).wrapping_sub(carry_in));

        if self.a() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(true);
    }

    fn sbc_a_h(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        let carry_in = self.c_flag();
        self.update_h_flag_with_carry(self.a(), self.h().wrapping_neg(), carry_in.wrapping_neg());
        self.update_c_flag_with_carry(self.a(), self.h().wrapping_neg(), carry_in.wrapping_neg());

        self.set_a(self.a().wrapping_sub(self.h()).wrapping_sub(carry_in));

        if self.a() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(true);
    }

    fn sbc_a_l(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        let carry_in = self.c_flag();
        self.update_h_flag_with_carry(self.a(), self.l().wrapping_neg(), carry_in.wrapping_neg());
        self.update_c_flag_with_carry(self.a(), self.l().wrapping_neg(), carry_in.wrapping_neg());

        self.set_a(self.a().wrapping_sub(self.l()).wrapping_sub(carry_in));

        if self.a() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(true);
    }

    fn sbc_a_a(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        let carry_in = self.c_flag();
        self.update_h_flag_with_carry(self.a(), self.a().wrapping_neg(), carry_in.wrapping_neg());
        self.update_c_flag_with_carry(self.a(), self.a().wrapping_neg(), carry_in.wrapping_neg());

        self.set_a(self.a().wrapping_sub(self.a()).wrapping_sub(carry_in));

        if self.a() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(true);
    }

    fn sbc_a_hl(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 1;

        let carry_in = self.c_flag();
        self.update_h_flag_with_carry(self.a(), self.ram.load(self.hl()).wrapping_neg(), carry_in.wrapping_neg());
        self.update_c_flag_with_carry(self.a(), self.ram.load(self.hl()).wrapping_neg(), carry_in.wrapping_neg());

        self.set_a(self.a().wrapping_sub(self.ram.load(self.hl())).wrapping_sub(carry_in));

        if self.a() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(true);
    }

    fn sbc_a_d8(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 2;

        let carry_in = self.c_flag();
        self.update_h_flag_with_carry(self.a(), operands[0].wrapping_neg(), carry_in.wrapping_neg());
        self.update_c_flag_with_carry(self.a(), operands[0].wrapping_neg(), carry_in.wrapping_neg());

        self.set_a(self.a().wrapping_sub(operands[0]).wrapping_sub(carry_in));

        if self.a() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(true);
    }

    fn cp_b(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.update_h_flag(self.a(), self.b().wrapping_neg());
        self.update_c_flag(self.a(), self.b().wrapping_neg());
        if self.a() == self.b() {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(true);
    }

    fn cp_c(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.update_h_flag(self.a(), self.c().wrapping_neg());
        self.update_c_flag(self.a(), self.c().wrapping_neg());
        if self.a() == self.c() {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(true);
    }

    fn cp_d(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.update_h_flag(self.a(), self.d().wrapping_neg());
        self.update_c_flag(self.a(), self.d().wrapping_neg());
        if self.a() == self.d() {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(true);
    }

    fn cp_e(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.update_h_flag(self.a(), self.e().wrapping_neg());
        self.update_c_flag(self.a(), self.e().wrapping_neg());
        if self.a() == self.e() {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(true);
    }

    fn cp_h(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.update_h_flag(self.a(), self.h().wrapping_neg());
        self.update_c_flag(self.a(), self.h().wrapping_neg());
        if self.a() == self.h() {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(true);
    }

    fn cp_l(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.update_h_flag(self.a(), self.l().wrapping_neg());
        self.update_c_flag(self.a(), self.l().wrapping_neg());
        if self.a() == self.l() {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(true);
    }

    fn cp_a(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.update_h_flag(self.a(), self.a().wrapping_neg());
        self.update_c_flag(self.a(), self.a().wrapping_neg());
        if self.a() == self.a() {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(true);
    }

    fn cp_hl(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 1;

        self.update_h_flag(self.a(), self.ram.load(self.hl()).wrapping_neg());
        self.update_c_flag(self.a(), self.ram.load(self.hl()).wrapping_neg());
        if self.a() == self.ram.load(self.hl()) {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(true);
    }

    fn cp_d8(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 2;

        self.update_h_flag(self.a(), operands[0].wrapping_neg());
        self.update_c_flag(self.a(), operands[0].wrapping_neg());
        if self.a() == operands[0] {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(true);
    }

    fn inc_b(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.update_h_flag(self.b(), 1);

        self.set_b(self.b().wrapping_add(1));

        if self.b() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
    }

    fn inc_c(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.update_h_flag(self.c(), 1);

        self.set_c(self.c().wrapping_add(1));

        if self.c() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
    }

    fn inc_d(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.update_h_flag(self.d(), 1);

        self.set_d(self.d().wrapping_add(1));

        if self.d() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
    }

    fn inc_e(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.update_h_flag(self.e(), 1);

        self.set_e(self.e().wrapping_add(1));

        if self.e() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
    }

    fn inc_h(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.update_h_flag(self.h(), 1);

        self.set_h(self.h().wrapping_add(1));

        if self.h() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
    }

    fn inc_l(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.update_h_flag(self.l(), 1);

        self.set_l(self.l().wrapping_add(1));

        if self.l() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
    }

    fn inc_a(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.update_h_flag(self.a(), 1);

        self.set_a(self.a().wrapping_add(1));

        if self.a() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
    }

    fn inc_hl(&mut self, operands: Vec<u8>) {
        self.time += 3;
        self.pc += 1;

        self.update_h_flag(self.ram.load(self.hl()), 1);

        self.ram.store(self.hl(), self.ram.load(self.hl()).wrapping_add(1));

        if self.ram.load(self.hl()) == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
    }

    fn dec_b(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.update_h_flag(self.b(), 1u8.wrapping_neg());

        self.set_b(self.b().wrapping_sub(1));

        if self.b() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(true);
    }

    fn dec_c(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.update_h_flag(self.c(), 1u8.wrapping_neg());

        self.set_c(self.c().wrapping_sub(1));

        if self.c() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(true);
    }

    fn dec_d(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.update_h_flag(self.d(), 1u8.wrapping_neg());

        self.set_d(self.d().wrapping_sub(1));

        if self.d() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(true);
    }

    fn dec_e(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.update_h_flag(self.e(), 1u8.wrapping_neg());

        self.set_e(self.e().wrapping_sub(1));

        if self.e() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(true);
    }

    fn dec_h(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.update_h_flag(self.h(), 1u8.wrapping_neg());

        self.set_h(self.h().wrapping_sub(1));

        if self.h() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(true);
    }

    fn dec_l(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.update_h_flag(self.l(), 1u8.wrapping_neg());

        self.set_l(self.l().wrapping_sub(1));

        if self.l() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(true);
    }

    fn dec_a(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.update_h_flag(self.a(), 1u8.wrapping_neg());

        self.set_a(self.a().wrapping_sub(1));

        if self.a() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(true);
    }

    fn dec_hl(&mut self, operands: Vec<u8>) {
        self.time += 3;
        self.pc += 1;

        self.update_h_flag(self.ram.load(self.hl()), 1u8.wrapping_neg());

        self.ram.store(self.hl(), self.ram.load(self.hl()).wrapping_sub(1));

        if self.ram.load(self.hl()) == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(true);
    }

    fn and_b(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.set_a(self.a() & self.b());

        if self.a() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(true);
        self.set_c_flag(false);
    }

    fn and_c(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.set_a(self.a() & self.c());

        if self.a() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(true);
        self.set_c_flag(false);
    }

    fn and_d(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.set_a(self.a() & self.d());

        if self.a() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(true);
        self.set_c_flag(false);
    }

    fn and_e(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.set_a(self.a() & self.e());

        if self.a() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(true);
        self.set_c_flag(false);
    }

    fn and_h(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.set_a(self.a() & self.h());

        if self.a() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(true);
        self.set_c_flag(false);
    }

    fn and_l(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.set_a(self.a() & self.l());

        if self.a() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(true);
        self.set_c_flag(false);
    }

    fn and_a(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.set_a(self.a() & self.a());

        if self.a() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(true);
        self.set_c_flag(false);
    }

    fn and_hl(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 1;

        self.set_a(self.a() & self.ram.load(self.hl()));

        if self.a() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(true);
        self.set_c_flag(false);
    }

    fn and_d8(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 2;

        self.set_a(self.a() & operands[0]);

        if self.a() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(true);
        self.set_c_flag(false);
    }

    fn or_b(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.set_a(self.a() | self.b());

        if self.a() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
        self.set_c_flag(false);
    }

    fn or_c(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.set_a(self.a() | self.c());

        if self.a() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
        self.set_c_flag(false);
    }

    fn or_d(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.set_a(self.a() | self.d());

        if self.a() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
        self.set_c_flag(false);
    }

    fn or_e(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.set_a(self.a() | self.e());

        if self.a() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
        self.set_c_flag(false);
    }

    fn or_h(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.set_a(self.a() | self.h());

        if self.a() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
        self.set_c_flag(false);
    }

    fn or_l(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.set_a(self.a() | self.l());

        if self.a() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
        self.set_c_flag(false);
    }

    fn or_a(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.set_a(self.a() | self.a());

        if self.a() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
        self.set_c_flag(false);
    }

    fn or_hl(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 1;

        self.set_a(self.a() | self.ram.load(self.hl()));

        if self.a() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
        self.set_c_flag(false);
    }

    fn or_d8(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 2;

        self.set_a(self.a() | operands[0]);

        if self.a() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
        self.set_c_flag(false);
    }

    fn xor_b(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.set_a(self.a() ^ self.b());

        if self.a() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
        self.set_c_flag(false);
    }

    fn xor_c(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.set_a(self.a() ^ self.c());

        if self.a() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
        self.set_c_flag(false);
    }

    fn xor_d(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.set_a(self.a() ^ self.d());

        if self.a() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
        self.set_c_flag(false);
    }

    fn xor_e(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.set_a(self.a() ^ self.e());

        if self.a() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
        self.set_c_flag(false);
    }

    fn xor_h(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.set_a(self.a() ^ self.h());

        if self.a() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
        self.set_c_flag(false);
    }

    fn xor_l(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.set_a(self.a() ^ self.l());

        if self.a() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
        self.set_c_flag(false);
    }

    fn xor_a(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.set_a(self.a() ^ self.a());

        if self.a() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
        self.set_c_flag(false);
    }

    fn xor_hl(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 1;

        self.set_a(self.a() ^ self.ram.load(self.hl()));

        if self.a() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
        self.set_c_flag(false);
    }

    fn xor_d8(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 2;

        self.set_a(self.a() ^ operands[0]);

        if self.a() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
        self.set_c_flag(false);
    }

    fn ccf(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.set_n_flag(false);
        self.set_h_flag(false);
        if self.c_flag() == 0 {
            self.set_c_flag(true);
        } else {
            self.set_c_flag(false);
        }
    }

    fn scf(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.set_n_flag(false);
        self.set_h_flag(false);
        self.set_c_flag(true);
    }

    fn cpl(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.set_a(!self.a());

        self.set_n_flag(true);
        self.set_h_flag(true);
    }

    fn inc_bc(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 1;

        self.set_bc(self.bc().wrapping_add(1));
    }

    fn inc_de(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 1;

        self.set_de(self.de().wrapping_add(1));
    }

    fn inc_hl_reg(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 1;

        self.set_hl(self.hl().wrapping_add(1));
    }

    fn inc_sp(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 1;

        self.set_sp(self.sp().wrapping_add(1));
    }

    fn dec_bc(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 1;

        self.set_bc(self.bc().wrapping_sub(1));
    }

    fn dec_de(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 1;

        self.set_de(self.de().wrapping_sub(1));
    }

    fn dec_hl_reg(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 1;

        self.set_hl(self.hl().wrapping_sub(1));
    }

    fn dec_sp(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 1;

        self.set_sp(self.sp().wrapping_sub(1));
    }

    fn add_hl_bc(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 1;

        self.update_h_flag_16_bit(self.hl(), self.bc());
        self.update_c_flag_16_bit(self.hl(), self.bc());

        self.set_hl(self.hl() + self.bc());

        self.set_n_flag(false);
    }

    fn add_hl_de(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 1;

        self.update_h_flag_16_bit(self.hl(), self.de());
        self.update_c_flag_16_bit(self.hl(), self.de());

        self.set_hl(self.hl() + self.de());

        self.set_n_flag(false);
    }

    fn add_hl_hl(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 1;

        self.update_h_flag_16_bit(self.hl(), self.hl());
        self.update_c_flag_16_bit(self.hl(), self.hl());

        self.set_hl(self.hl() + self.hl());

        self.set_n_flag(false);
    }

    fn add_hl_sp(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 1;

        self.update_h_flag_16_bit(self.hl(), self.sp());
        self.update_c_flag_16_bit(self.hl(), self.sp());

        self.set_hl(self.hl() + self.sp());

        self.set_n_flag(false);
    }

    fn add_sp_s8(&mut self, operands: Vec<u8>) {
        self.time += 4;
        self.pc += 2;

        let sp = u16::to_be_bytes(self.sp());
        self.update_h_flag(sp[1], operands[0]);
        self.update_c_flag(sp[1], operands[0]);

        let s8: i8 = operands[0] as i8;
        self.set_sp(((self.sp() as i16).wrapping_add(s8 as i16)) as u16);

        self.set_z_flag(false);
        self.set_n_flag(false);
    }

    fn rlca(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        if (self.a() & 0x80) == 0x80 {
            self.set_c_flag(true);
        } else {
            self.set_c_flag(false);
        }

        self.set_a(self.a().rotate_left(1));

        self.set_z_flag(false);
        self.set_n_flag(false);
        self.set_h_flag(false);
    }

    fn rrca(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        if (self.a() & 0x01) == 0x01 {
            self.set_c_flag(true);
        } else {
            self.set_c_flag(false);
        }

        self.set_a(self.a().rotate_right(1));

        self.set_z_flag(false);
        self.set_n_flag(false);
        self.set_h_flag(false);
    }

    fn rla(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        let c = self.c_flag();

        if (self.a() & 0x80) == 0x80 {
            self.set_c_flag(true);
        } else {
            self.set_c_flag(false);
        }

        self.set_a(self.a().rotate_left(1));
        if c == 1 {
            self.set_a(self.a() | 0x01);
        } else {
            self.set_a(self.a() & 0xfe);
        }

        self.set_z_flag(false);
        self.set_n_flag(false);
        self.set_h_flag(false);
    }

    fn rra(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        let c = self.c_flag();

        if (self.a() & 0x01) == 0x01 {
            self.set_c_flag(true);
        } else {
            self.set_c_flag(false);
        }

        self.set_a(self.a().rotate_right(1));
        if c == 1 {
            self.set_a(self.a() | 0x80);
        } else {
            self.set_a(self.a() & 0x7f);
        }

        self.set_z_flag(false);
        self.set_n_flag(false);
        self.set_h_flag(false);
    }

    fn rlc_b(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 2;

        if (self.b() & 0x80) == 0x80 {
            self.set_c_flag(true);
        } else {
            self.set_c_flag(false);
        }

        self.set_b(self.b().rotate_left(1));

        if self.b() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
    }

    fn rlc_c(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 2;

        if (self.c() & 0x80) == 0x80 {
            self.set_c_flag(true);
        } else {
            self.set_c_flag(false);
        }

        self.set_c(self.c().rotate_left(1));

        if self.c() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
    }

    fn rlc_d(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 2;

        if (self.d() & 0x80) == 0x80 {
            self.set_c_flag(true);
        } else {
            self.set_c_flag(false);
        }

        self.set_d(self.d().rotate_left(1));

        if self.d() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
    }

    fn rlc_e(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 2;

        if (self.e() & 0x80) == 0x80 {
            self.set_c_flag(true);
        } else {
            self.set_c_flag(false);
        }

        self.set_e(self.e().rotate_left(1));

        if self.e() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
    }

    fn rlc_h(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 2;

        if (self.h() & 0x80) == 0x80 {
            self.set_c_flag(true);
        } else {
            self.set_c_flag(false);
        }

        self.set_h(self.h().rotate_left(1));

        if self.h() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
    }

    fn rlc_l(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 2;

        if (self.l() & 0x80) == 0x80 {
            self.set_c_flag(true);
        } else {
            self.set_c_flag(false);
        }

        self.set_l(self.l().rotate_left(1));

        if self.l() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
    }

    fn rlc_a(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 2;

        if (self.a() & 0x80) == 0x80 {
            self.set_c_flag(true);
        } else {
            self.set_c_flag(false);
        }

        self.set_a(self.a().rotate_left(1));

        if self.a() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
    }

    fn rlc_hl(&mut self, operands: Vec<u8>) {
        self.time += 4;
        self.pc += 2;

        if (self.ram.load(self.hl()) & 0x80) == 0x80 {
            self.set_c_flag(true);
        } else {
            self.set_c_flag(false);
        }

        self.ram.store(self.hl(), self.ram.load(self.hl()).rotate_left(1));

        if self.ram.load(self.hl()) == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
    }

    fn rrc_b(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 2;

        if (self.b() & 0x01) == 0x01 {
            self.set_c_flag(true);
        } else {
            self.set_c_flag(false);
        }

        self.set_b(self.b().rotate_right(1));

        if self.b() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
    }

    fn rrc_c(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 2;

        if (self.c() & 0x01) == 0x01 {
            self.set_c_flag(true);
        } else {
            self.set_c_flag(false);
        }

        self.set_c(self.c().rotate_right(1));

        if self.c() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
    }

    fn rrc_d(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 2;

        if (self.d() & 0x01) == 0x01 {
            self.set_c_flag(true);
        } else {
            self.set_c_flag(false);
        }

        self.set_d(self.d().rotate_right(1));

        if self.d() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
    }

    fn rrc_e(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 2;

        if (self.e() & 0x01) == 0x01 {
            self.set_c_flag(true);
        } else {
            self.set_c_flag(false);
        }

        self.set_e(self.e().rotate_right(1));

        if self.e() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
    }

    fn rrc_h(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 2;

        if (self.h() & 0x01) == 0x01 {
            self.set_c_flag(true);
        } else {
            self.set_c_flag(false);
        }

        self.set_h(self.h().rotate_right(1));

        if self.h() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
    }

    fn rrc_l(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 2;

        if (self.l() & 0x01) == 0x01 {
            self.set_c_flag(true);
        } else {
            self.set_c_flag(false);
        }

        self.set_l(self.l().rotate_right(1));

        if self.l() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
    }

    fn rrc_a(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 2;

        if (self.a() & 0x01) == 0x01 {
            self.set_c_flag(true);
        } else {
            self.set_c_flag(false);
        }

        self.set_a(self.a().rotate_right(1));

        if self.a() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
    }

    fn rrc_hl(&mut self, operands: Vec<u8>) {
        self.time += 4;
        self.pc += 2;

        if (self.ram.load(self.hl()) & 0x01) == 0x01 {
            self.set_c_flag(true);
        } else {
            self.set_c_flag(false);
        }

        self.ram.store(self.hl(), self.ram.load(self.hl()).rotate_right(1));

        if self.ram.load(self.hl()) == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
    }

    fn rl_b(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 2;

        let c = self.c_flag();
        if (self.b() & 0x80) == 0x80 {
            self.set_c_flag(true);
        } else {
            self.set_c_flag(false);
        }

        self.set_b(self.b().rotate_left(1));
        if c == 1 {
            self.set_b(self.b() | 0x01);
        } else {
            self.set_b(self.b() & 0xfe);
        }

        if self.b() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
    }

    fn rl_c(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 2;

        let c = self.c_flag();
        if (self.c() & 0x80) == 0x80 {
            self.set_c_flag(true);
        } else {
            self.set_c_flag(false);
        }

        self.set_c(self.c().rotate_left(1));
        if c == 1 {
            self.set_c(self.c() | 0x01);
        } else {
            self.set_c(self.c() & 0xfe);
        }

        if self.c() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
    }

    fn rl_d(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 2;

        let c = self.c_flag();
        if (self.d() & 0x80) == 0x80 {
            self.set_c_flag(true);
        } else {
            self.set_c_flag(false);
        }

        self.set_d(self.d().rotate_left(1));
        if c == 1 {
            self.set_d(self.d() | 0x01);
        } else {
            self.set_d(self.d() & 0xfe);
        }

        if self.d() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
    }

    fn rl_e(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 2;

        let c = self.c_flag();
        if (self.e() & 0x80) == 0x80 {
            self.set_c_flag(true);
        } else {
            self.set_c_flag(false);
        }

        self.set_e(self.e().rotate_left(1));
        if c == 1 {
            self.set_e(self.e() | 0x01);
        } else {
            self.set_e(self.e() & 0xfe);
        }

        if self.e() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
    }

    fn rl_h(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 2;

        let c = self.c_flag();
        if (self.h() & 0x80) == 0x80 {
            self.set_c_flag(true);
        } else {
            self.set_c_flag(false);
        }

        self.set_h(self.h().rotate_left(1));
        if c == 1 {
            self.set_h(self.h() | 0x01);
        } else {
            self.set_h(self.h() & 0xfe);
        }

        if self.h() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
    }

    fn rl_l(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 2;

        let c = self.c_flag();
        if (self.l() & 0x80) == 0x80 {
            self.set_c_flag(true);
        } else {
            self.set_c_flag(false);
        }

        self.set_l(self.l().rotate_left(1));
        if c == 1 {
            self.set_l(self.l() | 0x01);
        } else {
            self.set_l(self.l() & 0xfe);
        }

        if self.l() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
    }

    fn rl_a(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 2;

        let c = self.c_flag();
        if (self.a() & 0x80) == 0x80 {
            self.set_c_flag(true);
        } else {
            self.set_c_flag(false);
        }

        self.set_a(self.a().rotate_left(1));
        if c == 1 {
            self.set_a(self.a() | 0x01);
        } else {
            self.set_a(self.a() & 0xfe);
        }

        if self.a() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
    }

    fn rl_hl(&mut self, operands: Vec<u8>) {
        self.time += 4;
        self.pc += 2;

        let c = self.c_flag();
        if (self.ram.load(self.hl()) & 0x80) == 0x80 {
            self.set_c_flag(true);
        } else {
            self.set_c_flag(false);
        }

        self.ram.store(self.hl(), self.ram.load(self.hl()).rotate_left(1));
        if c == 1 {
            self.ram.store(self.hl(), self.ram.load(self.hl()) | 0x01);
        } else {
            self.ram.store(self.hl(), self.ram.load(self.hl()) & 0xfe);
        }

        if self.ram.load(self.hl()) == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
    }

    fn rr_b(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 2;

        let c = self.c_flag();
        if (self.b() & 0x01) == 0x01 {
            self.set_c_flag(true);
        } else {
            self.set_c_flag(false);
        }

        self.set_b(self.b().rotate_right(1));
        if c == 1 {
            self.set_b(self.b() | 0x80);
        } else {
            self.set_b(self.b() & 0x7f);
        }

        if self.b() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
    }

    fn rr_c(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 2;

        let c = self.c_flag();
        if (self.c() & 0x01) == 0x01 {
            self.set_c_flag(true);
        } else {
            self.set_c_flag(false);
        }

        self.set_c(self.c().rotate_right(1));
        if c == 1 {
            self.set_c(self.c() | 0x80);
        } else {
            self.set_c(self.c() & 0x7f);
        }

        if self.c() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
    }

    fn rr_d(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 2;

        let c = self.c_flag();
        if (self.d() & 0x01) == 0x01 {
            self.set_c_flag(true);
        } else {
            self.set_c_flag(false);
        }

        self.set_d(self.d().rotate_right(1));
        if c == 1 {
            self.set_d(self.d() | 0x80);
        } else {
            self.set_d(self.d() & 0x7f);
        }

        if self.d() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
    }

    fn rr_e(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 2;

        let c = self.c_flag();
        if (self.e() & 0x01) == 0x01 {
            self.set_c_flag(true);
        } else {
            self.set_c_flag(false);
        }

        self.set_e(self.e().rotate_right(1));
        if c == 1 {
            self.set_e(self.e() | 0x80);
        } else {
            self.set_e(self.e() & 0x7f);
        }

        if self.e() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
    }

    fn rr_h(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 2;

        let c = self.c_flag();
        if (self.h() & 0x01) == 0x01 {
            self.set_c_flag(true);
        } else {
            self.set_c_flag(false);
        }

        self.set_h(self.h().rotate_right(1));
        if c == 1 {
            self.set_h(self.h() | 0x80);
        } else {
            self.set_h(self.h() & 0x7f);
        }

        if self.h() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
    }

    fn rr_l(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 2;

        let c = self.c_flag();
        if (self.l() & 0x01) == 0x01 {
            self.set_c_flag(true);
        } else {
            self.set_c_flag(false);
        }

        self.set_l(self.l().rotate_right(1));
        if c == 1 {
            self.set_l(self.l() | 0x80);
        } else {
            self.set_l(self.l() & 0x7f);
        }

        if self.l() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
    }

    fn rr_a(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 2;

        let c = self.c_flag();
        if (self.a() & 0x01) == 0x01 {
            self.set_c_flag(true);
        } else {
            self.set_c_flag(false);
        }

        self.set_a(self.a().rotate_right(1));
        if c == 1 {
            self.set_a(self.a() | 0x80);
        } else {
            self.set_a(self.a() & 0x7f);
        }

        if self.a() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
    }

    fn rr_hl(&mut self, operands: Vec<u8>) {
        self.time += 4;
        self.pc += 2;

        let c = self.c_flag();
        if (self.ram.load(self.hl()) & 0x01) == 0x01 {
            self.set_c_flag(true);
        } else {
            self.set_c_flag(false);
        }

        self.ram.store(self.hl(), self.ram.load(self.hl()).rotate_right(1));
        if c == 1 {
            self.ram.store(self.hl(), self.ram.load(self.hl()) | 0x80);
        } else {
            self.ram.store(self.hl(), self.ram.load(self.hl()) & 0x7f);
        }

        if self.ram.load(self.hl()) == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
    }

    fn sla_b(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 2;

        if (self.b() & 0x80) == 0x80 {
            self.set_c_flag(true);
        } else {
            self.set_c_flag(false);
        }

        self.set_b(self.b().rotate_left(1));
        self.set_b(self.b() & 0xfe);

        if self.b() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
    }

    fn sla_c(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 2;

        if (self.c() & 0x80) == 0x80 {
            self.set_c_flag(true);
        } else {
            self.set_c_flag(false);
        }

        self.set_c(self.c().rotate_left(1));
        self.set_c(self.c() & 0xfe);

        if self.c() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
    }

    fn sla_d(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 2;

        if (self.d() & 0x80) == 0x80 {
            self.set_c_flag(true);
        } else {
            self.set_c_flag(false);
        }

        self.set_d(self.d().rotate_left(1));
        self.set_d(self.d() & 0xfe);

        if self.d() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
    }

    fn sla_e(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 2;

        if (self.e() & 0x80) == 0x80 {
            self.set_c_flag(true);
        } else {
            self.set_c_flag(false);
        }

        self.set_e(self.e().rotate_left(1));
        self.set_e(self.e() & 0xfe);

        if self.e() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
    }

    fn sla_h(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 2;

        if (self.h() & 0x80) == 0x80 {
            self.set_c_flag(true);
        } else {
            self.set_c_flag(false);
        }

        self.set_h(self.h().rotate_left(1));
        self.set_h(self.h() & 0xfe);

        if self.h() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
    }

    fn sla_l(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 2;

        if (self.l() & 0x80) == 0x80 {
            self.set_c_flag(true);
        } else {
            self.set_c_flag(false);
        }

        self.set_l(self.l().rotate_left(1));
        self.set_l(self.l() & 0xfe);

        if self.l() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
    }

    fn sla_a(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 2;

        if (self.a() & 0x80) == 0x80 {
            self.set_c_flag(true);
        } else {
            self.set_c_flag(false);
        }

        self.set_a(self.a().rotate_left(1));
        self.set_a(self.a() & 0xfe);

        if self.a() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
    }

    fn sla_hl(&mut self, operands: Vec<u8>) {
        self.time += 4;
        self.pc += 2;

        if (self.ram.load(self.hl()) & 0x80) == 0x80 {
            self.set_c_flag(true);
        } else {
            self.set_c_flag(false);
        }

        self.ram.store(self.hl(), self.ram.load(self.hl()).rotate_left(1));
        self.ram.store(self.hl(), self.ram.load(self.hl()) & 0xfe);

        if self.ram.load(self.hl()) == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
    }

    fn sra_b(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 2;

        if (self.b() & 0x01) == 0x01 {
            self.set_c_flag(true);
        } else {
            self.set_c_flag(false);
        }

        let bit_7 = if (self.b() & 0x80) == 0x80 {
            1
        } else {
            0
        };
        self.set_b(self.b().rotate_right(1));
        if bit_7 == 1 {
            self.set_b(self.b() | 0x80);
        } else {
            self.set_b(self.b() & 0x7f);
        }

        if self.b() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
    }

    fn sra_c(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 2;

        if (self.c() & 0x01) == 0x01 {
            self.set_c_flag(true);
        } else {
            self.set_c_flag(false);
        }

        let bit_7 = if (self.c() & 0x80) == 0x80 {
            1
        } else {
            0
        };
        self.set_c(self.c().rotate_right(1));
        if bit_7 == 1 {
            self.set_c(self.c() | 0x80);
        } else {
            self.set_c(self.c() & 0x7f);
        }

        if self.c() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
    }

    fn sra_d(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 2;

        if (self.d() & 0x01) == 0x01 {
            self.set_c_flag(true);
        } else {
            self.set_c_flag(false);
        }

        let bit_7 = if (self.d() & 0x80) == 0x80 {
            1
        } else {
            0
        };
        self.set_d(self.d().rotate_right(1));
        if bit_7 == 1 {
            self.set_d(self.d() | 0x80);
        } else {
            self.set_d(self.d() & 0x7f);
        }

        if self.d() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
    }

    fn sra_e(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 2;

        if (self.e() & 0x01) == 0x01 {
            self.set_c_flag(true);
        } else {
            self.set_c_flag(false);
        }

        let bit_7 = if (self.e() & 0x80) == 0x80 {
            1
        } else {
            0
        };
        self.set_e(self.e().rotate_right(1));
        if bit_7 == 1 {
            self.set_e(self.e() | 0x80);
        } else {
            self.set_e(self.e() & 0x7f);
        }

        if self.e() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
    }

    fn sra_h(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 2;

        if (self.h() & 0x01) == 0x01 {
            self.set_c_flag(true);
        } else {
            self.set_c_flag(false);
        }

        let bit_7 = if (self.h() & 0x80) == 0x80 {
            1
        } else {
            0
        };
        self.set_h(self.h().rotate_right(1));
        if bit_7 == 1 {
            self.set_h(self.h() | 0x80);
        } else {
            self.set_h(self.h() & 0x7f);
        }

        if self.h() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
    }

    fn sra_l(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 2;

        if (self.l() & 0x01) == 0x01 {
            self.set_c_flag(true);
        } else {
            self.set_c_flag(false);
        }

        let bit_7 = if (self.l() & 0x80) == 0x80 {
            1
        } else {
            0
        };
        self.set_l(self.l().rotate_right(1));
        if bit_7 == 1 {
            self.set_l(self.l() | 0x80);
        } else {
            self.set_l(self.l() & 0x7f);
        }

        if self.l() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
    }

    fn sra_a(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 2;

        if (self.a() & 0x01) == 0x01 {
            self.set_c_flag(true);
        } else {
            self.set_c_flag(false);
        }

        let bit_7 = if (self.a() & 0x80) == 0x80 {
            1
        } else {
            0
        };
        self.set_a(self.a().rotate_right(1));
        if bit_7 == 1 {
            self.set_a(self.a() | 0x80);
        } else {
            self.set_a(self.a() & 0x7f);
        }

        if self.a() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
    }

    fn sra_hl(&mut self, operands: Vec<u8>) {
        self.time += 4;
        self.pc += 2;

        if (self.ram.load(self.hl()) & 0x01) == 0x01 {
            self.set_c_flag(true);
        } else {
            self.set_c_flag(false);
        }

        let bit_7 = if (self.ram.load(self.hl()) & 0x80) == 0x80 {
            1
        } else {
            0
        };
        self.ram.store(self.hl(), self.ram.load(self.hl()).rotate_right(1));
        if bit_7 == 1 {
            self.ram.store(self.hl(), self.ram.load(self.hl()) | 0x80);
        } else {
            self.ram.store(self.hl(), self.ram.load(self.hl()) & 0x7f);
        }

        if self.ram.load(self.hl()) == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
    }

    fn swap_b(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 2;

        self.set_b((self.b() << 4) | (self.b() >> 4));

        if self.b() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
        self.set_c_flag(false);
    }

    fn swap_c(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 2;

        self.set_c((self.c() << 4) | (self.c() >> 4));

        if self.c() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
        self.set_c_flag(false);
    }

    fn swap_d(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 2;

        self.set_d((self.d() << 4) | (self.d() >> 4));

        if self.d() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
        self.set_c_flag(false);
    }

    fn swap_e(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 2;

        self.set_e((self.e() << 4) | (self.e() >> 4));

        if self.e() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
        self.set_c_flag(false);
    }

    fn swap_h(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 2;

        self.set_h((self.h() << 4) | (self.h() >> 4));

        if self.h() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
        self.set_c_flag(false);
    }

    fn swap_l(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 2;

        self.set_l((self.l() << 4) | (self.l() >> 4));

        if self.l() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
        self.set_c_flag(false);
    }

    fn swap_a(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 2;

        self.set_a((self.a() << 4) | (self.a() >> 4));

        if self.a() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
        self.set_c_flag(false);
    }

    fn swap_hl(&mut self, operands: Vec<u8>) {
        self.time += 4;
        self.pc += 2;

        self.ram.store(self.hl(), (self.ram.load(self.hl()) << 4) | (self.ram.load(self.hl() >> 4)));

        if self.ram.load(self.hl()) == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
        self.set_c_flag(false);
    }

    fn srl_b(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 2;

        if (self.b() & 0x01) == 0x01 {
            self.set_c_flag(true);
        } else {
            self.set_c_flag(false);
        }

        self.set_b(self.b() >> 1);

        if self.b() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
    }

    fn srl_c(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 2;

        if (self.c() & 0x01) == 0x01 {
            self.set_c_flag(true);
        } else {
            self.set_c_flag(false);
        }

        self.set_c(self.c() >> 1);

        if self.c() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
    }

    fn srl_d(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 2;

        if (self.d() & 0x01) == 0x01 {
            self.set_c_flag(true);
        } else {
            self.set_c_flag(false);
        }

        self.set_d(self.d() >> 1);

        if self.d() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
    }

    fn srl_e(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 2;

        if (self.e() & 0x01) == 0x01 {
            self.set_c_flag(true);
        } else {
            self.set_c_flag(false);
        }

        self.set_e(self.e() >> 1);

        if self.e() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
    }

    fn srl_h(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 2;

        if (self.h() & 0x01) == 0x01 {
            self.set_c_flag(true);
        } else {
            self.set_c_flag(false);
        }

        self.set_h(self.h() >> 1);

        if self.h() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
    }

    fn srl_l(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 2;

        if (self.l() & 0x01) == 0x01 {
            self.set_c_flag(true);
        } else {
            self.set_c_flag(false);
        }

        self.set_l(self.l() >> 1);

        if self.l() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
    }

    fn srl_a(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 2;

        if (self.a() & 0x01) == 0x01 {
            self.set_c_flag(true);
        } else {
            self.set_c_flag(false);
        }

        self.set_a(self.a() >> 1);

        if self.a() == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
    }

    fn srl_hl(&mut self, operands: Vec<u8>) {
        self.time += 4;
        self.pc += 2;

        if (self.ram.load(self.hl()) & 0x01) == 0x01 {
            self.set_c_flag(true);
        } else {
            self.set_c_flag(false);
        }

        self.ram.store(self.hl(), self.ram.load(self.hl()) >> 1);

        if self.ram.load(self.hl()) == 0 {
            self.set_z_flag(true);
        } else {
            self.set_z_flag(false);
        }
        self.set_n_flag(false);
        self.set_h_flag(false);
    }

    fn bit_b(&mut self, operands: Vec<u8>, mask: u8) {
        self.time += 2;
        self.pc += 2;

        if (self.b() & mask) == mask {
            self.set_z_flag(false);
        } else {
            self.set_z_flag(true);
        }
        self.set_n_flag(false);
        self.set_h_flag(true);
    }

    fn bit_c(&mut self, operands: Vec<u8>, mask: u8) {
        self.time += 2;
        self.pc += 2;

        if (self.c() & mask) == mask {
            self.set_z_flag(false);
        } else {
            self.set_z_flag(true);
        }
        self.set_n_flag(false);
        self.set_h_flag(true);
    }

    fn bit_d(&mut self, operands: Vec<u8>, mask: u8) {
        self.time += 2;
        self.pc += 2;

        if (self.d() & mask) == mask {
            self.set_z_flag(false);
        } else {
            self.set_z_flag(true);
        }
        self.set_n_flag(false);
        self.set_h_flag(true);
    }

    fn bit_e(&mut self, operands: Vec<u8>, mask: u8) {
        self.time += 2;
        self.pc += 2;

        if (self.e() & mask) == mask {
            self.set_z_flag(false);
        } else {
            self.set_z_flag(true);
        }
        self.set_n_flag(false);
        self.set_h_flag(true);
    }

    fn bit_h(&mut self, operands: Vec<u8>, mask: u8) {
        self.time += 2;
        self.pc += 2;

        if (self.h() & mask) == mask {
            self.set_z_flag(false);
        } else {
            self.set_z_flag(true);
        }
        self.set_n_flag(false);
        self.set_h_flag(true);
    }

    fn bit_l(&mut self, operands: Vec<u8>, mask: u8) {
        self.time += 2;
        self.pc += 2;

        if (self.l() & mask) == mask {
            self.set_z_flag(false);
        } else {
            self.set_z_flag(true);
        }
        self.set_n_flag(false);
        self.set_h_flag(true);
    }

    fn bit_a(&mut self, operands: Vec<u8>, mask: u8) {
        self.time += 2;
        self.pc += 2;

        if (self.a() & mask) == mask {
            self.set_z_flag(false);
        } else {
            self.set_z_flag(true);
        }
        self.set_n_flag(false);
        self.set_h_flag(true);
    }

    fn bit_hl(&mut self, operands: Vec<u8>, mask: u8) {
        self.time += 3;
        self.pc += 2;

        if (self.ram.load(self.hl()) & mask) == mask {
            self.set_z_flag(false);
        } else {
            self.set_z_flag(true);
        }
        self.set_n_flag(false);
        self.set_h_flag(true);
    }

    fn res_b(&mut self, operands: Vec<u8>, mask: u8) {
        self.time += 2;
        self.pc += 2;

        self.set_b(self.b() & !mask);
    }

    fn res_c(&mut self, operands: Vec<u8>, mask: u8) {
        self.time += 2;
        self.pc += 2;

        self.set_c(self.c() & !mask);
    }

    fn res_d(&mut self, operands: Vec<u8>, mask: u8) {
        self.time += 2;
        self.pc += 2;

        self.set_d(self.d() & !mask);
    }

    fn res_e(&mut self, operands: Vec<u8>, mask: u8) {
        self.time += 2;
        self.pc += 2;

        self.set_e(self.e() & !mask);
    }

    fn res_h(&mut self, operands: Vec<u8>, mask: u8) {
        self.time += 2;
        self.pc += 2;

        self.set_h(self.h() & !mask);
    }

    fn res_l(&mut self, operands: Vec<u8>, mask: u8) {
        self.time += 2;
        self.pc += 2;

        self.set_l(self.l() & !mask);
    }

    fn res_a(&mut self, operands: Vec<u8>, mask: u8) {
        self.time += 2;
        self.pc += 2;

        self.set_a(self.a() & !mask);
    }

    fn res_hl(&mut self, operands: Vec<u8>, mask: u8) {
        self.time += 4;
        self.pc += 2;

        self.ram.store(self.hl(), self.ram.load(self.hl()) & !mask);
    }

    fn set_bit_b(&mut self, operands: Vec<u8>, mask: u8) {
        self.time += 2;
        self.pc += 2;

        self.set_b(self.b() | mask);
    }

    fn set_bit_c(&mut self, operands: Vec<u8>, mask: u8) {
        self.time += 2;
        self.pc += 2;

        self.set_c(self.c() | mask);
    }

    fn set_bit_d(&mut self, operands: Vec<u8>, mask: u8) {
        self.time += 2;
        self.pc += 2;

        self.set_d(self.d() | mask);
    }

    fn set_bit_e(&mut self, operands: Vec<u8>, mask: u8) {
        self.time += 2;
        self.pc += 2;

        self.set_e(self.e() | mask);
    }

    fn set_bit_h(&mut self, operands: Vec<u8>, mask: u8) {
        self.time += 2;
        self.pc += 2;

        self.set_h(self.h() | mask);
    }

    fn set_bit_l(&mut self, operands: Vec<u8>, mask: u8) {
        self.time += 2;
        self.pc += 2;

        self.set_l(self.l() | mask);
    }

    fn set_bit_a(&mut self, operands: Vec<u8>, mask: u8) {
        self.time += 2;
        self.pc += 2;

        self.set_a(self.a() | mask);
    }

    fn set_bit_hl(&mut self, operands: Vec<u8>, mask: u8) {
        self.time += 4;
        self.pc += 2;

        self.ram.store(self.hl(), self.ram.load(self.hl()) | mask);
    }
}

enum Instruction {
    Instruction8 {
        opcode: u8,
        operands: Vec<u8>,
    },
    Instruction16 {
        opcode: u16,
        operands: Vec<u8>,
    },
}
