use cpu::CPU;
use cpu::Register;
use cpu::RegisterOperations;

#[allow(dead_code)]

impl CPU {
    pub fn rlca(&mut self) {
        let carry = self.a.msb();
        self.set_c(carry);
        self.set_h(false);
        self.set_n(false);

        let result = self.a.rotate_left(1);
        self.a = result;

        self.incr_pc(1);
    }

    pub fn rla(&mut self) {
        let lsb = match self.get_c() {
            true => 1,
            false => 0,
        };

        let carry = self.a.msb();
        self.set_c(carry);
        self.set_h(false);
        self.set_n(false);

        let result = self.a << 1;
        self.a = result | lsb;

        self.incr_pc(1);
    }

    // RRCA
    pub fn rrca(&mut self) {
        let carry = self.a.lsb();
        self.set_c(carry);
        self.set_h(false);
        self.set_n(false);

        let result = self.a.rotate_right(1);
        self.a = result;

        self.incr_pc(1);
    }

    fn rr_reg(&mut self, reg: Register) -> u8 {
        let msb = match self.get_c() {
            true => 0b1000_0000,
            false => 0,
        };

        let (result, carry) = match reg {
            Register::a => {
                let carry = self.a.lsb();
                self.a = (self.a >> 1) | msb;
                (self.a, carry)
            },
            Register::b => {
                let carry = self.b.lsb();
                self.b = (self.b >> 1) | msb;
                (self.b, carry)
            },
            Register::c => {
                let carry = self.c.lsb();
                self.c = (self.c >> 1) | msb;
                (self.c, carry)
            },
            Register::d => {
                let carry = self.d.lsb();
                self.d = (self.d >> 1) | msb;
                (self.d, carry)
            },
            Register::e => {
                let carry = self.e.lsb();
                self.e = (self.e >> 1) | msb;
                (self.e, carry)
            },
            Register::h => {
                let carry = self.h.lsb();
                self.h = (self.h >> 1) | msb;
                (self.h, carry)
            },
            Register::l => {
                let carry = self.l.lsb();
                self.l = (self.l >> 1) | msb;
                (self.l, carry)
            },
        };

        self.set_c(carry);
        self.set_h(false);
        self.set_n(false);

        result
    }

    pub fn rra(&mut self) {
        self.rr_reg(Register::a);
        self.incr_pc(1);
    }

    fn _rlc_memory_location(&mut self, addr: usize) {
        self.memory[addr] = self.memory[addr].rotate_left(1);
        let result = self.memory[addr];
        self._set_rlc_flags_from_result(result);
    }

    fn _rrc_memory_location(&mut self, addr: usize) {
        self.memory[addr] = self.memory[addr].rotate_right(1);
        let result = self.memory[addr];
        self._set_rrc_flags_from_result(result);
    }

    fn _set_rlc_flags_from_result(&mut self, result: u8) {
        self.set_c((result & 0b0000_0001) != 0);
        self.set_s_from_byte(result);
        self.set_z_from_byte(result);
        self.set_pv_from_byte(result);
        self.set_h(false);
        self.set_n(false);
    }

    fn _set_rrc_flags_from_result(&mut self, result: u8) {
        self.set_c((result & 0b1000_0000) != 0);
        self.set_s_from_byte(result);
        self.set_z_from_byte(result);
        self.set_pv_from_byte(result);
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

        self.incr_pc(2);
    }

    pub fn rlc_hli(&mut self) {
        let addr = self.read_hl() as usize;
        self._rlc_memory_location(addr);
        self.incr_pc(2);
    }

    pub fn rlc_ixdi(&mut self) {
        let addr = self.ix as usize + self.memory_at_pc(2) as usize;
        self._rlc_memory_location(addr);
        self.incr_pc(4);
    }

    pub fn rlc_iydi(&mut self) {
        let addr = self.iy as usize + self.memory_at_pc(2) as usize;
        self._rlc_memory_location(addr);
        self.incr_pc(4);
    }

    pub fn rl_r(&mut self) {
        unimplemented!();
    }

    pub fn rl_hli(&mut self) {
        unimplemented!();
    }

    pub fn rl_ixdi(&mut self) {
        unimplemented!();
    }

    pub fn rl_iydi(&mut self) {
        unimplemented!();
    }

    pub fn rrc_r(&mut self) {
        let opcode = self.memory_at_pc(1);

        let result = match Self::select_src(opcode) {
            Register::a => {
                self.a = self.a.rotate_right(1);
                self.a
            },
            Register::b => {
                self.b = self.b.rotate_right(1);
                self.b
            },
            Register::c => {
                self.c = self.c.rotate_right(1);
                self.c
            },
            Register::d => {
                self.d = self.d.rotate_right(1);
                self.d
            },
            Register::e => {
                self.e = self.e.rotate_right(1);
                self.e
            },
            Register::h => {
                self.h = self.h.rotate_right(1);
                self.h
            },
            Register::l => {
                self.l = self.l.rotate_right(1);
                self.l
            },
        };

        self._set_rrc_flags_from_result(result);

        self.incr_pc(2);
    }

    pub fn rrc_hli(&mut self) {
        let addr = self.read_hl() as usize;
        self._rrc_memory_location(addr);
        self.incr_pc(2);
    }

    pub fn rrc_ixdi(&mut self) {
        let addr = self.ix as usize + self.memory_at_pc(2) as usize;
        self._rrc_memory_location(addr);
        self.incr_pc(4);
    }

    pub fn rrc_iydi(&mut self) {
        let addr = self.iy as usize + self.memory_at_pc(2) as usize;
        self._rrc_memory_location(addr);
        self.incr_pc(4);
    }

    pub fn rr_r(&mut self) {
        let reg = Self::select_src(self.memory_at_pc(1));
        let result = self.rr_reg(reg);
        self.set_pv_from_byte(result);
        self.incr_pc(2);
    }

    pub fn rr_hli(&mut self) {
        unimplemented!();
    }

    pub fn rr_ixdi(&mut self) {
        unimplemented!();
    }

    pub fn rr_iydi(&mut self) {
        unimplemented!();
    }

    pub fn sla_r(&mut self) {
        unimplemented!();
    }

    pub fn sla_hli(&mut self) {
        unimplemented!();
    }

    pub fn sla_ixdi(&mut self) {
        unimplemented!();
    }

    pub fn sla_iydi(&mut self) {
        unimplemented!();
    }

    pub fn sra_r(&mut self) {
        unimplemented!();
    }

    pub fn sra_hli(&mut self) {
        unimplemented!();
    }

    pub fn sra_ixdi(&mut self) {
        unimplemented!();
    }

    pub fn sra_iydi(&mut self) {
        unimplemented!();
    }

    pub fn srl_r(&mut self) {
        unimplemented!();
    }

    pub fn srl_hli(&mut self) {
        unimplemented!();
    }

    pub fn srl_ixdi(&mut self) {
        unimplemented!();
    }

    pub fn srl_iydi(&mut self) {
        unimplemented!();
    }

    pub fn rld(&mut self) {
        unimplemented!();
    }

    pub fn rrd(&mut self) {
        unimplemented!();
    }
}
