#[cfg(test)]
mod test {
    use cpu::CpuBuilder;
    use cpu::Assertor;

    // === 8-Bit Arithmetic Group / ADD ===

    #[test]
    fn add_a_r() {
        let mut cpu = CpuBuilder::new()
            .with_memory(vec![0b10000_001, 0x66, 0x66, 0x66,])
            .with_a(7)
            .with_c(4)
            .build();

        cpu.add_a_r();

        Assertor::new(cpu)
            .register_a_is(11)
            .zero_flag_is_reset()
            .program_counter_is(1);
    }

    #[test]
    fn add_a_n() {
        unimplemented!();
    }

    #[test]
    fn add_a_hli() {
        unimplemented!();
    }

    #[test]
    fn add_a_ixd() {
        unimplemented!();
    }

    #[test]
    fn add_a_iyd() {
        unimplemented!();
    }
}
