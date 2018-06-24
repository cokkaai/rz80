// === Exchange, Block Transfer, and Search Group ===

use cpu::CPU;
use cpu::Register16;
use cpu::bytes;

#[allow(dead_code)]

impl CPU {
    fn exchange(a: &mut u8, b: &mut u8) {
        let t = *a;
        *a = *b;
        *b = t;
    }

    // EX DE, HL
    pub fn ex_de_hl(&mut self) {
        Self::exchange(&mut self.d, &mut self.h);
        Self::exchange(&mut self.e, &mut self.l);
        self.incr_pc(1);
    }

    // EX AF, AF′
    pub fn ex_af_af1(&mut self) {
        Self::exchange(&mut self.a, &mut self.a1);
        Self::exchange(&mut self.f, &mut self.f1);
        self.incr_pc(1);
    }

    // EXX
    pub fn exx(&mut self) {
        Self::exchange(&mut self.b, &mut self.b1);
        Self::exchange(&mut self.c, &mut self.c1);
        
        Self::exchange(&mut self.d, &mut self.d1);
        Self::exchange(&mut self.e, &mut self.e1);

        Self::exchange(&mut self.h, &mut self.h1);
        Self::exchange(&mut self.l, &mut self.l1);
        
        self.incr_pc(1);
    }

    // EX (SP), HL
    pub fn ex_spi_hl(&mut self) {
        let h = self.h;
        let l = self.l;

        let addrh = (self.sp + 1) as usize;
        let addrl = self.sp as usize;

        self.h = self.memory[addrh];
        self.l = self.memory[addrl];

        self.memory[addrh] = h;
        self.memory[addrl] = l;

        self.incr_pc(1);
    }

    // EX (SP), IX
    pub fn ex_spi_ix(&mut self) {
        let h = bytes::high(self.ix);
        let l = bytes::low(self.ix);

        let addrh = (self.sp + 1) as usize;
        let addrl = self.sp as usize;

        self.ix = bytes::promote(self.memory[addrh], self.memory[addrl]);
        self.memory[addrh] = h;
        self.memory[addrl] = l;

        self.incr_pc(2);
    }

    // EX (SP), IY
    pub fn ex_spi_iy(&mut self) {
        let h = bytes::high(self.iy);
        let l = bytes::low(self.iy);

        let addrh = (self.sp + 1) as usize;
        let addrl = self.sp as usize;

        self.iy = bytes::promote(self.memory[addrh], self.memory[addrl]);
        self.memory[addrh] = h;
        self.memory[addrl] = l;

        self.incr_pc(2);
    }

    fn _lddiff(&mut self, delta: i16) {
        // (DE) ← (HL)
        let addr = self.read16(Register16::de) as usize;
        let value_addr = self.read16(Register16::hl) as usize;
        self.memory[addr] = self.memory[value_addr];

        // DE ← DE + 1
        let value = self.read16(Register16::de) as i32 + delta as i32;
        self.write16(Register16::de, value as u16);

        // HL ← HL + 1
        let value = self.read16(Register16::hl) as i32 + delta as i32;
        self.write16(Register16::hl, value as u16);

        // BC ← BC – 1
        let value = self.read16(Register16::bc) - 1;
        self.write16(Register16::bc, value);

        // P/V is set if BC – 1 ≠ 0; otherwise, it is reset.
        let value = self.read16(Register16::bc) != 0;
        self.set_pv(value);

        self.incr_pc(2);
    }

    // LDI
    pub fn ldi(&mut self) {
        self._lddiff(1);
    }

    // LDIR
    pub fn ldir(&mut self) {
        loop {
            self.ldi();

            // while (BC ≠ 0)
            if self.read_bc() == 0 {
                break;
            } else {
                self.pc -= 2;
                // TODO: Honor interrupts.
            }
        }
    }

    // LDD
    pub fn ldd(&mut self) {
        self._lddiff(-1);
    }

    // LDDR
    pub fn lddr(&mut self) {
        loop {
            self.ldd();

            // while (BC ≠ 0)
            if self.read_bc() == 0 {
                break;
            } else {
                self.pc -= 2;
                // TODO: Honor interrupts.
            }
        }
    }

    pub fn _cpi(&mut self, step: i16) {
        let addr = self.read_hl() as usize;
        let diff = self.a - self.memory[addr];

        // BC ← BC – 1
        self.add_bc(-1);

        // HL ← HL +- 1
        self.add_hl(step);

        // Z is set if A is (HL); otherwise, it is reset.
        self.set_z(diff == 0);

        // P/V is set if BC – 1 is not 0; otherwise, it is reset.
        let temp = self.read16(Register16::bc) != 0;
        self.set_pv(temp);

        // S is set if result is negative; otherwise, it is reset.
        self.set_s_from_byte(diff);

        // H is set if borrow from bit 4; otherwise, it is reset.
        self.set_h_from_byte(diff);

        self.pc += 2;
    }

    pub fn cpi(&mut self) {
        self._cpi(1);
    }

    // CPIR
    pub fn cpir(&mut self) {
        loop {
            self.cpi();

            // while (BC ≠ 0)
            if self.read_bc() == 0 {
                break;
            } else {
                self.pc -= 2;
                // TODO: Honor interrupts.
            }
        }
    }

    // CPD
    pub fn cpd(&mut self) {
        self._cpi(-1);
    }

    // CPDR
    pub fn cpdr(&mut self) {
        loop {
            self.cpd();

            // while (BC ≠ 0)
            if self.read_bc() == 0 {
                break;
            } else {
                self.pc -= 2;
                // TODO: Honor interrupts.
            }
        }
    }

}
