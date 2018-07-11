use cpu::Cpu;

#[allow(dead_code)]

impl Cpu {
    fn _and_with_accumulator(&mut self, value: u8) {
        self.a &= value;
        let a = self.a;

        self.set_s_from_byte(a);
        self.set_z_from_byte(a);

        // TODO: H is set if borrow from bit 4; otherwise, it is reset.
        
        // TODO: P/V is reset if overflow; otherwise, it is reset.

        self.set_n(true);

        // TODO: C is set if borrow; otherwise, it is reset.
    }

    pub fn and_r(&mut self) {
        let opcode = self.memory_at_pc(0);
        let operand = self.read(Self::select(opcode));
        self._and_with_accumulator(operand);
        self.pc += 1;
    }

    pub fn and_n(&mut self) {
        let operand = self.memory_at_pc(1);
        self._and_with_accumulator(operand);
        self.pc += 2;
    }

    pub fn and_hli(&mut self) {
        let operand = self.memory_at_hl(0);
        self._and_with_accumulator(operand);
        self.pc += 1;
    }

    pub fn and_ixdi(&mut self) {
        let offset = self.memory_at_pc(2);
        let operand = self.memory_at_ix(offset as u16);
        self._and_with_accumulator(operand);
        self.pc += 3;
    }

    pub fn and_iydi(&mut self) {
        let offset = self.memory_at_pc(2);
        let operand = self.memory_at_iy(offset as u16);
        self._and_with_accumulator(operand);
        self.pc += 3;
    }
}