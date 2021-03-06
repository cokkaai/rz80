use cpu::Cpu;
use cpu::RegisterDemote;
use cpu::RegisterPromote;
use cpu::RegisterOperations;

#[cfg(test)]
mod tests;

#[allow(dead_code)]

// === Call and Return Group ===

impl Cpu {
    fn _push_pc(&mut self) {
        // (SP – 1) ← PCH
        self.sp -= 1;
        self.memory[self.sp as usize] = self.pc.high();
        
        // (SP – 2) ← PCL
        self.sp -= 1;
        self.memory[self.sp as usize] = self.pc.low();
    }

    fn _call(&mut self) {
        let addr = (self.memory_at_pc(2), self.memory_at_pc(1)).promote();

        self.pc.reg_add(3);
        self._push_pc();
        
        // PC ← nn
        self.pc = addr;
    }

    pub fn call_nn(&mut self) {
        self._call();
    }

    pub fn call_cc_nn(&mut self) {
        if self.condition_at_pc(0) {
            self._call();
        } else {
            self.pc.reg_add(3);
        }
    }

    fn _pop_pc(&mut self) {
        // PCL ← (SP)
        let l = self.memory[self.sp as usize];
        self.sp += 1;

        // PCH ← (SP+1)
        let h = self.memory[self.sp as usize];
        self.sp += 1;

        self.pc = (h, l).promote();
    }

    pub fn ret(&mut self) {
        self._pop_pc();
    }

    pub fn ret_cc(&mut self) {
        if self.condition_at_pc(0) {
            self._pop_pc();
        } else {
            self.pc.reg_add(1);
        }
    }

    pub fn reti(&mut self) {
        unimplemented!();
    }

    pub fn retn(&mut self) {
        self.ret();
        self.iff1 = self.iff2;
    }

    pub fn rst_p(&mut self) {
        let cc = (self.memory_at_pc(0) & 0b00_111_000) >> 3;

        self.ret();

        self.pc = match cc {
            0b000 => 0x00,
            0b001 => 0x08,
            0b010 => 0x10,
            0b011 => 0x18,
            0b100 => 0x20,
            0b101 => 0x28,
            0b110 => 0x30,
            0b111 => 0x30,
            _ => panic!(),
        }
    }
}
