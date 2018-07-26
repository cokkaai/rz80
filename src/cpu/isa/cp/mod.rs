use cpu::Cpu;

#[cfg(test)]
mod tests;

#[allow(dead_code)]

impl Cpu {
    fn _cp_with_accumulator(&mut self, value: u8) {
        let result = self.a - value;

        self.set_s_from_byte(result);
        self.set_z_from_byte(result);
        self.set_pv_from_byte(result);
        self.set_n(true);

        // TODO: C is set if borrow; otherwise, it is reset.
        self.set_c(false);

        // TODO: H is set if borrow from bit 4; otherwise, it is reset.
        self.set_h(false);
    }

    pub fn cp_r(&mut self) {
        let opcode = self.memory_at_pc(0);
        let operand = self.read(Self::select(opcode & 0b111));
        self._cp_with_accumulator(operand);
        self.pc += 1;
    }

    pub fn cp_n(&mut self) {
        let operand = self.memory_at_pc(1);
        self._cp_with_accumulator(operand);
        self.pc += 2;
    }

    pub fn cp_hli(&mut self) {
        let operand = self.memory_at_hl(0);
        self._cp_with_accumulator(operand);
        self.pc += 1;
    }

    pub fn cp_ixdi(&mut self) {
        let offset = self.memory_at_pc(2);
        let operand = self.memory_at_ix(offset as u16);
        self._cp_with_accumulator(operand);
        self.pc += 3;
    }

    pub fn cp_iydi(&mut self) {
        let offset = self.memory_at_pc(2);
        let operand = self.memory_at_iy(offset as u16);
        self._cp_with_accumulator(operand);
        self.pc += 3;
    }
}
