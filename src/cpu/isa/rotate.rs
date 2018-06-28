use cpu::CPU;
use cpu::Register;

#[allow(dead_code)]

impl CPU {
    pub fn rlca(&mut self) {
        let carry = (self.a & 0b1000_0000) != 0;
        self.set_c(carry);
        self.set_h(false);
        self.set_n(false);

        let result = self.a.rotate_left(1);
        self.a = result;
    }

    pub fn rla(&mut self) {
        let lsb = match self.get_c() {
            true => 1,
            false => 0,
        };

        let carry = (self.a & 0b1000_0000) != 0;
        self.set_c(carry);
        self.set_h(false);
        self.set_n(false);

        let result = self.a << 1;
        self.a = result | lsb;
    }

    // RRCA
    pub fn rrca(&mut self) {
        let carry = (self.a & 0b0000_0001) != 0;
        self.set_c(carry);
        self.set_h(false);
        self.set_n(false);

        let result = self.a.rotate_right(1);
        self.a = result;
    }

    pub fn rra(&mut self) {
        let msb = match self.get_c() {
            true => 1,
            false => 0,
        };

        let carry = (self.a & 0b0000_0001) != 0;
        self.set_c(carry);
        self.set_h(false);
        self.set_n(false);

        let result = self.a >> 1;
        self.a = result | msb;
    }

    fn _set_rlc_flags_from_result(&mut self, result: u8) {
        self.set_c((result & 0b0000_0001) != 0);
        self.set_s_from_byte(result);
        self.set_z_from_byte(result);
        self.set_h(false);
        self.set_n(false);
    }

    pub fn rlc_r(&mut self) {
        let opcode = self.memory_at_pc(1);

        let result = match Self::select_src(opcode) {
            Register::a => {
                self.a = self.a.rotate_left(1);
                self.a
            },
            Register::b => {
                self.b = self.b.rotate_left(1);
                self.b
            },
            Register::c => {
                self.c = self.c.rotate_left(1);
                self.c
            },
            Register::d => {
                self.d = self.d.rotate_left(1);
                self.d
            },
            Register::e => {
                self.e = self.e.rotate_left(1);
                self.e
            },
            Register::h => {
                self.h = self.h.rotate_left(1);
                self.h
            },
            Register::l => {
                self.l = self.l.rotate_left(1);
                self.l
            },
        };

        self._set_rlc_flags_from_result(result);
    }

    pub fn rlc_hli(&mut self) {
        let addr = self.read_hl() as usize;
        self.memory[addr] = self.memory[addr].rotate_left(1);
        let result = self.memory[addr];
        self._set_rlc_flags_from_result(result);
    }

    // RLC (IX+d)
    pub fn rlc_ixdi(&mut self) {
        unimplemented!();
    }

    // RLC (IY+d)
    pub fn rlc_iydi(&mut self) {
        unimplemented!();
    }

    // RL m
    pub fn rl_m(&mut self) {
        unimplemented!();
    }

    // RRC m
    pub fn rrc_m(&mut self) {
        unimplemented!();
    }

    // RR m
    pub fn rr_m(&mut self) {
        unimplemented!();
    }

    // SLA m
    pub fn sla_m(&mut self) {
        unimplemented!();
    }

    // SRA m
    pub fn sra_m(&mut self) {
        unimplemented!();
    }

    // SRL m
    pub fn srl_m(&mut self) {
        unimplemented!();
    }

    // RLD
    pub fn rld(&mut self) {
        unimplemented!();
    }

    // RRD
    pub fn rrd(&mut self) {
        unimplemented!();
    }
}
