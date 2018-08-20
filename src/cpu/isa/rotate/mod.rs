#[cfg(test)]
mod tests;

use cpu::Cpu;
use cpu::Register;
use cpu::RegisterPromote;
use cpu::RegisterOperations;

#[allow(dead_code)]

impl Cpu {

    // === Rotate registers right through the carry flag ===

    fn rr_reg(&mut self, reg: Register) -> u8 {
        let msb = if self.get_c() { 0x80 } else { 0 };

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

        self.set_h(false);
        self.set_n(false);
        self.set_c(carry);

        result
    }

    fn rr_mem(&mut self, addr: usize) {
        let msb = if self.get_c() { 0x80 } else { 0 };
        let carry = self.memory[addr].lsb();
        let result = (self.memory[addr] >> 1) | msb;
        self.memory[addr] = result;

        self.set_h(false);
        self.set_n(false);
        self.set_c(carry);
        self.set_s_from_msb(result);
        self.set_z_from_byte(result);
        self.set_pv(!result.lsb());
    }

    pub fn rra(&mut self) {
        self.rr_reg(Register::a);
        self.pc.reg_add(1);
    }

    pub fn rr_r(&mut self) {
        let reg = Self::select_src(self.memory_at_pc(1));
        let result = self.rr_reg(reg);
        
        self.set_s_from_msb(result);
        self.set_z_from_byte(result);
        self.set_pv(!result.lsb());

        self.pc.reg_add(2);
    }

    pub fn rr_hli(&mut self) {
        let addr = (self.h, self.l).promote() as usize;
        self.rr_mem(addr);
        self.pc.reg_add(2);
    }

    pub fn rr_ixdi(&mut self) {
        // TODO: Manage negative offset
        let addr = self.ix as usize + self.memory_at_pc(2) as usize;
        self.rr_mem(addr);
        self.pc.reg_add(4);
    }

    pub fn rr_iydi(&mut self) {
        // TODO: Manage negative offset
        let addr = self.iy as usize + self.memory_at_pc(2) as usize;
        self.rr_mem(addr);
        self.pc.reg_add(4);
    }


    // === Rotate registers left through the carry flag ===

    fn rl_reg(&mut self, reg: Register) -> u8 {
        let lsb = if self.get_c() { 1 } else { 0 };

        let (result, carry) = match reg {
            Register::a => {
                let carry = self.a.msb();
                self.a = (self.a << 1) | lsb;
                (self.a, carry)
            },
            Register::b => {
                let carry = self.b.msb();
                self.b = (self.b << 1) | lsb;
                (self.b, carry)
            },
            Register::c => {
                let carry = self.c.msb();
                self.c = (self.c << 1) | lsb;
                (self.c, carry)
            },
            Register::d => {
                let carry = self.d.msb();
                self.d = (self.d << 1) | lsb;
                (self.d, carry)
            },
            Register::e => {
                let carry = self.e.msb();
                self.e = (self.e << 1) | lsb;
                (self.e, carry)
            },
            Register::h => {
                let carry = self.h.msb();
                self.h = (self.h << 1) | lsb;
                (self.h, carry)
            },
            Register::l => {
                let carry = self.l.msb();
                self.l = (self.l << 1) | lsb;
                (self.l, carry)
            },
        };

        self.set_h(false);
        self.set_n(false);
        self.set_c(carry);

        result
    }

    fn rl_mem(&mut self, addr: usize) {
        let lsb = if self.get_c() { 1 } else { 0 };
        let carry = self.memory[addr].msb();
        let result = (self.memory[addr] << 1) | lsb;
        self.memory[addr] = result;

        self.set_h(false);
        self.set_n(false);
        self.set_c(carry);
        self.set_s_from_msb(result);
        self.set_z_from_byte(result);
        self.set_pv(!result.lsb());
    }

    pub fn rla(&mut self) {
        self.rl_reg(Register::a);
        self.pc.reg_add(1);
    }

    pub fn rl_r(&mut self) {
        let reg = Self::select_src(self.memory_at_pc(1));
        let result = self.rl_reg(reg);

        self.set_s_from_msb(result);
        self.set_z_from_byte(result);
        self.set_pv(!result.lsb());

        self.pc.reg_add(2);
    }

    pub fn rl_hli(&mut self) {
        let addr = (self.h, self.l).promote() as usize;
        self.rl_mem(addr);
        self.pc.reg_add(2);
    }

    pub fn rl_ixdi(&mut self) {
        // TODO: Manage negative offset
        let addr = self.ix as usize + self.memory_at_pc(2) as usize;
        self.rl_mem(addr);
        self.pc.reg_add(4);
    }

    pub fn rl_iydi(&mut self) {
        // TODO: Manage negative offset
        let addr = self.iy as usize + self.memory_at_pc(2) as usize;
        self.rl_mem(addr);
        self.pc.reg_add(4);
    }


