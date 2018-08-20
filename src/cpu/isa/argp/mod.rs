#[cfg(test)]
mod tests;

use cpu::Cpu;
use cpu::RegisterOperations;

// === General-Purpose Arithmetic ===

#[allow(dead_code)]
impl Cpu {
    pub fn daa(&mut self) {
        unimplemented!();
    }

    pub fn cpl(&mut self) {
        self.a = !self.a;
        self.set_h(true);
        self.set_n(false);
        self.pc.reg_add(1);
    }

    pub fn neg(&mut self) {
        let a = self.a;
        self.set_pv(a == 0x80);
        self.set_c(a != 0);
        self.set_n(true);
        
        let a = self.a.two_compl();

        self.set_s_from_msb(a);
        self.set_z_from_byte(a);

        // TODO: implement: H is set if borrow from bit 4; otherwise, it is reset.
        self.set_h(false);

        self.pc.reg_add(2);
    }

    pub fn ccf(&mut self) {
        let value = self.get_c();

        self.set_h(value);
        self.set_c(!value);
        self.set_n(false);

        self.pc.reg_add(1);
    }

    pub fn scf(&mut self) {
        self.set_c(true);
        self.set_h(false);
        self.set_n(false);
        self.pc += 1;
    }
}
