//Useful links:
//https://meganesulli.com/blog/game-boy-opcodes/
//https://github.com/gbdev/awesome-gbdev

use super::ram::RAM;

#[derive(Clone)]
struct CPU {
    af: [u8; 2],
    bc: [u8; 2],
    de: [u8; 2],
    hl: [u8; 2],
    sp: [u8; 2],
    pc: u16,
    time: u64,

    ram: RAM,
}

impl CPU {
    fn new(ram: RAM) -> Self {
        Self {
            af: [0; 2],
            bc: [0; 2],
            de: [0; 2],
            hl: [0; 2],
            sp: [0; 2],
            pc: 0,
            time: 0,

            ram,
        }
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Instruction8 { opcode, operands } => {
                match opcode {
                    0x00 => self.nop(),
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
                    //0x1000 => _self.stop(operands),
                    0x11 => self.ld_de_d16(operands),
                    _ => (),
                }
            },
            Instruction::Instruction16 { opcode, operands } => {

            },
        }
    }

    fn af(&self) -> u16 {
        u16::from_le_bytes(self.af)
    }

    fn set_af(&mut self, val: u16) {
        self.af = val.to_le_bytes();
    }

    fn a(&self) -> u8 {
        self.af[1]
    }

    fn set_a(&mut self, val: u8) {
        self.af[1] = val;
    }

    fn f(&self) -> u8 {
        self.af[0]
    }

    fn set_f(&mut self, val: u8) {
        self.af[0] = val;
    }

    fn set_z_flag(&mut self) {
        self.af[0] |= 0x80;
    }

    fn unset_z_flag(&mut self) {
        self.af[0] &= 0x7f;
    }

    fn set_n_flag(&mut self) {
        self.af[0] |= 0x40;
    }

    fn unset_n_flag(&mut self) {
        self.af[0] &= 0xbf;
    }

    fn set_h_flag(&mut self) {
        self.af[0] |= 0x20;
    }

    fn unset_h_flag(&mut self) {
        self.af[0] &= 0xdf;
    }

    fn set_c_flag(&mut self) {
        self.af[0] |= 0x10;
    }

    fn unset_c_flag(&mut self) {
        self.af[0] &= 0xef;
    }

    fn b(&self) -> u8 {
        self.bc[1]
    }

    fn bc(&self) -> u16 {
        u16::from_le_bytes(self.bc)
    }

    fn set_bc(&mut self, val: u16) {
        self.bc = val.to_le_bytes();
    }

    fn set_b(&mut self, val: u8) {
        self.bc[1] = val;
    }

    fn c(&self) -> u8 {
        self.bc[0]
    }

    fn set_c(&mut self, val: u8) {
        self.bc[0] = val;
    }

    fn de(&self) -> u16 {
        u16::from_le_bytes(self.de)
    }

    fn set_de(&mut self, val: u16) {
        self.de = val.to_le_bytes();
    }

    fn d(&self) -> u8 {
        self.de[1]
    }

    fn set_d(&mut self, val: u8) {
        self.de[1] = val;
    }

    fn e(&self) -> u8 {
        self.de[0]
    }

    fn set_e(&mut self, val: u8) {
        self.de[0] = val;
    }

    fn hl(&self) -> u16 {
        u16::from_le_bytes(self.hl)
    }

    fn set_hl(&mut self, val: u16) {
        self.hl = val.to_le_bytes();
    }

    fn h(&self) -> u8 {
        self.hl[1]
    }

    fn set_h(&mut self, val: u8) {
        self.hl[1] = val;
    }

    fn l(&self) -> u8 {
        self.hl[0]
    }

    fn set_l(&mut self, val: u8) {
        self.hl[0] = val;
    }

    fn nop(&mut self) {
        self.time += 1;
        self.pc += 1;
    }

    fn ld_bc_d16(&mut self, operands: Vec<u8>) {
        self.time += 3;
        self.pc += 3;

        //NOTE:Not sure about this instruction
        self.set_b(operands[0]);
        self.set_c(operands[1]);
    }

    fn ld_bc_a(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 1;

        self.ram.store(self.bc(), self.a());
    }

    fn inc_bc(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 1;

        self.set_bc(self.bc() + 1);
    }

    fn inc_b(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        if (((self.b() & 0xf) + (1 & 0xf)) & 0x10) == 0x10 {
            self.set_h_flag();
        } else {
            self.unset_h_flag();
        }

        if self.b() == 0xff {
            self.set_b(0x00);
        } else {
            self.set_b(self.b() + 1);
        }

        if self.b() == 0 {
            self.set_z_flag();
        } else {
            self.unset_z_flag();
        }
        self.unset_n_flag();
    }

    fn dec_b(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        if (((self.b() & 0xf) + (1 & 0xf)) & 0x10) == 0x10 {
            self.set_h_flag();
        } else {
            self.unset_h_flag();
        }

        if self.b() == 0x00 {
            self.set_b(0xff);
        } else {
            self.set_b(self.b() - 1);
        }

        if self.b() == 0 {
            self.set_z_flag();
        } else {
            self.unset_z_flag();
        }
        self.set_n_flag();
    }

    fn ld_b_d8(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 2;

        self.set_b(operands[0]);
    }

    fn rlca(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.unset_z_flag();
        self.unset_n_flag();
        self.unset_h_flag();
        if self.a() & 0x80 == 0 {
            self.unset_c_flag();
        } else {
            self.set_c_flag();
        }

        self.set_a(self.a().rotate_left(1));
    }

    fn ld_a16_sp(&mut self, operands: Vec<u8>) {
        self.time += 5;
        self.pc += 3;

        //NOTE:Not sure about this function
        self.ram.store(u16::from_le_bytes([operands[1], operands[0]]), self.sp[0]);
        self.ram.store(u16::from_le_bytes([operands[1] + 1, operands[0]]), self.sp[1]);
    }

    fn add_hl_bc(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 1;

        if (((self.hl() & 0xfff) + (self.bc() & 0xfff)) & 0x1000) == 0x1000 {
            self.set_h_flag();
        } else {
            self.unset_h_flag();
        }
        if ((self.hl() & 0x8000) == 0x8000) && ((self.bc() & 0x8000) == 0x8000) {
            self.set_c_flag();
        } else {
            self.unset_c_flag();
        }
        self.unset_n_flag();

        self.set_hl(self.hl().wrapping_add(self.bc()));
    }

    fn ld_a_bc(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 1;

        self.set_a(self.ram.load(self.bc()));
    }

    fn dec_bc(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 1;

        self.set_bc(self.bc() - 1);
    }

    fn inc_c(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        if (((self.c() & 0xf) + (0x01 & 0xf)) & 0x10) == 0x10 {
            self.set_h_flag();
        } else {
            self.unset_h_flag();
        }

        if self.c() == 0xff {
            self.set_c(0x00);
        } else {
            self.set_c(self.c() + 1);
        }

        if self.c() == 0 {
            self.set_z_flag();
        } else {
            self.unset_z_flag();
        }
        self.unset_n_flag();
    }

    fn dec_c(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        if (((self.c() & 0xf) + (0x01 & 0xf)) & 0x10) == 0x10 {
            self.set_h_flag();
        } else {
            self.unset_h_flag();
        }

        if self.c() == 0x00 {
            self.set_c(0xff);
        } else {
            self.set_c(self.c() - 1);
        }

        if self.c() == 0 {
            self.set_z_flag();
        } else {
            self.unset_z_flag();
        }
        self.set_n_flag();
    }

    fn ld_c_d8(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 2;

        self.set_c(operands[0]);
    }

    fn rrca(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        self.unset_z_flag();
        self.unset_n_flag();
        self.unset_h_flag();
        if (self.a() & 0x01) == 0x01 {
            self.set_c_flag();
        } else {
            self.unset_c_flag();
        }

        self.set_a(self.a().rotate_right(1));
    }

    fn ld_de_d16(&mut self, operands: Vec<u8>) {
        self.time += 3;
        self.pc += 3;

        self.set_de(u16::from_le_bytes([operands[1], operands[0]]));
    }

    fn ld_de_a(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 1;

        self.ram.store(self.de(), self.a());
    }

    fn inc_de(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 1;

        self.set_de(self.de() + 1);
    }

    fn inc_d(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        if (((self.d() & 0xf) + (1 & 0xf)) & 0x10) == 0x10 {
            self.set_h_flag();
        } else {
            self.unset_h_flag();
        }

        self.set_d(self.d() + 1);

        if self.d() == 0 {
            self.set_z_flag();
        } else {
            self.unset_z_flag();
        }
        self.unset_n_flag();
    }

    fn dec_d(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        // How do I compute half-carry for -1 as an operand?
        //if (((self.d() & 0xf) + (1 & 0xf)) & 0x10) == 0x10 {
            //self.set_h_flag();
        //} else {
            //self.unset_h_flag();
        //}

        self.set_d(self.d() - 1);

        if self.d() == 0 {
            self.set_z_flag();
        } else {
            self.unset_z_flag();
        }
        self.set_n_flag();
    }

    fn ld_d_d8(&mut self, operands: Vec<u8>) {
        self.time += 2;
        self.pc += 2;

        self.set_d(operands[0]);
    }

    fn rla(&mut self, operands: Vec<u8>) {
        self.time += 1;
        self.pc += 1;

        if self.a() & 0x7f == 0 {
            self.unset_c_flag();
        } else {
            self.set_c_flag();
        }
        self.unset_z_flag();
        self.unset_n_flag();
        self.unset_h_flag();

        self.set_a(self.a().rotate_left(1));
        if self.f() & 0x10 == 0 {
            self.set_a(self.a() & 0xfe);
        } else {
            self.set_a(self.a() | 0x01);
        }
    }

    //fn jr_s8(&mut self, operands: Vec<u8>) {
        //self.time += 3;
        //self.pc += 2;

        //Do I keep the increment of pc by 2 above?
        //Forwards or backwards jump?
        //self.pc += operands[0];
    //}
}

