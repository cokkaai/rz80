use cpu::Assertor;
use cpu::CpuBuilder;

// === 8-Bit Arithmetic Group / XOR ===

#[test]
fn xor_r() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![0b10101_001, 1, 2, 3])
        .with_a(0b1100_0011)
        .with_c(0b0111_1011)
        .build();

    cpu.xor_r();

    Assertor::new(cpu)
        .register_a_is(0b1011_1000)
        .sign_flag_is_negative()
        .zero_flag_is_reset()
        .half_carry_flag_is_reset()
        .carry_flag_is_reset()
        .parity_overflow_flag_is_reset()
        .add_subtract_flag_is_reset();
}

#[test]
fn xor_n() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![0xee, 0b0111_1011, 2, 3])
        .with_a(0b1100_0011)
        .build();

    cpu.xor_n();

    Assertor::new(cpu)
        .register_a_is(0b1011_1000)
        .sign_flag_is_negative()
        .zero_flag_is_reset()
        .half_carry_flag_is_reset()
        .carry_flag_is_reset()
        .parity_overflow_flag_is_reset()
        .add_subtract_flag_is_reset();
}

#[test]
fn xor_hli() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![0xae, 0b0111_1011, 2, 3])
        .with_a(0b1100_0011)
        .with_hl(1)
        .build();

    cpu.xor_hli();

    Assertor::new(cpu)
        .register_a_is(0b1011_1000)
        .sign_flag_is_negative()
        .zero_flag_is_reset()
        .half_carry_flag_is_reset()
        .carry_flag_is_reset()
        .parity_overflow_flag_is_reset()
        .add_subtract_flag_is_reset();
}

#[test]
fn xor_ixdi() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![0xdd, 0xae, 2, 0b0111_1011])
        .with_a(0b1100_0011)
        .with_ix(1)
        .build();

    cpu.xor_ixdi();

    Assertor::new(cpu)
        .register_a_is(0b1011_1000)
        .sign_flag_is_negative()
        .zero_flag_is_reset()
        .half_carry_flag_is_reset()
        .carry_flag_is_reset()
        .parity_overflow_flag_is_reset()
        .add_subtract_flag_is_reset();
}

#[test]
fn xor_iydi() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![0xfd, 0xae, 2, 0b0111_1011])
        .with_a(0b1100_0011)
        .with_iy(1)
        .build();

    cpu.xor_iydi();

    Assertor::new(cpu)
        .register_a_is(0b1011_1000)
        .sign_flag_is_negative()
        .zero_flag_is_reset()
        .half_carry_flag_is_reset()
        .carry_flag_is_reset()
        .parity_overflow_flag_is_reset()
        .add_subtract_flag_is_reset();
}
