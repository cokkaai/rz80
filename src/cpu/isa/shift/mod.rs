#[cfg(test)]
mod tests;

use cpu::Cpu;
use cpu::Register;
use cpu::RegisterOperations;
use cpu::RegisterPromote;

#[allow(dead_code)]

// === Shift Group ===

impl Cpu {
    // === Shift registers left through the carry flag ===

    pub fn sla_r(&mut self) {
        let reg = Self::select_src(self.memory_at_pc(1));

        let (result, carry) = match reg {
            Register::a => {
                let msb = self.a.msb();
                self.a = self.a << 1;
                (self.a, msb)
            },
            Register::b => {
                let msb = self.b.msb();
                self.b = self.b << 1;
                (self.b, msb)
            },
            Register::c => {
                let msb = self.c.msb();
                self.c = self.c << 1;
                (self.c, msb)
            },
            Register::d => {
                let msb = self.d.msb();
                self.d = self.d << 1;
                (self.d, msb)
            },
            Register::e => {
                let msb = self.e.msb();
                self.e = self.e << 1;
                (self.e, msb)
            },
            Register::h => {
                let msb = self.h.msb();
                self.h = self.h << 1;
                (self.h, msb)
            },
            Register::l => {
                let msb = self.l.msb();
                self.l = self.l << 1;
                (self.l, msb)
            },
        };

        self.set_c(carry);
        self.set_s(false);
        self.set_h(false);
        self.set_n(false);
        self.set_z_from_byte(result);
        self.set_pv(!result.lsb());

        self.incr_pc(2);
    }

    fn sl_mem(&mut self, addr: usize) {
        let carry = self.memory[addr].msb();
        self.set_c(carry);
        let result = self.memory[addr] << 1;
        self.memory[addr] = result;

        self.set_s(false);
        self.set_h(false);
        self.set_n(false);
        self.set_z_from_byte(result);
        self.set_pv(!result.lsb());
    }

    pub fn sla_hli(&mut self) {
        let addr = (self.h, self.l).promote() as usize;
        self.sl_mem(addr);
        self.incr_pc(2);
    }

    pub fn sla_ixdi(&mut self) {
        // TODO: Manage negative offset
        let addr = self.ix as usize + self.memory_at_pc(2) as usize;
        self.sl_mem(addr);
        self.incr_pc(4);
    }

    pub fn sla_iydi(&mut self) {
        // TODO: Manage negative offset
        let addr = self.iy as usize + self.memory_at_pc(2) as usize;
        self.sl_mem(addr);
        self.incr_pc(4);
    }

    // === Shift registers right through the carry flag keeping msb ===

    pub fn sra_r(&mut self) {
        let reg = Self::select_src(self.memory_at_pc(1));

        let (result, carry) = match reg {
            Register::a => {
                let msb = self.a & 0x80;
                let carry = self.a.lsb(); 
                self.a = (self.a >> 1) | msb;
                (self.a, carry)
            },
            Register::b => {
                let msb = self.b & 0x80;
                let carry = self.b.lsb();
                self.b = (self.b >> 1) | msb;
                (self.b, carry)
            },
            Register::c => {
                let msb = self.c & 0x80;
                let carry = self.c.lsb();
                self.c = (self.c >> 1) | msb;
                (self.c, carry)
            },
            Register::d => {
                let msb = self.d & 0x80;
                let carry = self.d.lsb();
                self.d = (self.d >> 1) | msb;
                (self.d, carry)
            },
            Register::e => {
                let msb = self.e & 0x80;
                let carry = self.e.lsb();
                self.e = (self.e >> 1) | msb;
                (self.e, carry)
            },
            Register::h => {
                let msb = self.h & 0x80;
                let carry = self.h.lsb();
                self.h = (self.h >> 1) | msb;
                (self.h, carry)
            },
            Register::l => {
                let msb = self.l & 0x80;
                let carry = self.l.lsb();
                self.l = (self.l >> 1) | msb;
                (self.l, carry)
            },
        };

        self.set_c(carry);
        self.set_s(result.msb());
        self.set_z_from_byte(result);
        self.set_h(false);
        self.set_pv(!result.lsb());
        self.set_n(false);

        self.incr_pc(2);
    }

    fn sra_mem(&mut self, addr: usize) {
        let carry = self.memory[addr].lsb();
        self.set_c(carry);
        let msb = self.memory[addr] & 0x80;

        let result = self.memory[addr] >> 1 | msb;
        self.memory[addr] = result;

        self.set_s(result.msb());
        self.set_z_from_byte(result);
        self.set_h(false);
        self.set_pv(!result.lsb());
        self.set_n(false);
    }

    pub fn sra_hli(&mut self) {
        let addr = (self.h, self.l).promote() as usize;
        self.sra_mem(addr);
        self.incr_pc(2);
    }

    pub fn sra_ixdi(&mut self) {
        // TODO: Manage negative offset
        let addr = self.ix as usize + self.memory_at_pc(2) as usize;
        self.sra_mem(addr);
        self.incr_pc(4);
    }

    pub fn sra_iydi(&mut self) {
        // TODO: Manage negative offset
        let addr = self.iy as usize + self.memory_at_pc(2) as usize;
        self.sra_mem(addr);
        self.incr_pc(4);
    }

    // === Shift registers right through the carry flag ===

    pub fn srl_r(&mut self) {
        let reg = Self::select_src(self.memory_at_pc(1));

        let (result, carry) = match reg {
            Register::a => {
                let carry = self.a.lsb();
                self.a = self.a >> 1;
                (self.a, carry)
            },
            Register::b => {
                let carry = self.b.lsb();
                self.b = self.b >> 1;
                (self.b, carry)
            },
            Register::c => {
                let carry = self.c.lsb();
                self.c = self.c >> 1;
                (self.c, carry)
            },
            Register::d => {
                let carry = self.d.lsb();
                self.d = self.d >> 1;
                (self.d, carry)
            },
            Register::e => {
                let carry = self.e.lsb();
                self.e = self.e >> 1;
                (self.e, carry)
            },
            Register::h => {
                let carry = self.h.lsb();
                self.h = self.h >> 1;
                (self.h, carry)
            },
            Register::l => {
                let carry = self.l.lsb();
                self.l = self.l >> 1;
                (self.l, carry)
            },
        };

        self.set_c(carry);
        self.set_s(false);
        self.set_z_from_byte(result);
        self.set_h(false);
        self.set_pv(!result.lsb());
        self.set_n(false);

        self.incr_pc(2);
    }

    fn srl_mem(&mut self, addr: usize) {
        let carry = self.memory[addr].lsb();
        self.set_c(carry);

        let result = self.memory[addr] >> 1;
        self.memory[addr] = result;

        self.set_s(false);
        self.set_z_from_byte(result);
        self.set_h(false);
        self.set_pv(!result.lsb());
        self.set_n(false);
    }

    pub fn srl_hli(&mut self) {
        let addr = (self.h, self.l).promote() as usize;
        self.srl_mem(addr);
        self.incr_pc(2);
    }

    pub fn srl_ixdi(&mut self) {
        // TODO: Manage negative offset
        let addr = self.ix as usize + self.memory_at_pc(2) as usize;
        self.srl_mem(addr);
        self.incr_pc(4);
    }

    pub fn srl_iydi(&mut self) {
        // TODO: Manage negative offset
        let addr = self.iy as usize + self.memory_at_pc(2) as usize;
        self.srl_mem(addr);
        self.incr_pc(4);
    }
}
