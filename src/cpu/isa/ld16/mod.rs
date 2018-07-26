use cpu::Cpu;
use cpu::Register16;
use cpu::RegisterDemote;
use cpu::RegisterPromote;

#[cfg(test)]
mod tests;

#[allow(dead_code)]

// === 16-Bit Load Group ===

impl Cpu {
    pub fn ld_dd_nn(&mut self) {
        let opcode = self.memory_at_pc(0);
        let l = self.memory_at_pc(1);
        let h = self.memory_at_pc(2);

        match Cpu::select_reg16(opcode) {
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

        match Cpu::select_reg16(opcode) {
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
        match Cpu::select_reg16(code) {
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
}
