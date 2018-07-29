use cpu::Cpu;
use cpu::RegisterPromote;

/// An assertor for CPU status modeled as a builder

#[derive(Debug)]
pub struct Assertor {
    cpu: Cpu,
}

#[allow(dead_code)]
impl Assertor {
    pub fn new(cpu: Cpu) -> Assertor {
        Assertor { cpu: cpu }
    }

    /// Tests if the S flag is false
    #[inline]
    pub fn sign_flag_is_reset(&self) -> &Assertor {
        assert_eq!(self.cpu.get_s(), false, "Flag S unexpected value");
        self
    }

    #[inline]
    pub fn sign_is_positive(&self) -> &Assertor {
        self.sign_flag_is_reset()
    }

    /// Tests if the S flag is true
    #[inline]
    pub fn sign_flag_is_set(&self) -> &Assertor {
        assert_eq!(self.cpu.get_s(), true, "Flag S unexpected value");
        self
    }

    #[inline]
    pub fn sign_is_negative(&self) -> &Assertor {
        self.sign_flag_is_set()
    }

    /// Tests if the Z flag is true
    pub fn zero_flag_is_set(&self) -> &Assertor {
        assert_eq!(self.cpu.get_z(), true, "Flag Z unexpected value");
        self
    }

    /// Tests if the Z flag is false
    pub fn zero_flag_is_reset(&self) -> &Assertor {
        assert_eq!(self.cpu.get_z(), false, "Flag Z unexpected value");
        self
    }

    /// Tests if the half carry (H) flag is true
    pub fn half_carry_flag_is_set(&self) -> &Assertor {
        assert_eq!(self.cpu.get_h(), true, "Flag H carry flag unexpected value");
        self
    }

    /// Tests if the half carry (H) flag is false
    pub fn half_carry_flag_is_reset(&self) -> &Assertor {
        assert_eq!(self.cpu.get_h(), false, "Flag H flag unexpected value");
        self
    }

    /// Tests if the C flag is true
    pub fn carry_flag_is_set(&self) -> &Assertor {
        assert_eq!(self.cpu.get_c(), true, "Flag C unexpected value");
        self
    }

    /// Tests if the C flag is false
    pub fn carry_flag_is_reset(&self) -> &Assertor {
        assert_eq!(self.cpu.get_c(), false, "Flag C unexpected value");
        self
    }

    /// Tests if the PV flag is true
    pub fn parity_overflow_flag_is_set(&self) -> &Assertor {
        assert!(self.cpu.overflow_flag_is_set());
        self
    }

    /// Tests if the PV flag is false
    pub fn parity_overflow_flag_is_reset(&self) -> &Assertor {
        assert!(!self.cpu.overflow_flag_is_set());
        self
    }

    pub fn parity_is_odd(&self) -> &Assertor {
        assert!(self.cpu.parity_is_odd());
        self
    }

    pub fn parity_is_even(&self) -> &Assertor {
        assert!(self.cpu.parity_is_even());
        self
    }

    /// Tests if the add/substract (N) flag is true
    pub fn add_subtract_flag_is_set(&self) -> &Assertor {
        assert_eq!(self.cpu.get_n(), true, "N flag unexpected value");
        self
    }

    /// Tests if the add/substract (N) flag is false
    pub fn add_subtract_flag_is_reset(&self) -> &Assertor {
        assert_eq!(self.cpu.get_n(), false, "N flag unexpected value");
        self
    }

    /// Tests if flag 3 of the status register is true
    pub fn flag_3_is_set(&self) -> &Assertor {
        unimplemented!();
    }

    /// Tests if flag 3 of the status register is false
    pub fn flag_3_is_reset(&self) -> &Assertor {
        unimplemented!();
    }

    /// Tests if flag 5 of the status register is true
    pub fn flag_5_is_set(&self) -> &Assertor {
        unimplemented!();
    }

    /// Tests if flag 5 of the status register is false
    pub fn flag_5_is_reset(&self) -> &Assertor {
        unimplemented!();
    }

    /// Tests the PC register value
    pub fn program_counter_is(&self, value: u16) -> &Assertor {
        assert_eq!(self.cpu.pc, value, "PC unexpected value");
        self
    }

    /// Tests the SP register value
    pub fn stack_pointer_is(&self, value: u16) -> &Assertor {
        assert_eq!(self.cpu.sp, value, "SP unexpected value");
        self
    }

    /// Tests the IX register value
    pub fn index_register_ix_is(&self, value: u16) -> &Assertor {
        assert_eq!(self.cpu.ix, value, "Register IX unexpected value");
        self
    }

    /// Tests the IY register value
    pub fn index_register_iy_is(&self, value: u16) -> &Assertor {
        assert_eq!(self.cpu.iy, value, "Register IY unexpected value");
        self
    }

    /// Tests the A register value
    pub fn register_a_is(&self, value: u8) -> &Assertor {
        assert_eq!(self.cpu.a, value, "Register A unexpected value");
        self
    }

    /// Tests the B register value
    pub fn register_b_is(&self, value: u8) -> &Assertor {
        assert_eq!(self.cpu.b, value, "Register B unexpected value");
        self
    }

    /// Tests the C register value
    pub fn register_c_is(&self, value: u8) -> &Assertor {
        assert_eq!(self.cpu.c, value, "Register C unexpected value");
        self
    }

