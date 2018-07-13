mod bytes;
mod registers;
mod reg8;
mod reg16;
mod reg88;
mod builder;
mod isa;
#[cfg(test)]
mod assertor;
#[cfg(test)]
mod test;

pub use self::registers::*;
pub use self::builder::CpuBuilder;

#[cfg(test)]
pub use self::assertor::Assertor;

#[derive(Debug)]
pub struct Cpu {
    pub pc: u16,
    pub sp: u16,
    pub ix: u16,
    pub iy: u16,

    pub i: u8,
    pub r: u8,

    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,

    pub a1: u8,
    pub b1: u8,
    pub c1: u8,
    pub d1: u8,
    pub e1: u8,
    pub f1: u8,
    pub h1: u8,
    pub l1: u8,

    pub iff1: bool,
    pub iff2: bool,

    pub memory: Vec<u8>,
}

#[allow(dead_code)]
impl Cpu {
    /// Read memory at address: pc + offset
    fn memory_at_pc(&self, offset_from_pc: u16) -> u8 {
        self.memory[(self.pc + offset_from_pc) as usize]
    }

    fn addr_at_pc(&self, offset_from_pc: u16) -> usize {
        self.memory_at_pc(offset_from_pc) as usize
            + ((self.memory_at_pc(offset_from_pc + 1) as usize) << 8)
    }

    /// Adds offset to PC register.
    /// TODO: Check if compliant with z80 hw.
    fn incr_pc(&mut self, offset: u16) {
        let msz = self.memory.capacity();
        let mut next = self.pc as usize + offset as usize;

        if next >= msz {
            next = next % msz;
        }

        self.pc = next as u16;
    }

    fn condition_at_pc(&self, offset_from_pc: u16) -> bool {
        let data = self.memory_at_pc(offset_from_pc);

        match (data & 0b00_111_000) >> 3 {
            0b000 => !self.get_z(),
            0b001 => self.get_z(),
            0b010 => !self.get_c(),
            0b011 => self.get_c(),
            0b100 => self.parity_is_odd(),
            0b101 => self.parity_is_even(),
            0b110 => self.sign_is_positive(),
            0b111 => self.sign_is_negative(),
            _ => panic!(),
        }
    }

    /// Read memory at address: hl + offset
    fn memory_at_hl(&self, offset_from_hl: u16) -> u8 {
        let addr = (self.h, self.l).promote() + offset_from_hl;
        self.memory[addr as usize]
    }

    /// Read memory at address: ix + offset
    fn memory_at_ix(&self, offset_from_ix: u16) -> u8 {
        self.memory[(self.ix + offset_from_ix) as usize]
    }

    /// Read memory at address: iy + offset
    fn memory_at_iy(&self, offset_from_iy: u16) -> u8 {
        self.memory[(self.iy + offset_from_iy) as usize]
    }
}
