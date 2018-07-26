#[cfg(test)]
mod tests;

use cpu::Cpu;
use cpu::Register;
use cpu::RegisterOperations;

#[allow(dead_code)]

impl Cpu {
    fn select_reg(opcode: u8) -> Register {
        Self::select((opcode & 0b0011_1000) >> 3)
    }

    fn _add_r(&mut self, reg: Register, value: u8) -> (u8, u8) {
        // TODO: set C and H flags

        match reg {
            Register::a => {
                let old = self.a;
                let (_result, _carry) = self.a.reg_add(value);
                (old, self.a)
            },
            Register::b => {
                let old = self.b;
                let (_result, _carry) = self.b.reg_add(value);
                (old, self.b)
            }
            Register::c => {
                let old = self.c;
                let (_result, _carry) = self.c.reg_add(value);
                (old, self.c)
            },
            Register::d => {
                let old = self.d;
                let (_result, _carry) = self.d.reg_add(value);
                (old, self.d)
            },
            Register::e => {
                let old = self.e;
                let (_result, _carry) = self.e.reg_add(value);
                (old, self.e)
            },
            Register::h => {
                let old = self.h;
                let (_result, _carry) = self.h.reg_add(value);
                (old, self.h)
            },
            Register::l => {
                let old = self.l;
                let (_result, _carry) = self.l.reg_add(value);
                (old, self.l)
            }
        }
    }

    // TODO: work on u8 or i8
    fn _add_addr(&mut self, addr: usize, value: i32) -> (u8, u8) {
        let old = self.memory[addr];
        //let (_, carry) = self.memory[addr].reg_add(value);
        self.memory[addr] = (self.memory[addr] as i32 + value) as u8;
        (old, self.memory[addr])
    }

    fn _evaluate_flags_after_inc(&mut self, old_value: u8, new_value: u8) {
        // S is set if result is negative; otherwise, it is reset.
        self.set_s_from_byte(new_value);

        // Z is set if result is 0; otherwise, it is reset.
        self.set_z_from_byte(new_value);
        
        // H is set if carry from bit 3; otherwise, it is reset.
        // TODO: Implement as requested!
        self.set_h(false);

        // P/V is set if (HL) was 7Fh before operation; otherwise, it is reset.
        self.set_pv(old_value == 0x7f);

        // N is reset.
        self.set_n(false);
        
        // C is not affected.
    }

    fn _evaluate_flags_after_dec(&mut self, old_value: u8, new_value: u8) {
        // S is set if result is negative; otherwise, it is reset.
        self.set_s_from_byte(new_value);

        // Z is set if result is 0; otherwise, it is reset.
        self.set_z_from_byte(new_value);
        
        // H is set if borrow from bit 4, otherwise, it is reset.
        // TODO: Implement as requested!
        self.set_h(false);
        
        // P/V is set if r was 80h before operation; otherwise, it is reset.
        self.set_pv(old_value == 0x80);
        
        // N is set.
        self.set_n(true);
        
        // C is not affected.
    }

    pub fn inc_r(&mut self) {
        let opcode = self.memory_at_pc(0);
        let result = self._add_r(Self::select_reg(opcode), 1);
        self._evaluate_flags_after_inc(result.0, result.1);
        self.incr_pc(1);
    }

    pub fn inc_hli(&mut self) {
        let addr = self.read_hl() as usize;
        let result = self._add_addr(addr, 1);
        self._evaluate_flags_after_inc(result.0, result.1);
        self.incr_pc(1);
    }

    pub fn inc_ixdi(&mut self) {
        let addr = self.ix as usize + self.memory_at_pc(2) as usize;
        let result = self._add_addr(addr, 1);
        self._evaluate_flags_after_inc(result.0, result.1);
        self.incr_pc(3);
    }

    pub fn inc_iydi(&mut self) {
        let addr = self.iy as usize + self.memory_at_pc(2) as usize;
        let result = self._add_addr(addr, 1);
        self._evaluate_flags_after_inc(result.0, result.1);
        self.incr_pc(3);
    }

    pub fn dec_r(&mut self) {
        let opcode = self.memory_at_pc(0);
        let result = self._add_r(Self::select_reg(opcode), 1);  // -1
        self._evaluate_flags_after_dec(result.0, result.1);
        self.incr_pc(1);
    }

    pub fn dec_hli(&mut self) {
        let addr = self.read_hl() as usize;
        let result = self._add_addr(addr, -1);
        self._evaluate_flags_after_dec(result.0, result.1);
        self.incr_pc(1);
    }

    pub fn dec_ixdi(&mut self) {
        let addr = self.ix as usize + self.memory_at_pc(2) as usize;
        let result = self._add_addr(addr, -1);
        self._evaluate_flags_after_dec(result.0, result.1);
        self.incr_pc(3);
    }

    pub fn dec_iydi(&mut self) {
        let addr = self.iy as usize + self.memory_at_pc(2) as usize;
        let result = self._add_addr(addr, -1);
        self._evaluate_flags_after_dec(result.0, result.1);
        self.incr_pc(3);
    }
}
