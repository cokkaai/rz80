use cpu::Cpu;

#[allow(dead_code)]

impl Cpu {
    fn _xor_with_accumulator(&mut self, value: u8) {
        self.a ^= value;
        let a = self.a;

        self.set_s_from_byte(a);
        self.set_z_from_byte(a);
        self.set_n(false);
        self.set_c(false);
        self.set_h(false);

        // TODO: P/V is reset if overflow; otherwise, it is reset.
    }

    pub fn xor_r(&mut self) {
        let opcode = self.memory_at_pc(0);
        let operand = self.read(Self::select(opcode & 0b111));
        self._xor_with_accumulator(operand);
        self.pc += 1;
    }

    pub fn xor_n(&mut self) {
        let operand = self.memory_at_pc(1);
        self._xor_with_accumulator(operand);
        self.pc += 2;
    }

    pub fn xor_hli(&mut self) {
        let operand = self.memory_at_hl(0);
        self._xor_with_accumulator(operand);
        self.pc += 1;
    }

    pub fn xor_ixdi(&mut self) {
        let offset = self.memory_at_pc(2);
        let operand = self.memory_at_ix(offset as u16);
        self._xor_with_accumulator(operand);
        self.pc += 3;
    }

    pub fn xor_iydi(&mut self) {
        let offset = self.memory_at_pc(2);
        let operand = self.memory_at_iy(offset as u16);
        self._xor_with_accumulator(operand);
        self.pc += 3;
    }
}
