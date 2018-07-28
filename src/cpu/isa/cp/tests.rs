use cpu::Assertor;
use cpu::CpuBuilder;

// === 8-Bit Arithmetic Group / CP ===

#[test]
fn cp_r() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![0b10111_001, 0, 0, 0])
        .with_a(0b1100_0011)
        .with_c(0b1100_0011)
        .build();

    cpu.cp_r();

    Assertor::new(cpu)
        .register_a_is(0b1100_0011)
        .sign_flag_is_reset()
        .zero_flag_is_set()
        .half_carry_flag_is_reset()
        .carry_flag_is_reset()
        // .parity_is_odd()
        .add_subtract_flag_is_set();
}

#[test]
fn cp_n() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![0xfe, 0b1100_0011, 0, 0])
        .with_a(0b1100_0011)
        .build();

    cpu.cp_n();

    Assertor::new(cpu)
        .register_a_is(0b1100_0011)
        .sign_flag_is_reset()
        .zero_flag_is_set()
        .half_carry_flag_is_reset()
        .carry_flag_is_reset()
        // .parity_is_odd()
        .add_subtract_flag_is_set();
}

#[test]
fn cp_hli() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![0xbe, 0b1100_0011, 2, 3])
        .with_a(0b1100_0011)
        .with_hl(1)
        .build();

    cpu.cp_hli();

    Assertor::new(cpu)
        .register_a_is(0b1100_0011)
        .sign_flag_is_reset()
        .zero_flag_is_set()
        .half_carry_flag_is_reset()
        .carry_flag_is_reset()
        // .parity_is_odd()
        .add_subtract_flag_is_set();
}

#[test]
fn cp_ixdi() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![0xdd, 0xbe, 2, 0b1100_0011])
        .with_a(0b1100_0011)
        .with_ix(1)
        .build();

    cpu.cp_ixdi();

    Assertor::new(cpu)
        .register_a_is(0b1100_0011)
        .sign_flag_is_reset()
        .zero_flag_is_set()
        .half_carry_flag_is_reset()
        .carry_flag_is_reset()
        // .parity_is_odd()
        .add_subtract_flag_is_set();
}

#[test]
fn cp_iydi() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![0xfd, 0xbe, 2, 0b1100_0011])
        .with_a(0b1100_0011)
        .with_iy(1)
        .build();

    cpu.cp_iydi();

    Assertor::new(cpu)
        .register_a_is(0b1100_0011)
        .sign_flag_is_reset()
        .zero_flag_is_set()
        .half_carry_flag_is_reset()
        .carry_flag_is_reset()
        // .parity_is_odd()
        .add_subtract_flag_is_set();
}
