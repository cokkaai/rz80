use cpu::RegisterOperations;

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
        // TODO: Check if compliant with z80 hw.
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
        // TODO: Check if testing u8 is correct
        // or if i8 should be tested instead.
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

}

// #[test]
// fn add_10_to_u8_binding() {
//     let mut data = 9u8;

//     let (result, overflow) = data.reg_add(10);

//     assert_eq!(result, 19);
//     assert_eq!(data, 19);
//     assert_eq!(overflow, false);
// }

// #[test]
// fn add_10_to_u8_part_of_a_struct() {
//     struct Container {
//         data: u8
//     }

//     let mut container = Container {
//         data: 9u8
//     };

//     let (result, overflow) = container.data.reg_add(10);

//     assert_eq!(result, 19);
//     assert_eq!(container.data, 19);
//     assert_eq!(overflow, false);
// }

// #[test]
// fn overflow_u8() {
//     let mut data = 1u8;

//     let (result, overflow) = data.reg_add(u8::max_value());

//     assert_eq!(result, 0);
//     assert_eq!(data, 0);
//     assert_eq!(overflow, true);
// }