//Z set to 1 when the result of an operation is 0; otherwise reset.
//N set to 1 following execution of the subtraction instruction, regardless of result.
//H set to 1 when an operation results in carrying from or borrowing to bit 3.
//C set to 1 when an operation results in carrying from or borrowing to bit 7.

//https://gist.github.com/meganesu/9e228b6b587decc783aa9be34ae27841

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

#[cfg(test)]
mod tests {
    use super::*;
    use super::Instruction::*;

    #[test]
    fn nop() {
        let mut cpu = CPU::new(RAM::new());
        let instruction = Instruction8 { opcode: 0x00, operands: Vec::new() };

        let cpu_before = cpu.clone();
        cpu.execute(instruction);

        assert_eq!(cpu.time, cpu_before.time + 1);
        assert_eq!(cpu.pc, cpu_before.pc + 1);
    }

    #[test]
    fn ld_bc_d16() {
        let mut cpu = CPU::new(RAM::new());
        let instruction = Instruction8 { opcode: 0x01, operands: vec![0x01, 0x02] };

        let cpu_before = cpu.clone();
        cpu.execute(instruction);

        assert_eq!(cpu.bc(), 0x0102);
        assert_eq!(cpu.time, cpu_before.time + 3);
        assert_eq!(cpu.pc, cpu_before.pc + 3);
    }

