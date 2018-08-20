use cpu::RegisterDemote;
use cpu::RegisterOperations;

impl RegisterDemote<u8> for u16 {
    fn high(&self) -> u8 {
        ((*self & 0xff00) >> 8) as u8
    }

    fn low(&self) -> u8 {
        (*self & 0x00ff) as u8
    }
}

impl RegisterOperations<u16> for u16 {
    fn msb(&self) -> bool {
        (*self & 0x8000) != 0
    }

    fn lsb(&self) -> bool {
        (*self & 0x0001) != 0
    }

    fn incr(&mut self) -> (u16, bool) {
        self.reg_add(1)
    }

    fn decr(&mut self) -> (u16, bool) {
        self.reg_sub(1)
    }

    fn reg_add(&mut self, value: u16) -> (u16, bool) {
        let (result, overflow) = self.overflowing_add(value);
        *self = result;
        (result, overflow)
    }

    fn reg_sub(&mut self, value: u16) -> (u16, bool) {
        let (result, overflow) = self.overflowing_sub(value);
        *self = result;
        (result, overflow)
    }

    fn is_zero(&self) -> bool {
        *self == 0
    }

    fn set(&mut self, bitmask: u16) -> u16 {
        *self |= bitmask;
        *self
    }

    /// Zero bits in the bitmask will be reset in the register.
    fn reset(&mut self, bitmask: u16) -> u16 {
        *self &= bitmask;
        *self
    }

    fn two_compl(&mut self) -> u16 {
        *self = (!*self).wrapping_add(1);
        *self
    }
}

#[cfg(test)]
mod test {
    use cpu::RegisterOperations;
    use cpu::RegisterDemote;

    #[test]
    fn msb() {
        assert_eq!(0x8000u16.msb(), true);
        assert_eq!(0x0001u16.msb(), false);
        assert_eq!(0x0000u16.msb(), false);
    }

    #[test]
    fn lsb() {
        assert_eq!(0x8000u16.lsb(), false);
        assert_eq!(0x0001u16.lsb(), true);
        assert_eq!(0x0000u16.lsb(), false);
    }

    #[test]
    fn incr() {
        assert_eq!(0u16.incr(), (1u16, false));
        assert_eq!(254u16.incr(), (255u16, false));
        assert_eq!(65534u16.incr(), (65535u16, false));
        assert_eq!(65535u16.incr(), (0u16, true));
    }

    #[test]
    fn decr() {
        assert_eq!(65535u16.decr(), (65534u16, false));
        assert_eq!(255u16.decr(), (254u16, false));
        assert_eq!(1u16.decr(), (0u16, false));
        assert_eq!(0u16.decr(), (65535u16, true));
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
        assert_eq!(65535u16.reg_sub(10), (65525u16, false));
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
        assert_eq!(0xcafeu16.high(), 0xcau8);
    }

    #[test]
    fn low() {
        assert_eq!(0xcafeu16.low(), 0xfeu8);
    }
}
