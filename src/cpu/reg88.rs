use cpu::RegisterDemote;
use cpu::RegisterOperations;
use cpu::RegisterPromote;

impl RegisterDemote<(u8, u8), u8> for (u8, u8) {
    fn high(&self) -> u8 {
        self.0
    }

    fn low(&self) -> u8 {
        self.1
    }
}

impl RegisterPromote<u16> for (u8, u8) {
    fn promote(&self) -> u16 {
        ((self.0 as u16) << 8) + self.1 as u16
    }
}

impl RegisterOperations<u16> for (u8, u8) {
    fn msb(&self) -> bool {
        (self.0 & 0x80) != 0
    }

    fn lsb(&self) -> bool {
        (self.1 & 0x01) != 0
    }

    fn incr(&mut self) -> (u16, bool) {
        self.reg_add(1)
    }

    fn decr(&mut self) -> (u16, bool) {
        self.reg_sub(1)
    }

    fn reg_add(&mut self, value: u16) -> (u16, bool) {
        // let (result, overflow) = bytes::promote(self.0, self.1).overflowing_add(value);
        // TODO: Check if compliant with z80 hw.
        let (result, overflow) = self.promote().overflowing_add(value);

        self.0 = result.high();
        self.1 = result.low();
        
        (result, overflow)
    }

    fn reg_sub(&mut self, value: u16) -> (u16, bool) {
        // let (result, overflow) = self.overflowing_sub(value);
        let (result, overflow) = self.promote().overflowing_sub(value);

        self.0 = result.high();
        self.1 = result.low();
        
        (result, overflow)
    }

    fn is_zero(&self) -> bool {
        // TODO: Check if testing u8 is correct
        // or if i8 should be tested instead.
        self.0 == 0 && self.1 == 0
    }

    fn set(&mut self, bitmask: u16) -> u16 {
        self.0 |= bitmask.high();
        self.1 |= bitmask.low();
        self.promote()
    }

    /// Zero bits in the bitmask will be reset in the register.
    fn reset(&mut self, bitmask: u16) -> u16 {
        self.0 &= bitmask.high();
        self.1 &= bitmask.low();
        self.promote()
    }
}

#[cfg(test)]
mod test {
    use cpu::RegisterDemote;
    use cpu::RegisterOperations;
    use cpu::RegisterPromote;

    #[test]
    fn msb() {
        assert_eq!((0x80u8, 0x00u8).msb(), true);
        assert_eq!((0x00, 0x01u8).msb(), false);
        assert_eq!((0x00, 0x00u8).msb(), false);
    }

    #[test]
    fn lsb() {
        assert_eq!((0x80, 0x00u8).lsb(), false);
        assert_eq!((0x00, 01u8).lsb(), true);
        assert_eq!((0x00, 0x00u8).lsb(), false);
    }

    #[test]
    fn incr() {
        assert_eq!((0x00u8, 0x00u8).incr(), (0x0001u16, false));
        assert_eq!((0x00u8, 0xfeu8).incr(), (0x00ffu16, false));
        assert_eq!((0x00u8, 0xffu8).incr(), (0x0100u16, false));
        assert_eq!((0xffu8, 0xfeu8).incr(), (0xffffu16, false));
        assert_eq!((0xffu8, 0xffu8).incr(), (0x0000u16, true));
    }

    #[test]
    fn decr() {
        assert_eq!(0xffffu16.decr(), (0xfffeu16, false));
        assert_eq!(0x0100u16.decr(), (0x00ffu16, false));
        assert_eq!(0x00ffu16.decr(), (0x00feu16, false));
        assert_eq!(1u16.decr(), (0u16, false));
        assert_eq!(0u16.decr(), (0xffffu16, true));
    }

    #[test]
    fn add_10_to_u16_binding() {
        let mut data = 9u16;

        let (result, overflow) = data.reg_add(10);

        assert_eq!(result, 19);
        assert_eq!(data, 19);
        assert_eq!(overflow, false);
    }

    #[test]
    fn add_10_to_u6_part_of_a_struct() {
        struct Container {
            data: u16,
        }

        let mut container = Container { data: 9u16 };

        let (result, overflow) = container.data.reg_add(10);

        assert_eq!(result, 19);
        assert_eq!(container.data, 19);
        assert_eq!(overflow, false);
    }

    #[test]
    fn overflow_u16() {
        let mut data = 1u16;

        let (result, overflow) = data.reg_add(u16::max_value());

        assert_eq!(result, 0);
        assert_eq!(data, 0);
        assert_eq!(overflow, true);
    }

    #[test]
    fn sub() {
        assert_eq!(0xffffu16.reg_sub(10), (0xfff5u16, false));
        assert_eq!(65535u16.reg_sub(65535), (0u16, false));
        assert_eq!(100u16.reg_sub(255), (65381u16, true));
    }

    #[test]
    fn zero() {
        assert_eq!(1u16.is_zero(), false);
        assert_eq!(0u16.is_zero(), true);
    }

    #[test]
    fn set() {
        assert_eq!(0u16.set(14), 14);
        assert_eq!(1u16.set(2), 3);
        assert_eq!(1u16.set(7), 7);
    }

    #[test]
    fn reset() {
        assert_eq!(14u16.reset(0), 0);
        assert_eq!(0xcafeu16.reset(0x0003u16), 2);
        assert_eq!(0xca01u16.reset(0x00ffu16), 1);
    }

    #[test]
    fn high() {
        assert_eq!((0xcau8, 0xfeu8).high(), 0xcau8);
    }

    #[test]
    fn low() {
        assert_eq!((0xcau8, 0xfeu8).low(), 0xfeu8);
    }

    #[test]
    fn promote() {
        assert_eq!((0xcau8, 0xfeu8).promote(), 0xcafe);
    }
}
