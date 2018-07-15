use cpu::Cpu;
use cpu::Register;

#[allow(dead_code)]

impl Cpu {
    fn select_reg(opcode: u8) -> Register {
        Self::select((opcode & 0b0011_1000) >> 3)
    }


    pub fn sub_r(&mut self) {
        unimplemented!();
    }

    pub fn sub_n(&mut self) {
        unimplemented!();
    }

    pub fn sub_hli(&mut self) {
        unimplemented!();
    }

    pub fn sub_ixdi(&mut self) {
        unimplemented!();
    }

    pub fn sub_iydi(&mut self) {
        unimplemented!();
    }

    pub fn adc_a_s(&mut self) {
        unimplemented!();
    }

    pub fn sbc_a_s(&mut self) {
        unimplemented!();
    }

    fn _add_r(&mut self, reg: Register, value: i32) -> (u8, u8) {
        match reg {
            Register::a => {
                let old = self.a;
                self.a = (self.a as i32 + value) as u8;
                (old, self.a)
            },
            Register::b => {
                let old = self.b;
                self.b = (self.b as i32 + value) as u8;
                (old, self.b)
            }
            Register::c => {
                let old = self.c;
                self.c = (self.b as i32 + value) as u8;
                (old, self.c)
            },
            Register::d => {
                let old = self.d;
                self.d = (self.d as i32 + value) as u8;
                (old, self.d)
            },
            Register::e => {
                let old = self.e;
                self.e = (self.e as i32 + value) as u8;
                (old, self.e)
            },
            Register::h => {
                let old = self.h;
                self.h = (self.h as i32 + value) as u8;
                (old, self.h)
            },
            Register::l => {
                let old = self.l;
                self.l = (self.l as i32 + value) as u8;
                (old, self.l)
            }
        }
    }

    fn _add_addr(&mut self, addr: usize, value: i32) -> (u8, u8) {
        let old = self.memory[addr];
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

    // INC r
    pub fn inc_r(&mut self) {
        let opcode = self.memory_at_pc(0);
        let result = self._add_r(Self::select_reg(opcode), 1);
        self._evaluate_flags_after_inc(result.0, result.1);
        self.incr_pc(1);
    }

    // INC (HL)
    pub fn inc_hli(&mut self) {
        let addr = self.read_hl() as usize;
        let result = self._add_addr(addr, 1);
        self._evaluate_flags_after_inc(result.0, result.1);
        self.incr_pc(1);
    }

    // INC (IX+d)
    pub fn inc_ixdi(&mut self) {
        let addr = self.ix as usize + self.memory_at_pc(2) as usize;
        let result = self._add_addr(addr, 1);
        self._evaluate_flags_after_inc(result.0, result.1);
        self.incr_pc(3);
    }

    // INC (IY+d)
    pub fn inc_iydi(&mut self) {
        let addr = self.iy as usize + self.memory_at_pc(2) as usize;
        let result = self._add_addr(addr, 1);
        self._evaluate_flags_after_inc(result.0, result.1);
        self.incr_pc(3);
    }

    // DEC r
    pub fn dec_r(&mut self) {
        let opcode = self.memory_at_pc(0);
        let result = self._add_r(Self::select_reg(opcode), -1);
        self._evaluate_flags_after_dec(result.0, result.1);
        self.incr_pc(1);
    }

    // DEC (HL)
    pub fn dec_hli(&mut self) {
        let addr = self.read_hl() as usize;
        let result = self._add_addr(addr, -1);
        self._evaluate_flags_after_dec(result.0, result.1);
        self.incr_pc(1);
    }

    // DEC (IX+d)
    pub fn dec_ixdi(&mut self) {
        let addr = self.ix as usize + self.memory_at_pc(2) as usize;
        let result = self._add_addr(addr, -1);
        self._evaluate_flags_after_dec(result.0, result.1);
        self.incr_pc(3);
    }

    // DEC (IY+d)
    pub fn dec_iydi(&mut self) {
        let addr = self.iy as usize + self.memory_at_pc(2) as usize;
        let result = self._add_addr(addr, -1);
        self._evaluate_flags_after_dec(result.0, result.1);
        self.incr_pc(3);
    }
}