    #[test]
    fn ld_bc_a() {
        let mut cpu = CPU::new(RAM::new());
        let instruction = Instruction8 { opcode: 0x02, operands: Vec::new() };

        cpu.set_a(0x12);

        let cpu_before = cpu.clone();
        cpu.execute(instruction);

        assert_eq!(cpu.ram.load(cpu.bc()), cpu.a());
        assert_eq!(cpu.time, cpu_before.time + 2);
        assert_eq!(cpu.pc, cpu_before.pc + 1);
    }

    #[test]
    fn inc_bc() {
        let mut cpu = CPU::new(RAM::new());
        let instruction = Instruction8 { opcode: 0x03, operands: Vec::new() };

        cpu.set_c(0x00);

        let cpu_before = cpu.clone();
        cpu.execute(instruction);

        assert_eq!(cpu.c(), cpu_before.c() + 1);
        assert_eq!(cpu.time, cpu_before.time + 2);
        assert_eq!(cpu.pc, cpu_before.pc + 1);

        let instruction = Instruction8 { opcode: 0x03, operands: Vec::new() };

        cpu.set_c(0xff);
        cpu.set_b(0x00);

        let cpu_before = cpu.clone();
        cpu.execute(instruction);

        assert_eq!(cpu.c(), 0x00);
        assert_eq!(cpu.b(), 0x01);
        assert_eq!(cpu.time, cpu_before.time + 2);
        assert_eq!(cpu.pc, cpu_before.pc + 1);
    }

