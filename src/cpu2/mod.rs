/////////////////////// CPU NG //////////////////

mod bytes;
mod reg8;
mod regw;
mod builder2;

use std::rc::Rc;
use std::cell::RefCell;

use self::reg8::Reg8;
use self::regw::RegW;

#[cfg(test)]
use self::builder2::CpuBuilder2;

// Status register bit positions
// Bit      7 6 5 4 3 2   1 0
// Position S Z X H X P/V N C
pub const S_MASK: u8 = 0x80;
pub const Z_MASK: u8 = 0x40;
pub const H_MASK: u8 = 0x10;
pub const PV_MASK: u8 = 0x4;
pub const N_MASK: u8 = 0x2;
pub const C_MASK: u8 = 0x1;

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub enum Register {
    a,
    b,
    c,
    d,
    e,
    h,
    l,
}

#[derive(Debug)]
pub struct Cpu {
    a: Rc<RefCell<u8>>,
    b: Rc<RefCell<u8>>,
    c: Rc<RefCell<u8>>,
    d: Rc<RefCell<u8>>,
    e: Rc<RefCell<u8>>,
    f: Rc<RefCell<u8>>,
    h: Rc<RefCell<u8>>,
    l: Rc<RefCell<u8>>,

    // Ghost registers should be u8?
    a1: Rc<RefCell<u8>>,
    b1: Rc<RefCell<u8>>,
    c1: Rc<RefCell<u8>>,
    d1: Rc<RefCell<u8>>,
    e1: Rc<RefCell<u8>>,
    f1: Rc<RefCell<u8>>,
    h1: Rc<RefCell<u8>>,
    l1: Rc<RefCell<u8>>,

    pub reg_a: Reg8,
    pub reg_b: Reg8,
    pub reg_c: Reg8,
    pub reg_d: Reg8,
    pub reg_e: Reg8,
    pub reg_f: Reg8,
    pub reg_h: Reg8,
    pub reg_l: Reg8,
    
    pub reg_af: RegW,
    pub reg_bc: RegW,
    pub reg_de: RegW,
    pub reg_hl: RegW,

    i: u8,
    r: u8,

    iff1: bool,
    iff2: bool,

    pc: u16,
    sp: u16,
    ix: u16,
    iy: u16,

    pub memory: Vec<u8>,
}

#[allow(dead_code)]
impl Cpu {
    // ===== FLAG S =====
    
    /// Read the sign flag. False means positive, true is negative.
    pub fn get_s(&self) -> bool {
        self.read_status_flag(S_MASK)
    }

    /// Set the sign flag. When value is positive, sign flag is 0,
    /// Positive intended as: MSB set. S flag is a copy of the MSB.
    pub fn set_s(&mut self, value: bool) {
        self.set_status_flag(S_MASK, value);
    }

    pub fn set_s_from_byte(&mut self, byte: u8) {
        // TODO: Verify that complies with Z80 impl
        self.set_s((byte & 0b1000_0000) != 0);
    }

    /// Verify that the sign flag is positive.
    pub fn sign_is_positive(&self) -> bool {
        !self.get_s()
    }

    /// Verify that the sign flag is negative.
    pub fn sign_is_negative(&self) -> bool {
        self.get_s()
    }

    // ===== FLAG Z =====

    /// Read the zero flag.
    pub fn get_z(&self) -> bool {
        self.read_status_flag(Z_MASK)
    }

    /// Set the zero flag. When accumulator is 0, zero flag is set.
    pub fn set_z(&mut self, value: bool) {
        self.set_status_flag(Z_MASK, value);
    }

    /// Given a value, updates the zero flag.
    pub fn set_z_from_byte(&mut self, byte: u8) {
        // TODO: Verify that complies with Z80 impl
        self.set_z(byte == 0);
    }

    // ===== FLAG C - Carry =====

    /// Read the carry flag.
    pub fn get_c(&self) -> bool {
        self.read_status_flag(C_MASK)
    }

    /// Set the carry flag.
    pub fn set_c(&mut self, value: bool) {
        self.set_status_flag(C_MASK, value);
    }

    // ===== FLAG H - Half carry =====

    ///
    pub fn set_h_from_byte(&mut self, _byte: u8) {
        // TODO: implement
    }

    /// Read the half-carry flag.
    pub fn get_h(&self) -> bool {
        self.read_status_flag(H_MASK)
    }

    /// Set the half-carry flag.
    pub fn set_h(&mut self, value: bool) {
        self.set_status_flag(H_MASK, value);
    }

