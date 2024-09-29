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
                    0x03 => todo!(),
                    0x04 => todo!(),
                    0x05 => todo!(),
                    0x06 => self.ld_b_d8(operands),
                    0x07 => todo!(),
                    0x08 => self.ld_a16_sp(operands),
                    0x09 => todo!(),
                    0x0a => self.ld_a_bc(operands),
                    0x0b => todo!(),
                    0x0c => todo!(),
                    0x0d => todo!(),
                    0x0e => self.ld_c_d8(operands),
                    0x0f => todo!(),
                    0x10 => todo!(),
                    0x11 => self.ld_de_d16(operands),
                    0x12 => self.ld_de_a(operands),
                    0x13 => todo!(),
                    0x14 => todo!(),
                    0x15 => todo!(),
                    0x16 => self.ld_d_d8(operands),
                    0x17 => todo!(),
                    0x18 => todo!(),
                    0x19 => todo!(),
                    0x1a => self.ld_a_de(operands),
                    0x1b => todo!(),
                    0x1c => todo!(),
                    0x1d => todo!(),
                    0x1e => self.ld_e_d8(operands),
                    0x1f => todo!(),
                    0x20 => todo!(),
                    0x21 => self.ld_hl_d16(operands),
                    0x22 => self.ld_hl_plus_a(operands),
                    0x23 => todo!(),
                    0x24 => todo!(),
                    0x25 => todo!(),
                    0x26 => self.ld_h_d8(operands),
                    0x27 => todo!(),
                    0x28 => todo!(),
                    0x29 => todo!(),
                    0x2a => self.ld_a_hl_plus(operands),
                    0x2b => todo!(),
                    0x2c => todo!(),
                    0x2d => todo!(),
                    0x2e => self.ld_l_d8(operands),
                    0x2f => todo!(),
                    0x30 => todo!(),
                    0x31 => self.ld_sp_d16(operands),
                    0x32 => self.ld_hl_minus_a(operands),
                    0x33 => todo!(),
                    0x34 => todo!(),
                    0x35 => todo!(),
                    0x36 => self.ld_hl_d8(operands),
                    0x37 => todo!(),
                    0x38 => todo!(),
                    0x39 => todo!(),
                    0x3a => self.ld_a_hl_minus(operands),
                    0x3b => todo!(),
                    0x3c => todo!(),
                    0x3d => todo!(),
                    0x3e => self.ld_a_d8(operands),
                    0x3f => todo!(),
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
                    0x80 => todo!(),
                    0x81 => todo!(),
                    0x82 => todo!(),
                    0x83 => todo!(),
                    0x84 => todo!(),
                    0x85 => todo!(),
                    0x86 => todo!(),
                    0x87 => todo!(),
                    0x88 => todo!(),
                    0x89 => todo!(),
                    0x8a => todo!(),
                    0x8b => todo!(),
                    0x8c => todo!(),
                    0x8d => todo!(),
                    0x8e => todo!(),
                    0x8f => todo!(),
                    0x90 => todo!(),
                    0x91 => todo!(),
                    0x92 => todo!(),
                    0x93 => todo!(),
                    0x94 => todo!(),
                    0x95 => todo!(),
                    0x96 => todo!(),
                    0x97 => todo!(),
                    0x98 => todo!(),
                    0x99 => todo!(),
                    0x9a => todo!(),
                    0x9b => todo!(),
                    0x9c => todo!(),
                    0x9d => todo!(),
                    0x9e => todo!(),
                    0x9f => todo!(),
                    0xa0 => todo!(),
                    0xa1 => todo!(),
                    0xa2 => todo!(),
                    0xa3 => todo!(),
                    0xa4 => todo!(),
                    0xa5 => todo!(),
                    0xa6 => todo!(),
                    0xa7 => todo!(),
                    0xa8 => todo!(),
                    0xa9 => todo!(),
                    0xaa => todo!(),
                    0xab => todo!(),
                    0xac => todo!(),
                    0xad => todo!(),
                    0xae => todo!(),
                    0xaf => todo!(),
                    0xb0 => todo!(),
                    0xb1 => todo!(),
                    0xb2 => todo!(),
                    0xb3 => todo!(),
                    0xb4 => todo!(),
                    0xb5 => todo!(),
                    0xb6 => todo!(),
                    0xb7 => todo!(),
                    0xb8 => todo!(),
                    0xb9 => todo!(),
                    0xba => todo!(),
                    0xbb => todo!(),
                    0xbc => todo!(),
                    0xbd => todo!(),
                    0xbe => todo!(),
                    0xbf => todo!(),
                    0xc0 => todo!(),
                    0xc1 => self.pop_bc(operands),
                    0xc2 => todo!(),
                    0xc3 => todo!(),
                    0xc4 => todo!(),
                    0xc5 => self.push_bc(operands),
                    0xc6 => todo!(),
                    0xc7 => todo!(),
                    0xc8 => todo!(),
                    0xc9 => todo!(),
                    0xca => todo!(),
                    0xcb => todo!(),
                    0xcc => todo!(),
                    0xcd => todo!(),
                    0xce => todo!(),
                    0xcf => todo!(),
                    0xd0 => todo!(),
                    0xd1 => self.pop_de(operands),
                    0xd2 => todo!(),
                    0xd3 => todo!(),
                    0xd4 => todo!(),
                    0xd5 => self.push_de(operands),
                    0xd6 => todo!(),
                    0xd7 => todo!(),
                    0xd8 => todo!(),
                    0xd9 => todo!(),
                    0xda => todo!(),
                    0xdb => todo!(),
                    0xdc => todo!(),
                    0xdd => todo!(),
                    0xde => todo!(),
                    0xdf => todo!(),
                    0xe0 => self.ldh_a8_a(operands),
                    0xe1 => self.pop_hl(operands),
                    0xe2 => self.ldh_c_a(operands),
                    0xe3 => todo!(),
                    0xe4 => todo!(),
                    0xe5 => self.push_hl(operands),
                    0xe6 => todo!(),
                    0xe7 => todo!(),
                    0xe8 => todo!(),
                    0xe9 => todo!(),
                    0xea => self.ld_a16_a(operands),
                    0xeb => todo!(),
                    0xec => todo!(),
                    0xed => todo!(),
                    0xee => todo!(),
                    0xef => todo!(),
                    0xf0 => self.ldh_a_a8(operands),
                    0xf1 => self.pop_af(operands),
                    0xf2 => self.ldh_a_c(operands),
                    0xf3 => todo!(),
                    0xf4 => todo!(),
                    0xf5 => self.push_af(operands),
                    0xf6 => todo!(),
                    0xf7 => todo!(),
                    0xf8 => self.ld_hl_sp_plus_s8(operands),
                    0xf9 => self.ld_sp_hl(operands),
                    0xfa => self.ld_a_a16(operands),
                    0xfb => todo!(),
                    0xfc => todo!(),
                    0xfd => todo!(),
                    0xfe => todo!(),
                    0xff => todo!(),
                    _ => (),
                }
            },
            Instruction::Instruction16 { opcode, operands } => {
                match opcode {
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

    fn set_b(&mut self, val: u8) {
        self.b = val;
    }

    fn set_c(&mut self, val: u8) {
        self.c = val;
    }

    fn set_d(&mut self, val: u8) {
        self.d = val;
    }

    fn set_e(&mut self, val: u8) {
        self.e = val;
    }

    fn set_h(&mut self, val: u8) {
        self.h = val;
    }

    fn set_l(&mut self, val: u8) {
        self.l = val;
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
