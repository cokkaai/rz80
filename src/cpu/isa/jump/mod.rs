use cpu::Cpu;
use cpu::RegisterPromote;

#[cfg(test)]
mod tests;

#[allow(dead_code)]

// === Jump Group ===

impl Cpu { 
    pub fn jp_nn(&mut self) {
        let addr = (self.memory_at_pc(2), self.memory_at_pc(1)).promote();
        self.pc = addr;
    }

    pub fn jp_cc_nn(&mut self) {
        if self.condition_at_pc(0) {
            self.pc = (self.memory_at_pc(2), self.memory_at_pc(1)).promote();
        } else {
            self.incr_pc(3);
        }
    }

    pub fn jr_e(&mut self) {
        let offset = i16::from(self.memory_at_pc(1));
        self.offset_pc(offset);
    }

    fn jump_on(&mut self, cnd: bool) {
        if cnd {
            self.jr_e();
        } else {
            self.pc += 2;
        }
    }

    pub fn jr_c_e(&mut self) {
        let cnd = self.get_c();
        self.jump_on(cnd);
    }

    pub fn jr_nc_e(&mut self) {
        let cnd = !self.get_c();
        self.jump_on(cnd);
    }

    pub fn jr_z_e(&mut self) {
        let cnd = self.get_z();
        self.jump_on(cnd);
    }

    pub fn jr_nz_e(&mut self) {
        let cnd = !self.get_z();
        self.jump_on(cnd);
    }

    pub fn jp_hl(&mut self) {
        let addr = (self.h, self.l).promote();
        self.pc = addr;
    }

    pub fn jp_ix(&mut self) {
        self.pc = self.ix;
    }

    pub fn jp_iy(&mut self) {
        self.pc = self.iy;
    }

    pub fn djnz_e(&mut self) {
        self.b -= 1;

        if self.b == 0 {
            self.incr_pc(2);
        } else {
            let offset = i16::from(self.memory_at_pc(1));
            self.offset_pc(offset);
        }
    }
}

