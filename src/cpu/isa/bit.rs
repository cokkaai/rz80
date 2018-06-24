
// === Bit Set, Reset, and Test Group ===

use cpu::CPU;

#[allow(dead_code)]

impl CPU {
    fn operand_b(opcode: u8) -> u8 {
        match (opcode & 0b0011_1000) >> 3 {
            0b000 => 0b0000_0001,
            0b001 => 0b0000_0010,
            0b010 => 0b0000_0100,
            0b011 => 0b0000_1000,
            0b100 => 0b0001_0000,
            0b101 => 0b0010_0000,
            0b110 => 0b0100_0000,
            0b111 => 0b1000_0000,
            _ => panic!(),
        }
    }

    fn value_r(&self, opcode: u8) -> u8 {
        match opcode & 0b0000_0111 {
            0b000 => self.b,
            0b001 => self.c,
            0b010 => self.d,
            0b011 => self.e,
            0b100 => self.h,
            0b101 => self.l,
            0b111 => self.a,
            _ => panic!(),
        }
    }

    fn is_zero(&mut self, bitmask: u8, value: u8) {
        let res = bitmask & value;
        
        self.set_z(res == 0);
        self.set_h(true);
        self.set_n(false);
    }

    // BIT b, r
    pub fn bit_b_r(&mut self) {
        let opcode = self.memory_at_pc(1);
        let bitmask = Self::operand_b(opcode);
        let data = self.value_r(opcode);
        self.is_zero(bitmask, data);
        self.pc += 2;
    }

    // BIT b, (HL)
    pub fn bit_b_hli(&mut self) {
        let bitmask = Self::operand_b(self.memory_at_pc(1));
        let data = self.memory[self.read_hl() as usize];
        self.is_zero(bitmask, data);
        self.pc += 2;
    }

    // BIT b, (IX+d)
    pub fn bit_b_ixdi(&mut self) {
        let bitmask = Self::operand_b(self.memory_at_pc(3));
        let addr = self.ix as usize + self.memory_at_pc(2) as usize;
        let data = self.memory[addr];
        self.is_zero(bitmask, data);
        self.pc += 4;
    }

    // BIT b, (IY+d)
    pub fn bit_b_iydi(&mut self) {
        let bitmask = Self::operand_b(self.memory_at_pc(3));
        let addr = self.iy as usize + self.memory_at_pc(2) as usize;
        let data = self.memory[addr];
        self.is_zero(bitmask, data);
        self.pc += 4;
    }

    // SET b, r
    pub fn set_b_r(&mut self) {
        let opcode = self.memory_at_pc(1);
        let bitmask = Self::operand_b(opcode);

        match opcode & 0b0000_0111 {
            0b000 => self.b |= bitmask,
            0b001 => self.c |= bitmask,
            0b010 => self.d |= bitmask,
            0b011 => self.e |= bitmask,
            0b100 => self.h |= bitmask,
            0b101 => self.l |= bitmask,
            0b111 => self.a |= bitmask,
            _ => panic!(),
        }
        
        self.pc += 2;
    }

    // SET b, (HL)
    pub fn set_b_hli(&mut self) {
        let opcode = self.memory_at_pc(1);
        let bitmask = Self::operand_b(opcode);
        let addr = self.read_hl() as usize;
        self.memory[addr] |= bitmask;
        self.pc += 2;
    }

    // SET b, (IX+d)
    pub fn set_b_ixdi(&mut self) {
        let bitmask = Self::operand_b(self.memory_at_pc(3));
        let addr = self.ix as usize + self.memory_at_pc(2) as usize;
        self.memory[addr] |= bitmask;
        self.pc += 4;
    }

    // SET b, (IY+d)
    pub fn set_b_iydi(&mut self) {
        let bitmask = Self::operand_b(self.memory_at_pc(3));
        let addr = self.iy as usize + self.memory_at_pc(2) as usize;
        self.memory[addr] |= bitmask;
        self.pc += 4;
    }

    // RES b, r
    pub fn res_b_r(&mut self) {
        let opcode = self.memory_at_pc(1);
        let bitmask = !Self::operand_b(opcode);

        match opcode & 0b0000_0111 {
            0b000 => self.b &= bitmask,
            0b001 => self.c &= bitmask,
            0b010 => self.d &= bitmask,
            0b011 => self.e &= bitmask,
            0b100 => self.h &= bitmask,
            0b101 => self.l &= bitmask,
            0b111 => self.a &= bitmask,
            _ => panic!(),
        }
        
        self.pc += 2;
    }

    // RES b, (HL)
    pub fn res_b_hli(&mut self) {
        let opcode = self.memory_at_pc(1);
        let bitmask = !Self::operand_b(opcode);
        let addr = self.read_hl() as usize;
        self.memory[addr] &= bitmask;
        self.pc += 2;
    }

    // RES b, (IX+d)
    pub fn res_b_ixdi(&mut self) {
        let bitmask = !Self::operand_b(self.memory_at_pc(3));
        let addr = self.ix as usize + self.memory_at_pc(2) as usize;
        self.memory[addr] &= bitmask;
        self.pc += 4;
    }

    // RES b, (IY+d)
    pub fn res_b_iydi(&mut self) {
        let bitmask = !Self::operand_b(self.memory_at_pc(3));
        let addr = self.iy as usize + self.memory_at_pc(2) as usize;
        self.memory[addr] &= bitmask;
        self.pc += 4;
    }
}
