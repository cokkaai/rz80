use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
pub struct Reg8 {
    data: Rc<RefCell<u8>>,
}

#[allow(dead_code)]
impl Reg8 {
    pub fn new(data: Rc<RefCell<u8>>) -> Reg8 {
        Reg8 {
            data: data
        }
    }

    pub fn new_with_value(data: Rc<RefCell<u8>>, value :u8) -> Reg8 {
        let reg = Reg8 {
            data: data
        };

        *reg.data.borrow_mut() = value;

        reg
    }

    pub fn msb(&self) -> bool {
        (*self.data.borrow() & 0b1000_0000) != 0
    }
    
    pub fn lsb(&self) -> bool {
        (*self.data.borrow() & 0b0000_0001) != 0
    }

    pub fn incr(&mut self) -> (u8, bool) {
        self.add(1)
    }

    pub fn decr(&mut self) -> (u8, bool) {
        self.sub(1)
    }

    pub fn add(&mut self, value: u8) -> (u8, bool) {
        // TODO: Check if compliant with z80 hw.
        // pub fn overflowing_add(self, rhs: u8) -> (u8, bool)	1.7.0
        // Calculates self + rhs
        // Returns a tuple of the addition along with a boolean indicating
        // whether an arithmetic overflow would occur. If an overflow would
        // have occurred then the wrapped value is returned.
        let (result, overflow) = (*self.data.borrow()).overflowing_add(value);
        *self.data.borrow_mut() = result;
        (result, overflow)
    }

    pub fn sub(&mut self, value: u8) -> (u8, bool) {
        // Old impl
        // *self.data.borrow_mut() -= value;
        // *self.data.borrow()
        let (result, overflow) = (*self.data.borrow()).overflowing_sub(value);
        *self.data.borrow_mut() = result;
        (result, overflow)
    }

    pub fn is_zero(&self) -> bool {
        // TODO: Check if testing u8 is correct
        // or if i8 should be tested instead.
        *self.data.borrow() == 0
    }

    pub fn set(&mut self, bitmask: u8) {
        *self.data.borrow_mut() |= bitmask;
    }

    /// Zero bits in the bitmask will be reset in the register.
    pub fn reset(&mut self, bitmask: u8) {
        *self.data.borrow_mut() &= bitmask;
    }
}