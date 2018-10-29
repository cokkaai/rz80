#[cfg(test)]
mod tests;

use cpu::Cpu;

#[allow(dead_code)]

// === 8-Bit Arithmetic Group / ADD ===

impl Cpu {
    fn _sub_from_accumulator(&mut self, value: u8, value2: u8) {
        let (mut result, mut carry) = self.a.overflowing_sub(value);

        // If value 2 is meaningful, add it and calculate carry.
        if value2 != 0 {
            let (result2, carry2) = result.overflowing_sub(value2);
            result = result2;
            carry |= carry2;
        }

        // S is set if result is negative (127<result<256).
        self.set_s_from_msb(result);

        // Z is set if result is 0 (result=0).
        self.set_z_from_byte(result);

        // P/V is set if overflow (overflow in twos complement).
        // TODO: Verify that implementation complies with Z80 arch.
        //let (_, overflow) = Cpu::compl2(result).overflowing_sub(Cpu::compl2(value));
        //self.set_pv(overflow);

        // N is reset (1).
        self.set_n(true);

        // C is set if borrow (result<0).
        self.set_c(carry);

        // H is set if borrow from bit 4.
        // Whether a half carry occured or not can be determined by looking 
        // at the 3rd bit of the two arguments and the result; these are 
        // hashed into this table in the form r12, where r is the 3rd bit 
        // of the result, 1 is the 3rd bit of the 1st argument and 2 is the 
        // third bit of the 2nd argument; the tables differ for add and 
        // subtract operations.
        let lookup = ((self.a & 0x88) >> 3) |
            ((value & 0x88) >> 2) |
            ((result & 0x88) >> 1);

        let halfcarry_sub_table = [false, false, true, false, true, false, true, true];

        self.set_h(halfcarry_add_table[lookup & 0x07]);

        self.a = result;
    }

    pub fn sub_r(&mut self) {
        let opcode = self.memory_at_pc(0);
        let operand = self.read(Self::select(opcode & 0b111));
        self._sub_from_accumulator(operand, 0);
        self.pc += 1;
    }

    pub fn sub_n(&mut self) {
        let operand = self.memory_at_pc(1);
        self._sub_from_accumulator(operand, 0);
        self.pc += 2;
    }

    pub fn sub_hli(&mut self) {
        let operand = self.memory_at_hl();
        self._sub_from_accumulator(operand, 0);
        self.pc += 1;
    }

    pub fn sub_ixdi(&mut self) {
        let offset = self.memory_at_pc(2);
        let operand = self.memory_at_ix(offset);
        self._sub_from_accumulator(operand, 0);
        self.pc += 3;
    }

    pub fn sub_iydi(&mut self) {
        let offset = self.memory_at_pc(2);
        let operand = self.memory_at_iy(offset);
        self._sub_from_accumulator(operand, 0);
        self.pc += 3;
    }

    pub fn sbc_a_r(&mut self) {
        let opcode = self.memory_at_pc(0);
        let operand = self.read(Self::select(opcode & 0b111));
        let c_value = self.get_c_value();
        self._sub_from_accumulator(operand, c_value);
        self.pc += 1;
    }

    pub fn sbc_a_n(&mut self) {
        let operand = self.memory_at_pc(1);
        let c_value = self.get_c_value();
        self._sub_from_accumulator(operand, c_value);
        self.pc += 2;
    }

    fn sbc_a_hli(&mut self) {
        let operand = self.memory_at_hl();
        let c_value = self.get_c_value();
        self._sub_from_accumulator(operand, c_value);
        self.pc += 1;
    }

    fn sbc_a_ixdi(&mut self) {
        let offset = self.memory_at_pc(2);
        let operand = self.memory_at_ix(offset);
        let c_value = self.get_c_value();
        self._sub_from_accumulator(operand, c_value);
        self.pc += 3;
    }

    fn sbc_a_iydi(&mut self) {
        let offset = self.memory_at_pc(2);
        let operand = self.memory_at_iy(offset);
        let c_value = self.get_c_value();
        self._sub_from_accumulator(operand, c_value);
        self.pc += 3;
    }
}
