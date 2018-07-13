#[cfg(test)]
mod test {
    use cpu::CpuBuilder;
    use cpu::Assertor;

    // === 8-Bit Arithmetic Group / AND ===

    #[test]
    fn and_r() {
        let mut cpu = CpuBuilder::new()
            .with_memory(vec!(0b10000_001, 1, 2, 3))
            .with_a(0b1100_0011)
            .with_c(0b0111_1011)
            .build();
        
        cpu.and_r();

        Assertor::new(cpu)
            .register_a_is(0b0100_0011)
            .sign_flag_is_positive()
            .zero_flag_is_reset()
            .half_carry_flag_is_set()
            .carry_flag_is_reset()
            .parity_overflow_flag_is_reset()
            .add_substract_flag_is_reset();
    }

    #[test]
    fn and_n() {
        let mut cpu = CpuBuilder::new()
            .with_memory(vec!(0xc6, 0b0111_1011, 2, 3))
            .with_a(0b1100_0011)
            .build();
        
        cpu.and_n();

        Assertor::new(cpu)
            .register_a_is(0b0100_0011)
            .sign_flag_is_positive()
            .zero_flag_is_reset()
            .half_carry_flag_is_set()
            .carry_flag_is_reset()
            .parity_overflow_flag_is_reset()
            .add_substract_flag_is_reset();
    }

    #[test]
    fn and_hli() {
        let mut cpu = CpuBuilder::new()
            .with_memory(vec!(0xb6, 0b0111_1011, 2, 3))
            .with_a(0b1100_0011)
            .with_hl(1)
            .build();
        
        cpu.and_hli();

        Assertor::new(cpu)
            .register_a_is(0b0100_0011)
            .sign_flag_is_positive()
            .zero_flag_is_reset()
            .half_carry_flag_is_set()
            .carry_flag_is_reset()
            .parity_overflow_flag_is_reset()
            .add_substract_flag_is_reset();
    }

    #[test]
    fn and_ixdi() {
        let mut cpu = CpuBuilder::new()
            .with_memory(vec!(0xdd, 0xa6, 2, 0b0111_1011))
            .with_a(0b1100_0011)
            .with_ix(1)
            .build();
        
        cpu.and_ixdi();

        Assertor::new(cpu)
            .register_a_is(0b0100_0011)
            .sign_flag_is_positive()
            .zero_flag_is_reset()
            .half_carry_flag_is_set()
            .carry_flag_is_reset()
            .parity_overflow_flag_is_reset()
            .add_substract_flag_is_reset();
    }

    #[test]
    fn and_iydi() {
        let mut cpu = CpuBuilder::new()
            .with_memory(vec!(0xdd, 0xa6, 2, 0b0111_1011))
            .with_a(0b1100_0011)
            .with_iy(1)
            .build();
        
        cpu.and_iydi();

        Assertor::new(cpu)
            .register_a_is(0b0100_0011)
            .sign_flag_is_positive()
            .zero_flag_is_reset()
            .half_carry_flag_is_set()
            .carry_flag_is_reset()
            .parity_overflow_flag_is_reset()
            .add_substract_flag_is_reset();
    }
}
