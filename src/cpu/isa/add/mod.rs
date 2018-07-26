#[cfg(test)]
mod tests;

use cpu::Cpu;

#[allow(dead_code)]

// === 8-Bit Arithmetic Group / ADD ===

impl Cpu {
    fn _add_to_accumulator(&mut self, value: u8) {
        let (result, carry) = self.a.overflowing_add(value);
        let (_, overflow) = Cpu::compl2(result).overflowing_add(Cpu::compl2(value));

        self.a = result;

        // S is set if result is negative (127<result<256).
        self.set_s_from_msb(result);

        // Z is set if result is 0 (result=0).
        self.set_z_from_byte(result);

        // P/V is set if overflow (overflow in twos complement).
        // TODO: Verify that implementation complies with Z80 arch.
        self.set_pv(overflow);

        // N is reset (0).
        self.set_n(false);

        // C is set if carry from bit 7 (result>255).
        self.set_c(carry);

        // H is set if carry from bit 3 (1 if carry from bit 3 to bit 4 else 0)
        // TODO: calculate flag H
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

    pub fn adc_a_s(&mut self) {
        unimplemented!();
    }
}
