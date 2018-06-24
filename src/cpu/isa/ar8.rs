use cpu::CPU;
use cpu::Register;

#[allow(dead_code)]

impl CPU {
    fn select_reg(opcode: u8) -> Register {
        Self::select((opcode & 0b0011_1000) >> 3)
    }

    // ADD A, r
    pub fn add_a_r(&mut self) {
        unimplemented!();
    }

    // ADD A, n
    pub fn add_d_n(&mut self) {
        unimplemented!();
    }

    // ADD A, (HL)
    pub fn add_a_hli(&mut self) {
        unimplemented!();
    }

    // ADD A, (IX + d)
    pub fn add_a_ixd(&mut self) {
        unimplemented!();
    }

    // ADD A, (IY + d)
    pub fn add_a_iyd(&mut self) {
        unimplemented!();
    }

    // ADC A, s
    pub fn adc_a_s(&mut self) {
        unimplemented!();
    }

    // SUB s
    pub fn sub_s(&mut self) {
        unimplemented!();
    }

    // SBC A, s
    pub fn sbc_a_s(&mut self) {
        unimplemented!();
    }

    // AND s
    pub fn and_s(&mut self) {
        unimplemented!();
    }

    // OR s
    pub fn or_s(&mut self) {
        unimplemented!();
    }

    // XOR s
    pub fn xor_s(&mut self) {
        unimplemented!();
    }

    // CP s
    pub fn cp_s(&mut self) {
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

    // INC r
    pub fn inc_r(&mut self) {
        let opcode = self.memory_at_pc(0);
        let result = self._add_r(Self::select_reg(opcode), 1);

        // S is set if result is negative; otherwise, it is reset.
        self.set_s_from_byte(result.1);

        // Z is set if result is 0; otherwise, it is reset.
        self.set_z_from_byte(result.1);
        
        // H is set if carry from bit 3; otherwise, it is reset.
        // TODO: Implement as requested!
        self.set_h(false);
        
        // P/V is set if r was 7Fh before operation; otherwise, it is reset.
        self.set_pv(result.0 == 0x7f);
        
        // N is reset.
        self.set_n(false);
        
        // C is not affected.

        self.incr_pc(1);
    }

    // INC (HL)
    pub fn inc_hli(&mut self) {
        unimplemented!();
    }

    // INC (IX+d)
    pub fn inc_ixd(&mut self) {
        unimplemented!();
    }

    // INC (IY+d)
    pub fn inc_iyd(&mut self) {
        unimplemented!();
    }

    // DEC r
    pub fn dec_r(&mut self) {
        let opcode = self.memory_at_pc(0);
        let result = self._add_r(Self::select_reg(opcode), -1);

        // S is set if result is negative; otherwise, it is reset.
        self.set_s_from_byte(result.1);

        // Z is set if result is 0; otherwise, it is reset.
        self.set_z_from_byte(result.1);
        
        // H is set if carry from bit 4; otherwise, it is reset.
        // TODO: Implement as requested!
        self.set_h(false);
        
        // P/V is set if r was 80h before operation; otherwise, it is reset.
        self.set_pv(result.0 == 0x80);
        
        // N is reset.
        self.set_n(true);
        
        // C is not affected.

        self.incr_pc(1);
    }

    // DEC (HL)
    pub fn dec_hli(&mut self) {
        unimplemented!();
    }

    // DEC (IX+d)
    pub fn dec_ixdi(&mut self) {
        unimplemented!();
    }

    // DEC (IY+d)
    pub fn dec_iydi(&mut self) {
        unimplemented!();
    }
}