    #[test]
    fn inc_b() {
        let mut cpu = CPU::new(RAM::new());
        let instruction = Instruction8 { opcode: 0x04, operands: Vec::new() };

        cpu.set_b(0x00);
        cpu.set_f(0x00);
        cpu.set_z_flag();
        cpu.set_n_flag();

        let cpu_before = cpu.clone();
        cpu.execute(instruction);

        assert_eq!(cpu.b(), cpu_before.b() + 1);
        assert_eq!(cpu.f(), 0x00);
        assert_eq!(cpu.time, cpu_before.time + 1);
        assert_eq!(cpu.pc, cpu_before.pc + 1);

        let instruction = Instruction8 { opcode: 0x04, operands: Vec::new() };

        cpu.set_b(0xff);
        cpu.set_f(0x00);

        let cpu_before = cpu.clone();
        cpu.execute(instruction);

        assert_eq!(cpu.b(), 0x00);
        assert_eq!(cpu.f(), 0x80 + 0x20);
        assert_eq!(cpu.time, cpu_before.time + 1);
        assert_eq!(cpu.pc, cpu_before.pc + 1);
    }

    #[test]
    fn dec_b() {
        let mut cpu = CPU::new(RAM::new());
        let instruction = Instruction8 { opcode: 0x05, operands: Vec::new() };

        cpu.set_b(0x01);
        cpu.set_f(0x00);

        let cpu_before = cpu.clone();
        cpu.execute(instruction);

        assert_eq!(cpu.b(), 0x00);
        assert_eq!(cpu.f(), 0x80 + 0x40);
        assert_eq!(cpu.time, cpu_before.time + 1);
        assert_eq!(cpu.pc, cpu_before.pc + 1);

        let instruction = Instruction8 { opcode: 0x05, operands: Vec::new() };

        cpu.set_b(0x00);
        cpu.set_f(0x00);
        cpu.set_z_flag();

        let cpu_before = cpu.clone();
        cpu.execute(instruction);

        assert_eq!(cpu.b(), 0xff);
        assert_eq!(cpu.f(), 0x40);
        assert_eq!(cpu.time, cpu_before.time + 1);
        assert_eq!(cpu.pc, cpu_before.pc + 1);

        let instruction = Instruction8 { opcode: 0x05, operands: Vec::new() };

        cpu.set_b(0xff);
        cpu.set_f(0x00);
        cpu.set_z_flag();

        let cpu_before = cpu.clone();
        cpu.execute(instruction);

        assert_eq!(cpu.b(), 0xfe);
        assert_eq!(cpu.f(), 0x40 + 0x20);
        assert_eq!(cpu.time, cpu_before.time + 1);
        assert_eq!(cpu.pc, cpu_before.pc + 1);
    }

    #[test]
    fn ld_b_d8() {
        let mut cpu = CPU::new(RAM::new());
        let instruction = Instruction8 { opcode: 0x06, operands: vec![0x01] };

        let cpu_before = cpu.clone();
        cpu.execute(instruction);

        assert_eq!(cpu.b(), 0x01);
        assert_eq!(cpu.f(), cpu_before.f());
        assert_eq!(cpu.time, cpu_before.time + 2);
        assert_eq!(cpu.pc, cpu_before.pc + 2);
    }

