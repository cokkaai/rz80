// === Jump Group ===

use cpu::CPU;
use cpu::RegisterPromote;

#[allow(dead_code)]

impl CPU { 
    // JP nn
    pub fn jp_nn(&mut self) {
        let addr = (self.memory_at_pc(2), self.memory_at_pc(1)).promote();
        self.pc = addr;
    }

    // JP cc, nn
    pub fn jp_cc_nn(&mut self) {
        // IF cc true, PC ← nn
        if self.condition_at_pc(0) {
            self.pc = (self.memory_at_pc(2), self.memory_at_pc(1)).promote();
        } else {
            self.incr_pc(3);
        }
    }

    // TODO: How should overflow be managed?
    fn pc_offset(&mut self, offset: i32) {
        self.pc = (self.pc as i32 + offset) as u16;
    }

    // JR e
    pub fn jr_e(&mut self) {
        let offset = self.memory_at_pc(1) as i32;
        self.pc_offset(offset);
    }

    fn jump_on(&mut self, cnd: bool) {
        if cnd {
            let offset = self.memory_at_pc(1) as i32;
            self.pc_offset(offset);
        } else {
            self.pc += 2;
        }
    }

    // JR C, e
    pub fn jr_c_e(&mut self) {
        let cnd = self.get_c();
        self.jump_on(cnd);
    }

    // JR NC, e
    pub fn jr_nc_e(&mut self) {
        let cnd = !self.get_c();
        self.jump_on(cnd);
    }

    // JR Z, e
    pub fn jr_z_e(&mut self) {
        let cnd = self.get_z();
        self.jump_on(cnd);
    }

    // JR NZ, e
    pub fn jr_nz_e(&mut self) {
        let cnd = !self.get_z();
        self.jump_on(cnd);
    }

    // JP (HL)
    pub fn jp_hl(&mut self) {
        let addr = (self.h, self.l).promote();
        self.pc = addr;
    }

    // JP (IX)
    pub fn jp_ix(&mut self) {
        self.pc = self.ix;
    }

    // JP (IY)
    pub fn jp_iy(&mut self) {
        self.pc = self.iy;
    }

    // DJNZ, e
    pub fn djnz_e(&mut self) {
        self.b -= 1;

        if self.b == 0 {
            self.incr_pc(2);
        } else {
            let offset = self.memory_at_pc(1) as i32;
            self.pc_offset(offset);
        }
    }
}

