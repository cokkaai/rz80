use cpu::Cpu;
use cpu::Register16;
use cpu::RegisterDemote;
use cpu::RegisterPromote;
use cpu::RegisterOperations;

#[cfg(test)]
mod tests;

#[allow(dead_code)]

// === 16-Bit Load Group ===

impl Cpu {
    fn push_byte(&mut self, value: u8) {
        self.sp -= 1;
        self.memory[self.sp as usize] = value;
    }

    fn pop_byte(&mut self) -> u8 {
        let value = self.memory[self.sp as usize];
        self.sp += 1;
        value
    }

    pub fn push_qq(&mut self) {
        let opcode = self.memory_at_pc(0);

        let h = match Cpu::select_reg16(opcode) {
            Register16::bc => self.b,
            Register16::de => self.d,
            Register16::hl => self.h,
            Register16::af => self.a,
            _ => panic!(),
        };

        let l = match Cpu::select_reg16(opcode) {
            Register16::bc => self.c,
            Register16::de => self.e,
            Register16::hl => self.l,
            Register16::af => self.f,
            _ => panic!(),
        };

        self.push_byte(h);
        self.push_byte(l);

        self.pc.reg_add(1);
    }

    pub fn push_ix(&mut self) {
        let value = self.ix;

        self.push_byte(value.high());
        self.push_byte(value.low());

        self.pc.reg_add(2);
    }

    pub fn push_iy(&mut self) {
        let value = self.iy;

        self.push_byte(value.high());
        self.push_byte(value.low());

        self.pc.reg_add(2);
    }

    pub fn pop_qq(&mut self) {
        let opcode = self.memory_at_pc(0);

        let l = self.pop_byte();
        let h = self.pop_byte();

        match Cpu::select_push16(opcode) {
            Register16::af => {
                self.a = h;
                self.f = l;
            },
            Register16::bc => {
                self.b = h;
                self.c = l;
            }
            Register16::de => {
                self.d = h;
                self.e = l;
            }
            Register16::hl => {
                self.h = h;
                self.l = l;
            }
            _ => panic!(),
        };

        self.pc.reg_add(1);
    }

    pub fn pop_ix(&mut self) {
        let l = self.pop_byte();
        let h = self.pop_byte();

        self.ix = (h, l).promote();

        self.pc.reg_add(2);
    }

    pub fn pop_iy(&mut self) {
        let l = self.pop_byte();
        let h = self.pop_byte();

        self.iy = (h, l).promote();

        self.pc.reg_add(2);
    }

}
