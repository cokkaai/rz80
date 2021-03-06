#[cfg(test)]
mod tests;

use cpu::Cpu;
use cpu::Register16;
use cpu::RegisterPromote;
use cpu::RegisterOperations;

#[allow(dead_code)]

impl Cpu {
    fn match_ss(&self, opcode: u8) -> Register16 {
        match opcode & 0x30 {
            0x00 => Register16::bc,
            0x10 => Register16::de,
            0x20 => Register16::hl,
            0x30 => Register16::sp,
            _ => panic!(),
        }
    }

    fn read_ss(&self, opcode: u8) -> u16 {
        match opcode & 0x30{
            0x00 => (self.b, self.c).promote(),
            0x10 => (self.d, self.e).promote(),
            0x20 => (self.h, self.l).promote(),
            0x30 => self.sp,
            _ => panic!(),
        }
    }

    fn carry_to_u16(&self) -> u16 {
        if self.get_c() {
            1
        } else {
            0
        }
    }

    pub fn add_hl_ss(&mut self) {
        let opcode = self.memory_at_pc(0);
        let operand = self.read_ss(opcode);
        let (_result, carry) = (&mut self.h, &mut self.l).reg_add(operand);

        // TODO: H is set if carry from bit 11

        self.set_c(carry);
        self.set_n(false);
        self.pc.reg_add(1);
    }

    pub fn adc_hl_ss(&mut self) {
        let operand = self.read_ss(self.memory_at_pc(1)) + self.carry_to_u16();
        let (result, carry) = (&mut self.h, &mut self.l).reg_add(operand);

        self.set_s_from_msbw(result);
        self.set_z_from_word(result);
        // TODO: H is set if carry from bit 11
        self.set_c(carry);
        //self.set_pv(overflow);
        self.set_n(false);

        self.pc.reg_add(2);
    }

    pub fn sbc_hl_ss(&mut self) {
        let operand = self.read_ss(self.memory_at_pc(1)) + self.carry_to_u16();
        let (result, carry) = (&mut self.h, &mut self.l).reg_sub(operand);

        self.set_s_from_msbw(result);
        self.set_z_from_word(result);
        // TODO: H is set if carry from bit 11
        self.set_c(carry);
        //self.set_pv(overflow);
        self.set_n(true);

        self.pc.reg_add(2);
    }

    pub fn add_ix_pp(&mut self) {
        let operand = match self.memory_at_pc(1) & 0x30 {
            0x00 => (self.b, self.c).promote(),
            0x10 => (self.d, self.e).promote(),
            0x20 => self.ix,
            0x30 => self.sp,
            _ => panic!(),
        };

        let (_result, carry) = self.ix.reg_add(operand);

        // TODO: H is set if carry from bit 11
        self.set_c(carry);
        self.set_n(false);

        self.pc.reg_add(2);
    }

    pub fn add_iy_rr(&mut self) {
        let operand = match self.memory_at_pc(1) & 0x30 {
            0x00 => (self.b, self.c).promote(),
            0x10 => (self.d, self.e).promote(),
            0x20 => self.iy,
            0x30 => self.sp,
            _ => panic!(),
        };

        let (_result, carry) = self.iy.reg_add(operand);

        // TODO: H is set if carry from bit 11
        self.set_c(carry);
        self.set_n(false);

        self.pc.reg_add(2);
    }

    pub fn inc_ss(&mut self) {
        match self.match_ss(self.memory_at_pc(0)) {
            Register16::bc => (&mut self.b, &mut self.c).incr(),
            Register16::de => (&mut self.d, &mut self.e).incr(),
            Register16::hl => (&mut self.h, &mut self.l).incr(),
            Register16::sp => self.sp.incr(),
            _ => panic!(),
        };

        self.pc.reg_add(1);
    }

    pub fn inc_ix(&mut self) {
        self.ix.incr();
        self.pc.reg_add(2);
    }

    pub fn inc_iy(&mut self) {
        self.iy.incr();
        self.pc.reg_add(2);
    }

    pub fn dec_ss(&mut self) {
        match self.match_ss(self.memory_at_pc(0)) {
            Register16::bc => (&mut self.b, &mut self.c).decr(),
            Register16::de => (&mut self.d, &mut self.e).decr(),
            Register16::hl => (&mut self.h, &mut self.l).decr(),
            Register16::sp => self.sp.decr(),
            _ => panic!(),
        };

        self.pc.reg_add(1);
    }

    pub fn dec_ix(&mut self) {
        self.ix.decr();
        self.pc.reg_add(2);
    }

    pub fn dec_iy(&mut self) {
        self.iy.decr();
        self.pc.reg_add(2);
    }
}
