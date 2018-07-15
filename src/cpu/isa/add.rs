use cpu::Cpu;

#[allow(dead_code)]

// === 8-Bit Arithmetic Group / ADD ===

impl Cpu {
    fn _add_to_accumulator(&mut self, value: u8) {
        let (result, overflow) = (self.a as i8).overflowing_add(value as i8);
        self.a = result as u8;

        self.set_s_from_byte(result as u8);
        self.set_z_from_byte(result as u8);
        self.set_pv(overflow);
        self.set_n(false);

        // TODO: C is set if carry from bit 7; otherwise, it is reset.
        self.set_c(false);

        // TODO: H is set if carry from bit 3; otherwise, it is reset.
        self.set_h(false);
    }

    pub fn add_a_r(&mut self) {
        let opcode = self.memory_at_pc(0);
        let operand = self.read(Self::select(opcode & 0b111));
        self._add_to_accumulator(operand);
        self.pc += 1;
    }

    pub fn add_d_n(&mut self) {
        let operand = self.memory_at_pc(1);
        self._add_to_accumulator(operand);
        self.pc += 2;
    }

    pub fn add_a_hli(&mut self) {
        let operand = self.memory_at_hl(0);
        self._add_to_accumulator(operand);
        self.pc += 1;
    }

    pub fn add_a_ixd(&mut self) {
        let offset = self.memory_at_pc(2);
        let operand = self.memory_at_ix(offset as u16);
        self._add_to_accumulator(operand);
        self.pc += 3;
    }

    pub fn add_a_iyd(&mut self) {
        let offset = self.memory_at_pc(2);
        let operand = self.memory_at_iy(offset as u16);
        self._add_to_accumulator(operand);
        self.pc += 3;
    }
}