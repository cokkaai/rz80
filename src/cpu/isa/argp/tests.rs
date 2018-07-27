// === General-Purpose Arithmetic ===

use cpu::CpuBuilder;
use cpu::Assertor;

#[test]
fn daa() {
    unimplemented!();
}

#[test]
fn cpl() {
    let mut cpu = CpuBuilder::new()
        .with_memory_size(10)
        .with_a(0b1011_0100)
        .build();

    cpu.cpl();

    Assertor::new(cpu)
        .register_a_is(0b0100_1011)
        .half_carry_flag_is_set()
        .parity_overflow_flag_is_reset()
        .program_counter_is(1);
}

#[test]
fn neg() {
    let mut cpu = CpuBuilder::new()
        .with_memory_size(10)
        .with_a(0b1001_1000)
        .build();

    cpu.neg();

    Assertor::new(cpu)
        .register_a_is(0b0110_1000)
        .sign_flag_is_positive()
        .zero_flag_is_reset()
        .half_carry_flag_is_reset()
        .parity_overflow_flag_is_reset()
        .add_subtract_flag_is_set()
        .carry_flag_is_set()
        .program_counter_is(2);
}

#[test]
fn ccf() {
    let mut cpu = CpuBuilder::new()
        .with_memory_size(16)
        .with_flag_c(true)
        .build();

    cpu.ccf();

    Assertor::new(cpu)
        .sign_flag_is_negative()
        .zero_flag_is_reset()
        .parity_overflow_flag_is_reset()
        .half_carry_flag_is_set()
        .add_subtract_flag_is_reset()
        .carry_flag_is_reset()
        .program_counter_is(1);
}

#[test]
fn scf() {
    let mut cpu = CpuBuilder::new().with_memory_size(10).build();

    cpu.scf();

    Assertor::new(cpu)
        .carry_flag_is_set()
        .half_carry_flag_is_reset()
        .add_subtract_flag_is_reset()
        .program_counter_is(1);
}
