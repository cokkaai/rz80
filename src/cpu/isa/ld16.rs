// === 16-Bit Load Group ===

use cpu::CPU;
use cpu::Register16;
use cpu::RegisterDemote;
use cpu::RegisterPromote;

#[allow(dead_code)]

impl CPU {
    pub fn ld_dd_nn(&mut self) {
        let opcode = self.memory_at_pc(0);
        let l = self.memory_at_pc(1);
        let h = self.memory_at_pc(2);

        match CPU::select_reg16(opcode) {
            Register16::bc => {
                self.c = l;
                self.b = h;
            },
            Register16::de => {
                self.e = l;
                self.d = h;
            },
            Register16::hl => {
                self.l = l;
                self.h = h;
            },
            Register16::sp => {
                self.sp = (h, l).promote();
            },
            _ => panic!(),
        }

        self.incr_pc(3);
    }

    pub fn ld_ix_nn(&mut self) {
        let value = (self.memory_at_pc(3), self.memory_at_pc(2)).promote();
        self.ix = value;
        self.incr_pc(4);
    }

    pub fn ld_iy_nn(&mut self) {
        let value = (self.memory_at_pc(3), self.memory_at_pc(2)).promote();
        self.iy = value;
        self.incr_pc(4);
    }

    pub fn ld_hl_nni(&mut self) {
        let l = self.memory_at_pc(1);
        let h = self.memory_at_pc(2);
        let addr = (h, l).promote() as usize;
        self.l = self.memory[addr];
        self.h = self.memory[addr + 1];
        self.incr_pc(3);
    }

    pub fn ld_dd_nni(&mut self) {
        let opcode = self.memory_at_pc(1);
        let addr = self.memory_at_pc(2) as usize + ((self.memory_at_pc(3) as usize) << 8);
        let value = self.memory[addr] as u16 + ((self.memory[addr + 1] as u16) << 8);

        match CPU::select_reg16(opcode) {
            Register16::bc => self.write_bc(value),
            Register16::de => self.write_de(value),
            Register16::hl => self.write_hl(value),
            Register16::sp => self.sp = value,
            _ => panic!(),
        }

        self.incr_pc(4);
    }

    pub fn ld_ix_nni(&mut self) {
        let addr = (self.memory_at_pc(3), self.memory_at_pc(2)).promote() as usize;
        self.ix = (self.memory[addr + 1], self.memory[addr]).promote();
        self.incr_pc(4);
    }

    pub fn ld_iy_nni(&mut self) {
        let addr = (self.memory_at_pc(3), self.memory_at_pc(2)).promote() as usize;
        self.iy = (self.memory[addr + 1], self.memory[addr]).promote();
        self.incr_pc(4);
    }

    pub fn ld_nni_hl(&mut self) {
        let addr = (self.memory_at_pc(2), self.memory_at_pc(1)).promote() as usize;
        self.memory[addr] = self.l;
        self.memory[addr + 1] = self.h;
        self.incr_pc(3);
    }

    pub fn ld_nni_dd(&mut self) {
        let addr = (self.memory_at_pc(3), self.memory_at_pc(2)).promote() as usize;
        let code = self.memory_at_pc(1);
        match CPU::select_reg16(code) {
            Register16::bc => {
                self.memory[addr] = self.c;
                self.memory[addr + 1] = self.b;
            },
            Register16::de => {
                self.memory[addr] = self.e;
                self.memory[addr + 1] = self.d;
            },
            Register16::hl => {
                self.memory[addr] = self.l;
                self.memory[addr + 1] = self.h;
            },
            Register16::sp => {
                self.memory[addr] = self.sp.low();
                self.memory[addr + 1] = self.sp.high();
            },
            _ => panic!(),
        }
        self.incr_pc(4);
    }

    pub fn ld_nni_ix(&mut self) {
        let addr = (self.memory_at_pc(3), self.memory_at_pc(2)).promote() as usize;
        self.memory[addr] = self.ix.low();
        self.memory[addr + 1] = self.ix.high();
        self.incr_pc(4);
    }

    pub fn ld_nni_iy(&mut self) {
        let addr = (self.memory_at_pc(3), self.memory_at_pc(2)).promote() as usize;
        self.memory[addr] = self.iy.low();
        self.memory[addr + 1] = self.iy.high();
        self.incr_pc(4);
    }

    pub fn ld_sp_hl(&mut self) {
        self.sp = self.read16(Register16::hl);
        self.incr_pc(1);
    }

    pub fn ld_sp_ix(&mut self) {
        self.sp = self.ix;
        self.incr_pc(2);
    }

    pub fn ld_sp_iy(&mut self) {
        self.sp = self.iy;
        self.incr_pc(2);
    }

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

        let h = match CPU::select_reg16(opcode) {
            Register16::bc => self.b,
            Register16::de => self.d,
            Register16::hl => self.h,
            Register16::af => self.a,
            _ => panic!(),
        };

        let l = match CPU::select_reg16(opcode) {
            Register16::bc => self.c,
            Register16::de => self.e,
            Register16::hl => self.l,
            Register16::af => self.f,
            _ => panic!(),
        };

        self.push_byte(h);
        self.push_byte(l);

        self.incr_pc(1);
    }

    pub fn push_ix(&mut self) {
        let value = self.ix;

        self.push_byte(value.high());
        self.push_byte(value.low());

        self.incr_pc(2);
    }

    pub fn push_iy(&mut self) {
        let value = self.iy;

        self.push_byte(value.high());
        self.push_byte(value.low());

        self.incr_pc(2);
    }

    pub fn pop_qq(&mut self) {
        let opcode = self.memory_at_pc(0);

        let l = self.pop_byte();
        let h = self.pop_byte();

        match CPU::select_push16(opcode) {
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

        self.incr_pc(1);
    }

    pub fn pop_ix(&mut self) {
        let l = self.pop_byte();
        let h = self.pop_byte();

        self.ix = (h, l).promote();

        self.incr_pc(2);
    }

    pub fn pop_iy(&mut self) {
        let l = self.pop_byte();
        let h = self.pop_byte();

        self.iy = (h, l).promote();

        self.incr_pc(2);
    }

}
