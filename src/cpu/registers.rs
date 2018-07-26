use cpu::Cpu;

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

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub enum Register16 {
    af,
    bc,
    de,
    hl,
    sp,
}

/// Additional register functionalities
pub trait RegisterOperations<T> {
    fn msb(&self) -> bool;
    fn lsb(&self) -> bool;
    fn incr(&mut self) -> (T, bool);
    fn decr(&mut self) -> (T, bool);
    fn reg_add(&mut self, value: T) -> (T, bool);
    fn reg_sub(&mut self, value: T) -> (T, bool);
    fn is_zero(&self) -> bool;
    fn set(&mut self, bitmask: T) -> T;
    fn reset(&mut self, bitmask: T) -> T;
}

/// Conversion between different size registers.
/// T is the bigger register, composed by two R-sized registers.
pub trait RegisterDemote<T> {
    fn high(&self) -> T;
    fn low(&self) -> T;
}

/// Defines register conversion to type T.
pub trait RegisterPromote<T> {
    fn promote(&self) -> T;
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

    pub fn set_s_from_msb(&mut self, byte: u8) {
        //self.set_s((byte & 0b1000_0000) != 0);
        self.set_s(byte.msb());
    }

    pub fn set_s_from_msbw(&mut self, word: u16) {
        self.set_s(word.msb());
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

    /// Convert carry flag to value.
    pub fn get_c_value(&self) -> u8 {
        if self.read_status_flag(C_MASK) {
            1
        } else {
            0
        }
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

    pub fn overflow_flag_is_set(&self) -> bool {
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

    fn set_status_flag(&mut self, bitmask: u8, value: bool) {
        if value {
            self.f = self.f | bitmask;
        } else {
            self.f = self.f & !bitmask;
        }
    }

    fn read_status_flag(&self, bitmask: u8) -> bool {
        self.f & bitmask != 0
    }

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

    /// Select the source register for the specified opcode.
    /// TODO: Add unit test
    pub fn select_reg16(opcode: u8) -> Register16 {
        match (opcode & 0b0011_0000) >> 4 {
            0b00 => Register16::bc,
            0b01 => Register16::de,
            0b10 => Register16::hl,
            0b11 => Register16::sp,
            _ => panic!(),
        }
    }

    pub fn select_push16(opcode: u8) -> Register16 {
        // TODO: Add unit test
        match (opcode & 0b0011_0000) >> 4 {
            0b00 => Register16::bc,
            0b01 => Register16::de,
            0b10 => Register16::hl,
            0b11 => Register16::af,
            _ => panic!(),
        }
    }

    pub fn read(&self, reg: Register) -> u8 {
        match reg {
            Register::a => self.a,
            Register::b => self.b,
            Register::c => self.c,
            Register::d => self.d,
            Register::e => self.e,
            Register::h => self.h,
            Register::l => self.l,
        }
    }

    pub fn write(&mut self, reg: Register, value: u8) {
        match reg {
            Register::a => self.a = value,
            Register::b => self.b = value,
            Register::c => self.c = value,
            Register::d => self.d = value,
            Register::e => self.e = value,
            Register::h => self.h = value,
            Register::l => self.l = value,
        }
    }

    pub fn read16(&mut self, reg: Register16) -> u16 {
        match reg {
            Register16::af => self.read_af(),
            Register16::bc => self.read_bc(),
            Register16::de => self.read_de(),
            Register16::hl => self.read_hl(),
            Register16::sp => self.sp,
        }
    }

    pub fn write16(&mut self, reg: Register16, value: u16) {
        match reg {
            Register16::af => self.write_af(value),
            Register16::bc => self.write_bc(value),
            Register16::de => self.write_de(value),
            Register16::hl => self.write_hl(value),
            Register16::sp => self.sp = value,
        }
    }

    // ===== Register AF =====

    pub fn read_af(&self) -> u16 {
        (self.a, self.f).promote()
    }

    pub fn write_af(&mut self, value: u16) {
        self.a = value.high();
        self.f = value.low();
    }

    // ===== Register BC =====

    pub fn read_bc(&self) -> u16 {
        (self.b, self.c).promote()
    }

    pub fn write_bc(&mut self, value: u16) {
        self.b = value.high();
        self.c = value.low();
    }

    pub fn add_bc(&mut self, value: i16) {
        // TODO: Verify it complies with z80
        let bc = (self.read_bc() as i16) + value;
        self.write_bc(bc as u16);
    }

    // ===== Register DE =====

    pub fn read_de(&self) -> u16 {
        (self.d, self.e).promote()
    }

    pub fn write_de(&mut self, value: u16) {
        self.d = value.high();
        self.e = value.low();
    }

    pub fn add_de(&mut self, value: i16) {
        // TODO: Verify it complies with z80
        let de = (self.read_de() as i16) + value;
        self.write_de(de as u16);
    }

    // ===== Register HL =====

    pub fn read_hl(&self) -> u16 {
        (self.h, self.l).promote()
    }

    pub fn write_hl(&mut self, value: u16) {
        self.h = value.high();
        self.l = value.low();
    }

    pub fn add_hl(&mut self, value: i16) {
        // TODO: Verify it complies with z80
        let hl = (self.read_hl() as i16) + value;
        self.write_hl(hl as u16);
    }
}

#[cfg(test)]
mod tests {
    use cpu::Cpu;
    use cpu::CpuBuilder;
    use cpu::Register;
    use cpu::Register16;
    use cpu::{C_MASK, H_MASK, N_MASK, PV_MASK, S_MASK, Z_MASK};

    #[test]
    fn get_s() {
        let mut cpu = CpuBuilder::new().with_memory_size(16).build();

        // Initial value is 0
        assert_eq!(cpu.get_s(), false);
        assert!(cpu.sign_is_positive());

        // Value is true
        cpu.set_s(true);
        assert_eq!(cpu.get_s(), true);
        assert!(cpu.sign_is_negative());

        // All other flags are still false
        assert_eq!(cpu.f, S_MASK);

        // Set all flags and clear s
        cpu.f = 0xff;
        cpu.set_s(false);
        // Value is not true
        assert_eq!(cpu.get_s(), false);
        // All other flags are unchanged
        assert_eq!(cpu.f, !S_MASK);
    }

    #[test]
    fn status_flag_z() {
        let mut cpu = CpuBuilder::new().with_memory_size(16).build();

        assert_eq!(cpu.get_z(), false);

        cpu.set_z(true);
        assert_eq!(cpu.get_z(), true);
        assert_eq!(cpu.f, Z_MASK);

        cpu.f = 0xff;
        cpu.set_z(false);
        assert_eq!(cpu.get_z(), false);
        assert_eq!(cpu.f, !Z_MASK);
    }

    #[test]
    fn status_flag_h() {
        let mut cpu = CpuBuilder::new().with_memory_size(16).build();

        assert_eq!(cpu.get_h(), false);

        cpu.set_h(true);
        assert_eq!(cpu.get_h(), true);
        assert_eq!(cpu.f, H_MASK);

        cpu.f = 0xff;
        cpu.set_h(false);
        assert_eq!(cpu.get_h(), false);
        assert_eq!(cpu.f, !H_MASK);
    }

    #[test]
    fn status_flag_pv() {
        let mut cpu = CpuBuilder::new().with_memory_size(16).build();

        assert_eq!(cpu.get_pv(), false);

        cpu.set_pv(true);
        assert_eq!(cpu.get_pv(), true);
        assert_eq!(cpu.f, PV_MASK);

        cpu.f = 0xff;
        cpu.set_pv(false);
        assert_eq!(cpu.get_pv(), false);
        assert_eq!(cpu.f, !PV_MASK);
    }

    #[test]
    fn status_flag_n() {
        let mut cpu = CpuBuilder::new().with_memory_size(16).build();

        assert_eq!(cpu.get_n(), false);

        cpu.set_n(true);
        assert_eq!(cpu.get_n(), true);
        assert_eq!(cpu.f, N_MASK);

        cpu.f = 0xff;
        cpu.set_n(false);
        assert_eq!(cpu.get_n(), false);
        assert_eq!(cpu.f, !N_MASK);
    }

    #[test]
    fn status_flag_c() {
        let mut cpu = CpuBuilder::new().with_memory_size(16).build();

        assert_eq!(cpu.get_c(), false);

        cpu.set_c(true);
        assert_eq!(cpu.get_c(), true);
        assert_eq!(cpu.f, C_MASK);

        cpu.f = 0xff;
        cpu.set_c(false);
        assert_eq!(cpu.get_c(), false);
        assert_eq!(cpu.f, !C_MASK);
    }

    #[test]
    fn select_src() {
        assert_eq!(Cpu::select_src(0b00_000_111), Register::a);
        assert_eq!(Cpu::select_src(0b00_000_000), Register::b);
        assert_eq!(Cpu::select_src(0b00_000_001), Register::c);
        assert_eq!(Cpu::select_src(0b00_000_010), Register::d);
        assert_eq!(Cpu::select_src(0b00_000_011), Register::e);
        assert_eq!(Cpu::select_src(0b00_000_100), Register::h);
        assert_eq!(Cpu::select_src(0b00_000_101), Register::l);
    }

    #[test]
    fn select_dest() {
        assert_eq!(Cpu::select_dest(0b00_111_000), Register::a);
        assert_eq!(Cpu::select_dest(0b00_000_000), Register::b);
        assert_eq!(Cpu::select_dest(0b00_001_000), Register::c);
        assert_eq!(Cpu::select_dest(0b00_010_000), Register::d);
        assert_eq!(Cpu::select_dest(0b00_011_000), Register::e);
        assert_eq!(Cpu::select_dest(0b00_100_000), Register::h);
        assert_eq!(Cpu::select_dest(0b00_101_000), Register::l);
    }

    #[test]
    fn read() {
        let mut cpu = CpuBuilder::new().with_memory_size(16).build();

        cpu.a = 1;
        cpu.b = 2;
        cpu.c = 3;
        cpu.d = 4;
        cpu.e = 5;
        cpu.h = 6;
        cpu.l = 7;

        assert_eq!(cpu.read(Register::a), 1);
        assert_eq!(cpu.read(Register::b), 2);
        assert_eq!(cpu.read(Register::c), 3);
        assert_eq!(cpu.read(Register::d), 4);
        assert_eq!(cpu.read(Register::e), 5);
        assert_eq!(cpu.read(Register::h), 6);
        assert_eq!(cpu.read(Register::l), 7);
    }

    #[test]
    fn write() {
        let mut cpu = CpuBuilder::new().with_memory_size(16).build();

        cpu.write(Register::a, 1);

        assert_eq!(cpu.a, 1);
        assert_eq!(cpu.b, 0);
        assert_eq!(cpu.c, 0);
        assert_eq!(cpu.d, 0);
        assert_eq!(cpu.e, 0);
        assert_eq!(cpu.h, 0);
        assert_eq!(cpu.l, 0);

        cpu.write(Register::b, 2);
        cpu.write(Register::c, 3);
        cpu.write(Register::d, 4);
        cpu.write(Register::e, 5);
        cpu.write(Register::h, 6);
        cpu.write(Register::l, 7);

        assert_eq!(cpu.b, 2);
        assert_eq!(cpu.c, 3);
        assert_eq!(cpu.d, 4);
        assert_eq!(cpu.e, 5);
        assert_eq!(cpu.h, 6);
        assert_eq!(cpu.l, 7);
    }

    #[test]
    fn read_hl() {
        let mut cpu = CpuBuilder::new().with_memory_size(16).build();

        cpu.h = 0xab;
        cpu.l = 0xcd;

        assert_eq!(cpu.read16(Register16::hl), 0xabcd as u16);
    }

    #[test]
    fn write_hl() {
        let mut cpu = CpuBuilder::new().with_memory_size(16).build();

        cpu.write_hl(0xabcd);

        assert_eq!(cpu.h, 0xab);
        assert_eq!(cpu.l, 0xcd);
    }

    #[test]
    fn read_bc() {
        let mut cpu = CpuBuilder::new().with_memory_size(16).build();

        cpu.b = 0xab;
        cpu.c = 0xcd;

        assert_eq!(cpu.read16(Register16::bc), 0xabcd as u16);
    }

    #[test]
    fn write_bc() {
        let mut cpu = CpuBuilder::new().with_memory_size(16).build();

        cpu.write_bc(0xabcd);

        assert_eq!(cpu.b, 0xab);
        assert_eq!(cpu.c, 0xcd);
    }

    #[test]
    fn read_de() {
        let mut cpu = CpuBuilder::new().with_memory_size(16).build();

        cpu.d = 0xab;
        cpu.e = 0xcd;

        assert_eq!(cpu.read16(Register16::de), 0xabcd as u16);
    }

    #[test]
    fn write_de() {
        let mut cpu = CpuBuilder::new().with_memory_size(16).build();

        cpu.write16(Register16::de, 0xabcd);

        assert_eq!(cpu.d, 0xab);
        assert_eq!(cpu.e, 0xcd);
    }
}