    // === Rotate registers right ===

    fn rrc_reg(&mut self, reg: Register) -> u8 {
        let result = match reg {
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

        self.set_h(false);
        self.set_n(false);
        self.set_c(result.msb());

        result
    }

    pub fn rrc_r(&mut self) {
        let opcode = self.memory_at_pc(1);
        let result = self.rrc_reg(Self::select_src(opcode));
        
        self.set_s_from_msb(result);
        self.set_z_from_byte(result);
        self.set_pv(!result.lsb());

        self.pc.reg_add(2);
    }

    pub fn rrca(&mut self) {
        self.rrc_reg(Register::a);
        self.pc.reg_add(1);
    }

    // === Rotate registers left ===

    fn rlc_reg(&mut self, reg: Register) -> u8 {
        let result = match reg {
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

        self.set_h(false);
        self.set_n(false);
        self.set_c(result.lsb());

        result
    }

    pub fn rlc_r(&mut self) {
        let opcode = self.memory_at_pc(1);
        let result = self.rlc_reg(Self::select_src(opcode));

        self.set_s_from_msb(result);
        self.set_z_from_byte(result);
        self.set_pv(!result.lsb());

        self.pc.reg_add(2);
    }

    pub fn rlca(&mut self) {
        self.rlc_reg(Register::a);
        self.pc.reg_add(1);
    }


    // === Rotate memory location left ===
    
    fn rlc_memory_location(&mut self, addr: usize) {
        self.memory[addr] = self.memory[addr].rotate_left(1);
        let result = self.memory[addr];
        self.set_c(result.lsb());
        self.set_s_from_msb(result);
        self.set_z_from_byte(result);
        self.set_pv_from_byte(result);
        self.set_h(false);
        self.set_n(false);
    }

    pub fn rlc_hli(&mut self) {
        let addr = (self.h, self.l).promote() as usize;
        self.rlc_memory_location(addr);
        self.pc.reg_add(2);
    }

    pub fn rlc_ixdi(&mut self) {
        let addr = self.ix as usize + self.memory_at_pc(2) as usize;
        self.rlc_memory_location(addr);
        self.pc.reg_add(4);
    }

    pub fn rlc_iydi(&mut self) {
        let addr = self.iy as usize + self.memory_at_pc(2) as usize;
        self.rlc_memory_location(addr);
        self.pc.reg_add(4);
    }

    // === Rotate memory location right ===
    
    fn rrc_memory_location(&mut self, addr: usize) {
        self.memory[addr] = self.memory[addr].rotate_right(1);
        let result = self.memory[addr];
        self.set_c(result.msb());
        self.set_s_from_msb(result);
        self.set_z_from_byte(result);
        self.set_pv_from_byte(result);
        self.set_h(false);
        self.set_n(false);
    }

    pub fn rrc_hli(&mut self) {
        let addr = (self.h, self.l).promote() as usize;
        self.rrc_memory_location(addr);
        self.pc.reg_add(2);
    }

    pub fn rrc_ixdi(&mut self) {
        let addr = self.ix as usize + self.memory_at_pc(2) as usize;
        self.rrc_memory_location(addr);
        self.pc.reg_add(4);
    }

    pub fn rrc_iydi(&mut self) {
        let addr = self.iy as usize + self.memory_at_pc(2) as usize;
        self.rrc_memory_location(addr);
        self.pc.reg_add(4);
    }

    fn low_nibble(value: u8) -> u8 {
        value & 0x0f
    }

    fn high_nibble(value: u8) -> u8 {
        value & 0xf0
    }

    pub fn rld(&mut self) {
        let addr = (self.h, self.l).promote() as usize;
        let a_low_nibble = self.a & 0x0f;
        self.a = (self.a & 0xf0) | (self.memory[addr] >> 4);
        self.memory[addr] = (self.memory[addr] << 4) | a_low_nibble;

        let result = self.a;
        self.set_s_from_msb(result);
        self.set_z_from_byte(result);
        self.set_pv(!result.lsb());
        self.set_n(false);
        self.set_h(false);

        self.pc.reg_add(2);
    }

    pub fn rrd(&mut self) {
        let addr = (self.h, self.l).promote() as usize;
        let a_low_nibble = self.a & 0x0f;
        self.a = (self.a & 0xf0) | (self.memory[addr] & 0x0f);
        self.memory[addr] = (a_low_nibble << 4) | (self.memory[addr] >> 4);

        let result = self.a;
        self.set_s_from_msb(result);
        self.set_z_from_byte(result);
        self.set_pv(!result.lsb());
        self.set_n(false);
        self.set_h(false);

        self.pc.reg_add(2);
    }
}