    /// Tests the D register value
    pub fn register_d_is(&self, value: u8) -> &Assertor {
        assert_eq!(self.cpu.d, value, "Register D unexpected value");
        self
    }

    /// Tests the E register value
    pub fn register_e_is(&self, value: u8) -> &Assertor {
        assert_eq!(self.cpu.e, value, "Register E unexpected value");
        self
    }

    /// Tests the F register value
    pub fn register_f_is(&self, value: u8) -> &Assertor {
        assert_eq!(self.cpu.f, value, "Register F unexpected value");
        self
    }

    /// Tests the H register value
    pub fn register_h_is(&self, value: u8) -> &Assertor {
        assert_eq!(self.cpu.h, value, "Register H unexpected value");
        self
    }

    /// Tests the L register value
    pub fn register_l_is(&self, value: u8) -> &Assertor {
        assert_eq!(self.cpu.l, value, "Register L unexpected value");
        self
    }

    /// Tests the A1 register value
    pub fn register_a1_is(&self, value: u8) -> &Assertor {
        assert_eq!(self.cpu.a1, value, "Register A1 unexpected value");
        self
    }

    /// Tests the B1 register value
    pub fn register_b1_is(&self, value: u8) -> &Assertor {
        assert_eq!(self.cpu.b1, value, "Register B1 unexpected value");
        self
    }

    /// Tests the C1 register value
    pub fn register_c1_is(&self, value: u8) -> &Assertor {
        assert_eq!(self.cpu.c1, value, "Register C1 unexpected value");
        self
    }

    /// Tests the D1 register value
    pub fn register_d1_is(&self, value: u8) -> &Assertor {
        assert_eq!(self.cpu.d1, value, "Register D1 unexpected value");
        self
    }

    /// Tests the E1 register value
    pub fn register_e1_is(&self, value: u8) -> &Assertor {
        assert_eq!(self.cpu.e1, value, "Register E1 unexpected value");
        self
    }

    /// Tests the F1 register value
    pub fn register_f1_is(&self, value: u8) -> &Assertor {
        assert_eq!(self.cpu.f1, value, "Register F1 unexpected value");
        self
    }

    /// Tests the H1 register value
    pub fn register_h1_is(&self, value: u8) -> &Assertor {
        assert_eq!(self.cpu.h1, value, "Register H1 unexpected value");
        self
    }

    /// Tests the L1 register value
    pub fn register_l1_is(&self, value: u8) -> &Assertor {
        assert_eq!(self.cpu.l1, value, "Register L1 unexpected value");
        self
    }

    /// Tests the AF register value
    pub fn register_af_is(&self, value: u16) -> &Assertor {
        assert_eq!(
            (self.cpu.a, self.cpu.f).promote(),
            value,
            "Register AF unexpected value"
        );
        self
    }

    /// Tests the BC register value
    pub fn register_bc_is(&self, value: u16) -> &Assertor {
        assert_eq!(
            (self.cpu.b, self.cpu.c).promote(),
            value,
            "Register BC unexpected value"
        );
        self
    }

    /// Tests the DE register value
    pub fn register_de_is(&self, value: u16) -> &Assertor {
        assert_eq!(
            (self.cpu.d, self.cpu.e).promote(),
            value,
            "Register DE unexpected value"
        );
        self
    }

    /// Tests the HL register value
    pub fn register_hl_is(&self, value: u16) -> &Assertor {
        println!("{:?}", self.cpu);
        assert_eq!(
            (self.cpu.h, self.cpu.l).promote(),
            value,
            "Register HL unexpected value"
        );
        self
    }

    /// Tests the I register value
    pub fn interrupt_vector_is(&self, value: u8) -> &Assertor {
        assert_eq!(self.cpu.i, value, "Register I unexpected value");
        self
    }

    /// Tests the R register value
    pub fn memory_refresh_register_is(&self, value: u8) -> &Assertor {
        assert_eq!(self.cpu.r, value, "Register R unexpected value");
        self
    }

    /// Tests if the IFF1 flip flop is set
    pub fn interrupt_flip_flop_1_is_set(&self) -> &Assertor {
        assert_eq!(self.cpu.iff1, true, "IFF1 unexpected value");
        self
    }

    /// Tests if the IFF1 flip flop is reset
    pub fn interrupt_flip_flop_1_is_reset(&self) -> &Assertor {
        assert_eq!(self.cpu.iff1, false, "IFF1 unexpected value");
        self
    }

    /// Tests the IFF2 flip flop is set
    pub fn interrupt_flip_flop_2_is_set(&self) -> &Assertor {
        assert_eq!(self.cpu.iff2, true, "IFF2 unexpected value");
        self
    }

    /// Tests the IFF2 flip flop is reset
    pub fn interrupt_flip_flop_2_is_reset(&self) -> &Assertor {
        assert_eq!(self.cpu.iff2, false, "IFF2 unexpected value");
        self
    }

    pub fn memory_at_address_is(&self, addr: usize, value: u8) -> &Assertor {
        assert_eq!(
            self.cpu.memory[addr], value,
            "Memory location unexpected value"
        );
        self
    }

    pub fn memory_size_is(&self, size: usize) -> &Assertor {
        assert_eq!(
            self.cpu.memory.capacity(),
            size,
            "Memory size unexpected value"
        );
        self
    }

    // pub fn xxx(&self) -> &Assertor {
    //     self
    // }
}
