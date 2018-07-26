use cpu::Cpu;
use cpu::Register16;

#[cfg(test)]
mod tests;

#[allow(dead_code)]

// === 8-Bit Load Group ===

impl Cpu {
    pub fn ld_r_r1(&mut self) {
        let opcode = self.memory_at_pc(0);
        let src = Cpu::select_src(opcode);
        let dest = Cpu::select_dest(opcode);
        let value = self.read(src);
        self.write(dest, value);
        self.incr_pc(1);
    }

    pub fn ld_r_n(&mut self) {
        let opcode = self.memory_at_pc(0);
        let dest = Cpu::select_dest(opcode);
        let value = self.memory_at_pc(1);
        self.write(dest, value);
        self.incr_pc(2);
    }

    pub fn ld_r_hl(&mut self) {
        let opcode = self.memory_at_pc(0);
        let dest = Cpu::select_dest(opcode);
        let addr = self.read16(Register16::hl) as usize;
        let value = self.memory[addr];
        self.write(dest, value);
        self.incr_pc(1);
    }

    pub fn ld_r_ixd(&mut self) {
        // memory_at_pc(0) is always 0xdd
        let opcode = self.memory_at_pc(1);
        let addr = self.ix as usize + Cpu::compl2(self.memory_at_pc(2)) as usize;
        let dest = Cpu::select_dest(opcode);
        let value = self.memory[addr];
        self.write(dest, value);
        self.incr_pc(3);
    }

    pub fn ld_r_iyd(&mut self) {
        // memory_at_pc(0) is always 0xfd
        let opcode = self.memory_at_pc(1);
        let addr = self.iy as usize + Cpu::compl2(self.memory_at_pc(2)) as usize;
        let dest = Cpu::select_dest(opcode);
        let value = self.memory[addr];
        self.write(dest, value);
        self.incr_pc(3);
    }

    pub fn ld_hl_r(&mut self) {
        let opcode = self.memory_at_pc(0);
        let src = Cpu::select_src(opcode);
        let addr = self.read16(Register16::hl) as usize;
        self.memory[addr] = self.read(src);
        self.incr_pc(1);
    }

    pub fn ld_ixd_r(&mut self) {
        // memory_at_pc(0) is always 0xdd
        let opcode = self.memory_at_pc(1);
        let src = Cpu::select_src(opcode);
        let addr = self.ix as usize + Cpu::compl2(self.memory_at_pc(2)) as usize;
        self.memory[addr] = self.read(src);
        self.incr_pc(3);
    }

    pub fn ld_iyd_r(&mut self) {
        // memory_at_pc(0) is always 0xfd
        let opcode = self.memory_at_pc(1);
        let src = Cpu::select_src(opcode);
        let addr = self.iy as usize + Cpu::compl2(self.memory_at_pc(2)) as usize;
        self.memory[addr] = self.read(src);
        self.incr_pc(3);
    }

    pub fn ld_hl_n(&mut self) {
        let addr = self.read16(Register16::hl) as usize;
        let value = self.memory_at_pc(1);
        self.memory[addr] = value;
        self.incr_pc(2);
    }

    pub fn ld_ixd_n(&mut self) {
        let addr = self.ix as usize + self.memory_at_pc(2) as usize;
        self.memory[addr] = self.memory_at_pc(3);
        self.incr_pc(4);
    }

    pub fn ld_iyd_n(&mut self) {
        let addr = self.iy as usize + self.memory_at_pc(2) as usize;
        self.memory[addr] = self.memory_at_pc(3);
        self.incr_pc(4);
    }

    pub fn ld_a_bc(&mut self) {
        let addr = self.read16(Register16::bc) as usize;
        self.a = self.memory[addr];
        self.incr_pc(1);
    }

    pub fn ld_a_de(&mut self) {
        let addr = self.read16(Register16::de) as usize;
        self.a = self.memory[addr];
        self.incr_pc(1);
    }

    pub fn ld_a_nn(&mut self) {
        let addr = self.memory_at_pc(1) as usize + self.memory_at_pc(2) as usize;
        self.a = self.memory[addr];
        self.incr_pc(3);
    }

    pub fn ld_a_i(&mut self) {
        self.a = self.i;

        let temp = self.a;
        self.set_s_from_msb(temp);

        self.set_z_from_byte(temp);

        let temp = self.iff2;
        self.set_pv(temp);

        self.set_h(false);
        self.set_n(false);

        self.incr_pc(2);
    }

    pub fn ld_a_r(&mut self) {
        self.a = self.r;

        let value = self.a;
        self.set_s_from_msb(value);
        self.set_z_from_byte(value);

        let value = self.iff2;
        self.set_pv(value);

        self.set_h(false);
        self.set_n(false);

        self.incr_pc(2);
    }

    pub fn ld_i_a(&mut self) {
        self.i = self.a;
        self.incr_pc(2);
    }

    pub fn ld_r_a(&mut self) {
        self.r = self.a;
        self.incr_pc(2);
    }
}