    #[test]
    fn rlca() {
        let mut cpu = CPU::new(RAM::new());
        let instruction = Instruction8 { opcode: 0x07, operands: Vec::new() };

        cpu.set_a(0x03);
        cpu.set_f(0xf0);
        cpu.set_c_flag();

        let cpu_before = cpu.clone();
        cpu.execute(instruction);

        assert_eq!(cpu.a(), 0x06);
        assert_eq!(cpu.f(), 0x00);
        assert_eq!(cpu.time, cpu_before.time + 1);
        assert_eq!(cpu.pc, cpu_before.pc + 1);

        let mut cpu = CPU::new(RAM::new());
        let instruction = Instruction8 { opcode: 0x07, operands: Vec::new() };

        cpu.set_a(0xf8);
        cpu.set_f(0xf0);
        cpu.unset_c_flag();

        let cpu_before = cpu.clone();
        cpu.execute(instruction);

        assert_eq!(cpu.a(), 0xf1);
        assert_eq!(cpu.f(), 0x10);
        assert_eq!(cpu.time, cpu_before.time + 1);
        assert_eq!(cpu.pc, cpu_before.pc + 1);
    }

    #[test]
    fn ld_a16_sp() {
        let mut cpu = CPU::new(RAM::new());
        let instruction = Instruction8 { opcode: 0x08, operands: vec![0x12, 0x34] };

        cpu.sp[1] = 0xab;
        cpu.sp[0] = 0xcd;

        let cpu_before = cpu.clone();
        cpu.execute(instruction);

        assert_eq!(cpu.sp[0], cpu.ram.load(0x1234));
        assert_eq!(cpu.sp[1], cpu.ram.load(0x1235));
        assert_eq!(cpu.time, cpu_before.time + 5);
        assert_eq!(cpu.pc, cpu_before.pc + 3);
    }

    #[test]
    fn add_hl_bc() {
        let mut cpu = CPU::new(RAM::new());
        let instruction = Instruction8 { opcode: 0x09, operands: Vec::new() };

        cpu.set_bc(0x0101);
        cpu.set_hl(0x0202);
        cpu.set_f(0xf0);

        let cpu_before = cpu.clone();
        cpu.execute(instruction);

        assert_eq!(cpu.hl(), 0x0303);
        assert_eq!(cpu.f(), 0x80);
        assert_eq!(cpu.time, cpu_before.time + 2);
        assert_eq!(cpu.pc, cpu_before.pc + 1);

        let mut cpu = CPU::new(RAM::new());
        let instruction = Instruction8 { opcode: 0x09, operands: Vec::new() };

        cpu.set_bc(0x8800);
        cpu.set_hl(0x8800);
        cpu.set_f(0x00);

        let cpu_before = cpu.clone();
        cpu.execute(instruction);

        assert_eq!(cpu.hl(), 0x1000);
        assert_eq!(cpu.f(), 0x20 + 0x10);
    }

    #[test]
    fn ld_a_bc() {
        let mut cpu = CPU::new(RAM::new());
        let instruction = Instruction8 { opcode: 0x0a, operands: Vec::new() };

        cpu.ram.store(cpu.bc(), 0x11);

        let cpu_before = cpu.clone();
        cpu.execute(instruction);

        assert_eq!(cpu.a(), 0x11);
        assert_eq!(cpu.time, cpu_before.time + 2);
        assert_eq!(cpu.pc, cpu_before.pc + 1);
    }

    #[test]
    fn dec_bc() {
        let mut cpu = CPU::new(RAM::new());
        let instruction = Instruction8 { opcode: 0x0b, operands: Vec::new() };

        cpu.set_bc(0x0102);

        let cpu_before = cpu.clone();
        cpu.execute(instruction);

        assert_eq!(cpu.bc(), 0x0101);
        assert_eq!(cpu.time, cpu_before.time + 2);
        assert_eq!(cpu.pc, cpu_before.pc + 1);
    }

