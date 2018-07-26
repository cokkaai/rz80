// === General-Purpose Arithmetic and CPU Control Groups ===

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
        .sign_flag_is_negative()
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

    // CY ← !CY
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

#[test]
fn nop() {
    let mut cpu = CpuBuilder::new().with_memory_size(10).build();

    cpu.nop();

    Assertor::new(cpu).program_counter_is(1);
}

#[test]
fn halt() {
    unimplemented!();
}

#[test]
fn di() {
    let mut cpu = CpuBuilder::new().with_memory_size(10).build();

    cpu.di();

    Assertor::new(cpu)
        .interrupt_flip_flop_1_is_reset()
        .interrupt_flip_flop_2_is_reset()
        .program_counter_is(1);
}

#[test]
fn ei() {
    let mut cpu = CpuBuilder::new().with_memory_size(10).build();

    cpu.ei();

    Assertor::new(cpu)
        .interrupt_flip_flop_1_is_set()
        .interrupt_flip_flop_2_is_set()
        .program_counter_is(1);
}

// IM 0
#[test]
fn im_0() {
    unimplemented!();
}

// IM 1
#[test]
fn im_1() {
    unimplemented!();
}

// IM 2
#[test]
fn im_2() {
    unimplemented!();
}
