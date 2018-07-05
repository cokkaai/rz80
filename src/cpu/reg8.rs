use cpu::RegisterOperations;

impl RegisterOperations<u8> for u8 {
    fn msb(&self) -> bool {
        (*self & 0x80) != 0
    }

    fn lsb(&self) -> bool {
        (*self & 0x01) != 0
    }

    fn incr(&mut self) -> (u8, bool) {
        self.reg_add(1)
    }

    fn decr(&mut self) -> (u8, bool) {
        self.reg_sub(1)
    }

    fn reg_add(&mut self, value: u8) -> (u8, bool) {
        // TODO: Check if compliant with z80 hw.
        let (result, overflow) = self.overflowing_add(value);
        *self = result;
        (result, overflow)
    }

    fn reg_sub(&mut self, value: u8) -> (u8, bool) {
        let (result, overflow) = self.overflowing_sub(value);
        *self = result;
        (result, overflow)
    }

    fn is_zero(&self) -> bool {
        // TODO: Check if testing u8 is correct
        // or if i8 should be tested instead.
        *self == 0
    }

    fn set(&mut self, bitmask: u8) -> u8 {
        *self |= bitmask;
        *self
    }

    /// Zero bits in the bitmask will be reset in the register.
    fn reset(&mut self, bitmask: u8) -> u8{
        *self &= bitmask;
        *self
    }
}

#[cfg(test)]
mod test {
    use cpu::registers::RegisterOperations;

    #[test]
    fn msb() {
        assert_eq!(0x80u8.msb(), true);
        assert_eq!(0x00u8.msb(), false);
    }

    #[test]
    fn lsb() {
        assert_eq!(0x01u8.lsb(), true);
        assert_eq!(0x00u8.lsb(), false);
    }

    #[test]
    fn incr() {
        assert_eq!(0u8.incr(), (1u8, false));
        assert_eq!(254u8.incr(), (255u8, false));
        assert_eq!(255u8.incr(), (0u8, true));
    }

    #[test]
    fn decr() {
        assert_eq!(255u8.decr(), (254u8, false));
        assert_eq!(1u8.decr(), (0u8, false));
        assert_eq!(0u8.decr(), (255u8, true));
    }

    #[test]
    fn add_10_to_u8_binding() {
        let mut data = 9u8;

        let (result, overflow) = data.reg_add(10);

        assert_eq!(result, 19);
        assert_eq!(data, 19);
        assert_eq!(overflow, false);
    }

    #[test]
    fn add_10_to_u8_part_of_a_struct() {
        struct Container {
            data: u8,
        }

        let mut container = Container { data: 9u8 };

        let (result, overflow) = container.data.reg_add(10);

        assert_eq!(result, 19);
        assert_eq!(container.data, 19);
        assert_eq!(overflow, false);
    }

    #[test]
    fn overflow_u8() {
        let mut data = 1u8;

        let (result, overflow) = data.reg_add(u8::max_value());

        assert_eq!(result, 0);
        assert_eq!(data, 0);
        assert_eq!(overflow, true);
    }

    #[test]
    fn sub() {
        assert_eq!(255u8.reg_sub(10), (245u8, false));
        assert_eq!(255u8.reg_sub(255), (0u8, false));
        assert_eq!(100u8.reg_sub(255), (101u8, true));
    }

    #[test]
    fn zero() {
        assert_eq!(1u8.is_zero(), false);
        assert_eq!(0u8.is_zero(), true);
    }

    #[test]
    fn set() {
        assert_eq!(0u8.set(14), 14);
        assert_eq!(1u8.set(2), 3);
        assert_eq!(1u8.set(7), 7);
    }

    #[test]
    fn reset() {
        assert_eq!(14u8.reset(0), 0);
        assert_eq!(0b0000_0001u8.reset(0b0000_0011u8), 1);
        assert_eq!(0b0000_0001u8.reset(0b0000_1111u8), 1);
    }
}
