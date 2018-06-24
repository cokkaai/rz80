use cpu::CPU;
use cpu::bytes;

#[allow(dead_code)]

impl CPU {
    // DAA
    pub fn daa(&mut self) {
        unimplemented!();
    }

    // CPL
    pub fn cpl(&mut self) {
        self.a = !self.a;
        self.set_h(true);
        self.set_n(false);
        self.incr_pc(2);
    }

    // NEG
    pub fn neg(&mut self) {
        let a = self.a;

        self.set_pv(a == 0x80);
        self.set_c(a != 0);
        
        self.a = bytes::compl2(self.a);

        let a = self.a;
        self.set_z_from_byte(a);
        self.set_n(true);

        // TODO: implement: H is set if borrow from bit 4; otherwise, it is reset.
        self.set_h(false);

        self.incr_pc(2);
    }

    // CCF
    pub fn ccf(&mut self) {
        let value = self.get_c();

        // TODO: CY ← !CY ?????
        self.set_h(value);
        self.set_c(!value);
        self.set_n(false);

        // C is set if CY was 0 before operation; otherwise, it is reset.

        self.incr_pc(1);
    }

    // SCF
    pub fn scf(&mut self) {
        self.set_c(true);
        self.set_h(false);
        self.set_n(false);
        self.pc += 1;
    }

    // NOP
    pub fn nop(&mut self) {
        self.pc += 1;
    }

    // HALT
    pub fn halt(&mut self) {
        // The HALT instruction suspends CPU operation until 
        // a subsequent interrupt or reset is received.
        // While in the HALT state, the processor executes NOPs 
        // to maintain memory refresh logic.
        loop {
            // TODO: Intercept interrupts or reset and exit
            // from the loop.
            // Add a sleep here.
        }

        // self.pc += 1;
    }

    // DI
    pub fn di(&mut self) {
        self.iff1 = false;
        self.iff2 = false;
        self.pc += 1;
    }

    // EI
    pub fn ei(&mut self) {
        self.iff1 = true;
        self.iff2 = true;
        self.pc += 1;
    }

    // IM 0
    pub fn im_0(&mut self) {
        unimplemented!();
    }

    // IM 1
    pub fn im_1(&mut self) {
        unimplemented!();
    }

    // IM 2
    pub fn im_2(&mut self) {
        unimplemented!();
    }
}