    // ===== FLAG PV - Parity/overflow =====

    /// Read the parity/overflow flag.
    pub fn get_pv(&self) -> bool {
        self.read_status_flag(PV_MASK)
    }

    /// Set the parity/overflow flag.
    pub fn set_pv(&mut self, value: bool) {
        self.set_status_flag(PV_MASK, value);
    }

    pub fn set_pv_from_byte(&mut self, byte: u8) {
        // TODO: Verify that complies with Z80 impl
        self.set_pv((byte & 0b0000_0001) == 0);
    }

    pub fn parity_is_odd(&self) -> bool {
        !self.read_status_flag(PV_MASK)
    }

    pub fn parity_is_even(&self) -> bool {
        self.read_status_flag(PV_MASK)
    }

    // ===== FLAG N - Add/Subtract =====

    /// Read the add/subtract flag.
    pub fn get_n(&self) -> bool {
        self.read_status_flag(N_MASK)
    }

    /// Set the add/subtract flag.
    pub fn set_n(&mut self, value: bool) {
        self.set_status_flag(N_MASK, value);
    }

    // ===== Status register bit operations =====

    fn set_status_flag(&self, bitmask: u8, value: bool) {
        if value {
            *self.f.borrow_mut() |= bitmask;
        } else {
            *self.f.borrow_mut() &= !bitmask;
        }
    }

    fn read_status_flag(&self, bitmask: u8) -> bool {
        *self.f.borrow() & bitmask != 0
    }

    // === ===

    /// Identifies a register from patterns embedded
    /// in the object code.
    pub fn select(opcode: u8) -> Register {
        match opcode {
            0b111 => Register::a,
            0b000 => Register::b,
            0b001 => Register::c,
            0b010 => Register::d,
            0b011 => Register::e,
            0b100 => Register::h,
            0b101 => Register::l,
            _ => panic!(),
        }
    }

    /// Select the source register for the specified opcode.
    pub fn select_src(opcode: u8) -> Register {
        Self::select(opcode & 0b0000_0111)
    }

    /// Select the destination register for the specified opcode.
    pub fn select_dest(opcode: u8) -> Register {
        Self::select((opcode & 0b0011_1000) >> 3)
    }


    /// Read memory at address: pc + offset
    fn memory_at_pc(&self, offset_from_pc: u16) -> u8 {
        self.memory[(self.pc + offset_from_pc) as usize]
    }


    fn ld_r_r1(&mut self) {
        let opcode = self.memory_at_pc(0);

        let value = match Cpu::select_src(opcode) {
            Register::a => *self.a.borrow(),
            Register::b => *self.b.borrow(),
            Register::c => *self.c.borrow(),
            Register::d => *self.d.borrow(),
            Register::e => *self.e.borrow(),
            Register::h => *self.h.borrow(),
            Register::l => *self.l.borrow(),
        };

        match Cpu::select_dest(opcode) {
            Register::a => *self.a.borrow_mut() = value,
            Register::b => *self.b.borrow_mut() = value,
            Register::c => *self.c.borrow_mut() = value,
            Register::d => *self.d.borrow_mut() = value,
            Register::e => *self.e.borrow_mut() = value,
            Register::h => *self.h.borrow_mut() = value,
            Register::l => *self.l.borrow_mut() = value,
        };

        self.pc += 1;
    }
}

#[test]
fn ld_r_r1() {
    // Set initial cpu status and memory
    let mut cpu = CpuBuilder2::new()
        .with_memory(vec![
            0b01_100_011, // LD H,E
            0b01_001_010, // LD C,D
            0,
            0,
        ])
        .with_d(19)
        .with_e(26)
        .build();

    // Load registers while pc = 0
    cpu.ld_r_r1();
    assert_eq!(*cpu.a.borrow(), 0);
    assert_eq!(*cpu.b.borrow(), 0);
    assert_eq!(*cpu.c.borrow(), 0);
    assert_eq!(*cpu.f.borrow(), 0);
    assert_eq!(*cpu.h.borrow(), 26);
    assert_eq!(*cpu.l.borrow(), 0);
    assert_eq!(*cpu.f.borrow(), 0);
    assert_eq!(cpu.i, 0);
    assert_eq!(cpu.pc, 1);

    // Load registers while pc = 1
    cpu.ld_r_r1();
    assert_eq!(*cpu.c.borrow(), 19);
    assert_eq!(cpu.pc, 2);
}
