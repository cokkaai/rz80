use cpu::Cpu;

#[cfg(test)]
mod tests;

#[allow(dead_code)]

// === 8-Bit Arithmetic Group / ADD ===

impl Cpu {
    fn _add_to_accumulator(&mut self, value: u8) {
        let (result, carry) = self.a.overflowing_add(value);
        let (_, overflow) = (result as i8).overflowing_add(value as i8);

        self.a = result;

        self.set_s_from_byte(result);
        self.set_z_from_byte(result);
        self.set_pv(overflow);
        self.set_n(false);
        self.set_c(carry);

        // TODO: H is set if carry from bit 3; otherwise, it is reset.
        self.set_h(false);
    }

    pub fn add_a_r(&mut self) {
        let opcode = self.memory_at_pc(0);
        let operand = self.read(Self::select(opcode & 0b111));
        self._add_to_accumulator(operand);
        self.pc += 1;
    }

    pub fn add_a_n(&mut self) {
        let operand = self.memory_at_pc(1);
        self._add_to_accumulator(operand);
        self.pc += 2;
    }

    pub fn add_a_hli(&mut self) {
        let operand = self.memory_at_hl(0);
        self._add_to_accumulator(operand);
        self.pc += 1;
    }

    pub fn add_a_ixdi(&mut self) {
        let offset = self.memory_at_pc(2);
        let operand = self.memory_at_ix(offset as u16);
        self._add_to_accumulator(operand);
        self.pc += 3;
    }

    pub fn add_a_iydi(&mut self) {
        let offset = self.memory_at_pc(2);
        let operand = self.memory_at_iy(offset as u16);
        self._add_to_accumulator(operand);
        self.pc += 3;
    }
}
