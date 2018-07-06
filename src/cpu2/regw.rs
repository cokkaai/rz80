use std::rc::Rc;
use std::cell::RefCell;
use cpu2::bytes;

#[derive(Debug)]
pub struct RegW {
    high: Rc<RefCell<u8>>,
    low: Rc<RefCell<u8>>,
}

#[allow(dead_code)]
impl RegW {
    pub fn new(high: Rc<RefCell<u8>>, low: Rc<RefCell<u8>>) -> RegW {
        RegW {
            high: high,
            low: low,
        }
    }

    pub fn new_with_value(high: Rc<RefCell<u8>>, low: Rc<RefCell<u8>>, value: u16) -> RegW {
        let reg = RegW {
            high: high,
            low: low,
        };

        *reg.high.borrow_mut() = bytes::high(value);
        *reg.low.borrow_mut() = bytes::low(value);

        reg
    }

    fn promote(&self) -> u16 {
        bytes::promote(*self.high.borrow(), *self.low.borrow())
    }

    fn store(&self, value: u16) {
        *self.high.borrow_mut() = bytes::high(value);
        *self.low.borrow_mut() = bytes::low(value);
    }

    pub fn msb(&self) -> bool {
        (*self.high.borrow() & 0b1000_0000) != 0
    }
    
    pub fn lsb(&self) -> bool {
        (*self.low.borrow() & 0b0000_0001) != 0
    }

    pub fn incr(&mut self) -> (u16, bool) {
        self.add(1)
    }

    pub fn decr(&mut self) -> (u16, bool) {
        self.sub(1)
    }

    pub fn add(&mut self, value: u16) -> (u16, bool) {
        let (result, overflow) = self.promote().overflowing_add(value);
        self.store(result);
        (result, overflow)
    }

    pub fn sub(&mut self, value: u16) -> (u16, bool) {
        let (result, overflow) = self.promote().overflowing_sub(value);
        self.store(result);
        (result, overflow)
    }

    pub fn is_zero(&self) -> bool {
        // TODO: Check if testing u8 is correct
        // or if i8 should be tested instead.
        self.promote() == 0
    }

    pub fn set(&mut self, bitmask: u16) {
        let result = self.promote() | bitmask;
        self.store(result);
    }

    /// Zero bits in the bitmask will be reset in the register.
    pub fn reset(&mut self, bitmask: u16) {
        let result = self.promote() & bitmask;
        self.store(result);
    }
}