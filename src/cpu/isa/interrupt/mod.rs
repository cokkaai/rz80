#[cfg(test)]
mod tests;

use cpu::Cpu;

#[allow(dead_code)]

// === CPU Control Groups ===

impl Cpu {
    pub fn nop(&mut self) {
        self.pc += 1;
    }

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

    pub fn di(&mut self) {
        self.iff1 = false;
        self.iff2 = false;
        self.pc += 1;
    }

    pub fn ei(&mut self) {
        self.iff1 = true;
        self.iff2 = true;
        self.pc += 1;
    }

    pub fn im_0(&mut self) {
        unimplemented!();
    }

    pub fn im_1(&mut self) {
        unimplemented!();
    }

    pub fn im_2(&mut self) {
        unimplemented!();
    }
}
