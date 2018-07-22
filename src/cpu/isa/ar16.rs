use cpu::Cpu;
use cpu::Register16;
use cpu::RegisterPromote;
use cpu::RegisterOperations;

#[allow(dead_code)]

impl Cpu {
    fn match_ss(&self, opcode: u8) -> Register16 {
        match opcode & 0b0011_0000 {
            0b00_00_0000 => Register16::bc,
            0b00_01_0000 => Register16::de,
            0b00_10_0000 => Register16::hl,
            0b00_11_0000 => Register16::sp,
            _ => panic!(),
        }
    }

    fn read_ss(&self, opcode: u8) -> u16 {
        match opcode & 0b00110000{
            0b00_00_0000 => (self.b, self.c).promote(),
            0b00_01_0000 => (self.d, self.e).promote(),
            0b00_10_0000 => (self.h, self.l).promote(),
            0b00_11_0000 => self.sp,
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
        (&mut self.h, &mut self.l).reg_add(operand);

        // TODO: H is set if carry from bit 11
        // TODO: C is set if carry from bit 15

        self.set_n(false);
        self.incr_pc(1);
    }

    pub fn adc_hl_ss(&mut self) {
        let operand = self.read_ss(self.memory_at_pc(1)) + self.carry_to_u16();
        let (_result, overflow) = (&mut self.h, &mut self.l).reg_add(operand);

        // TODO: S is set if result is negative
        // TODO: Z is set if result is 0
        // TODO: H is set if carry from bit 11
        // TODO: C is set if carry from bit 15

        self.set_pv(overflow);
        self.set_n(false);
        self.incr_pc(2);
    }

    pub fn sbc_hl_ss(&mut self) {
        // TODO: same as adc_hl_ss but with a negative operand.
        // Operand must allow negative values.
        unimplemented!();
    }

    pub fn add_ix_pp(&mut self) {
        let operand = match self.memory_at_pc(1) & 0x30 {
            0b00_00_0000 => (self.b, self.c).promote(),
            0b00_01_0000 => (self.d, self.e).promote(),
            0b00_10_0000 => self.ix,
            0b00_11_0000 => self.sp,
            _ => panic!(),
        };

        let (_result, _overflow) = self.ix.reg_add(operand);

        // TODO: H is set if carry from bit 11
        // TODO: C is set if carry from bit 15

        self.set_n(false);
        self.incr_pc(2);
    }

    pub fn add_iy_rr(&mut self) {
        let operand = match self.memory_at_pc(1) & 0x30 {
            0b00_00_0000 => (self.b, self.c).promote(),
            0b00_01_0000 => (self.d, self.e).promote(),
            0b00_10_0000 => self.iy,
            0b00_11_0000 => self.sp,
            _ => panic!(),
        };

        let (_result, _overflow) = self.iy.reg_add(operand);

        // TODO: H is set if carry from bit 11
        // TODO: C is set if carry from bit 15

        self.set_n(false);
        self.incr_pc(2);
    }

    pub fn inc_ss(&mut self) {
        let (_result, overflow) = match self.match_ss(self.memory_at_pc(0)) {
            Register16::bc => (&mut self.b, &mut self.c).incr(),
            Register16::de => (&mut self.d, &mut self.e).incr(),
            Register16::hl => (&mut self.h, &mut self.l).incr(),
            Register16::sp => self.sp.incr(),
            _ => panic!(),
        };

        // TODO: Set flags
        self.set_pv(overflow);
        self.incr_pc(1);
    }

    pub fn inc_ix(&mut self) {
        let (_result, overflow) = self.ix.incr();

        // TODO: Set flags
        self.set_pv(overflow);
        self.incr_pc(2);
    }

    pub fn inc_iy(&mut self) {
        let (_result, overflow) = self.iy.incr();

        // TODO: Set flags
        self.set_pv(overflow);
        self.incr_pc(2);
    }

    pub fn dec_ss(&mut self) {
        let (_result, overflow) = match self.match_ss(self.memory_at_pc(0)) {
            Register16::bc => (&mut self.b, &mut self.c).decr(),
            Register16::de => (&mut self.d, &mut self.e).decr(),
            Register16::hl => (&mut self.h, &mut self.l).decr(),
            Register16::sp => self.sp.decr(),
            _ => panic!(),
        };

        // TODO: Set flags
        self.set_pv(overflow);
        self.incr_pc(1);
    }

    pub fn dec_ix(&mut self) {
        let (_result, overflow) = self.ix.decr();

        // TODO: Set flags
        self.set_pv(overflow);
        self.incr_pc(2);
    }

    pub fn dec_iy(&mut self) {
        let (_result, overflow) = self.iy.decr();

        // TODO: Set flags
        self.set_pv(overflow);
        self.incr_pc(2);
    }
}