    #[test]
    fn inc_c() {
        let mut cpu = CPU::new(RAM::new());
        let instruction = Instruction8 { opcode: 0x0c, operands: Vec::new() };

        cpu.set_c(0x00);
        cpu.set_f(0xf0);

        let cpu_before = cpu.clone();
        cpu.execute(instruction);

        assert_eq!(cpu.c(), 0x01);
        assert_eq!(cpu.f(), 0x10);
        assert_eq!(cpu.time, cpu_before.time + 1);
        assert_eq!(cpu.pc, cpu_before.pc + 1);

        let mut cpu = CPU::new(RAM::new());
        let instruction = Instruction8 { opcode: 0x0c, operands: Vec::new() };

        cpu.set_c(0xff);
        cpu.set_f(0x00);

        let cpu_before = cpu.clone();
        cpu.execute(instruction);

        assert_eq!(cpu.c(), 0x00);
        assert_eq!(cpu.f(), 0x80 + 0x20);

        let mut cpu = CPU::new(RAM::new());
        let instruction = Instruction8 { opcode: 0x0c, operands: Vec::new() };

        cpu.set_c(0x0f);
        cpu.set_f(0x00);

        let cpu_before = cpu.clone();
        cpu.execute(instruction);

        assert_eq!(cpu.c(), 0x10);
        assert_eq!(cpu.f(), 0x20);
    }

    #[test]
    fn dec_c() {
        let mut cpu = CPU::new(RAM::new());
        let instruction = Instruction8 { opcode: 0x0d, operands: Vec::new() };

        cpu.set_c(0x01);
        cpu.set_f(0x00);

        let cpu_before = cpu.clone();
        cpu.execute(instruction);

        assert_eq!(cpu.c(), 0x00);
        assert_eq!(cpu.f(), 0x80 + 0x40);
        assert_eq!(cpu.time, cpu_before.time + 1);
        assert_eq!(cpu.pc, cpu_before.pc + 1);

        let mut cpu = CPU::new(RAM::new());
        let instruction = Instruction8 { opcode: 0x0d, operands: Vec::new() };

        cpu.set_c(0x00);
        cpu.set_f(0x00);

        let cpu_before = cpu.clone();
        cpu.execute(instruction);

        assert_eq!(cpu.c(), 0xff);
        assert_eq!(cpu.f(), 0x40);

        //NOTE:Not sure how to test half-carry flag for subtraction
    }

    #[test]
    fn ld_c_d8() {
        let mut cpu = CPU::new(RAM::new());
        let instruction = Instruction8 { opcode: 0x0e, operands: vec![0x01] };

        let cpu_before = cpu.clone();
        cpu.execute(instruction);

        assert_eq!(cpu.c(), 0x01);
        assert_eq!(cpu.time, cpu_before.time + 2);
        assert_eq!(cpu.pc, cpu_before.pc + 2);
    }

    #[test]
    fn rrca() {
        let mut cpu = CPU::new(RAM::new());
        let instruction = Instruction8 { opcode: 0x0f, operands: Vec::new() };

        cpu.set_a(0x81);
        cpu.set_f(0xf0);

        let cpu_before = cpu.clone();
        cpu.execute(instruction);

        assert_eq!(cpu.a(), 0xc0);
        assert_eq!(cpu.f(), 0x10);
        assert_eq!(cpu.time, cpu_before.time + 1);
        assert_eq!(cpu.pc, cpu_before.pc + 1);

        let mut cpu = CPU::new(RAM::new());
        let instruction = Instruction8 { opcode: 0x0f, operands: Vec::new() };

        cpu.set_a(0x80);
        cpu.set_f(0xf0);

        let cpu_before = cpu.clone();
        cpu.execute(instruction);

        assert_eq!(cpu.a(), 0x40);
        assert_eq!(cpu.f(), 0x00);
    }
}
