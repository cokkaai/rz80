// === Shift Group ===

use cpu::CpuBuilder;
use cpu::Assertor;

#[test]
fn sla_r() {
    let mut cpu = CpuBuilder::new()
        .with_l(0b1011_0001)
        .with_memory(vec![0xcb, 0b0010_0101, 0, 0, 0, 0, 0, 0])
        .build();

    cpu.sla_r();

    Assertor::new(cpu)
        .register_l_is(0b0110_0010)
        .carry_flag_is_set()
        .sign_is_positive()
        .zero_flag_is_reset()
        .parity_overflow_flag_is_set()
        .half_carry_flag_is_reset()
        .add_subtract_flag_is_reset()
        .program_counter_is(2);
}

#[test]
fn sla_hli() {
    let mut cpu = CpuBuilder::new()
        .with_hl(0x07)
        .with_memory(vec![0xcb, 0x26, 0, 0, 0, 0, 0, 0b1011_0001])
        .build();

    cpu.sla_hli();

    Assertor::new(cpu)
        .memory_at_address_is(0x07, 0b0110_0010)
        .carry_flag_is_set()
        .sign_is_positive()
        .zero_flag_is_reset()
        .parity_overflow_flag_is_set()
        .half_carry_flag_is_reset()
        .add_subtract_flag_is_reset()
        .program_counter_is(2);
}

#[test]
fn sla_ixdi() {
    let mut cpu = CpuBuilder::new()
        .with_ix(0x06)
        .with_memory(vec![0xdd, 0xcb, 0x01, 0x26, 0, 0, 0, 0b1011_0001])
        .build();

    cpu.sla_ixdi();

    Assertor::new(cpu)
        .memory_at_address_is(0x07, 0b0110_0010)
        .carry_flag_is_set()
        .sign_is_positive()
        .zero_flag_is_reset()
        .parity_overflow_flag_is_set()
        .half_carry_flag_is_reset()
        .add_subtract_flag_is_reset()
        .program_counter_is(4);
}

#[test]
fn sla_iydi() {
    let mut cpu = CpuBuilder::new()
        .with_iy(0x06)
        .with_memory(vec![0xfd, 0xcb, 0x01, 0x26, 0, 0, 0, 0b1011_0001])
        .build();

    cpu.sla_iydi();

    Assertor::new(cpu)
        .memory_at_address_is(0x07, 0b0110_0010)
        .carry_flag_is_set()
        .sign_is_positive()
        .zero_flag_is_reset()
        .parity_overflow_flag_is_set()
        .half_carry_flag_is_reset()
        .add_subtract_flag_is_reset()
        .program_counter_is(4);
}

#[test]
fn sra_r() {
    let mut cpu = CpuBuilder::new()
        .with_l(0b1011_1000)
        .with_memory(vec![0xcb, 0b0010_1101, 0, 0, 0, 0, 0, 0])
        .build();

    cpu.sra_r();

    Assertor::new(cpu)
        .register_l_is(0b1101_1100)
        .carry_flag_is_reset()
        .sign_is_negative()
        .zero_flag_is_reset()
        .parity_overflow_flag_is_set()
        .half_carry_flag_is_reset()
        .add_subtract_flag_is_reset()
        .program_counter_is(2);
}

#[test]
fn sra_hli() {
    let mut cpu = CpuBuilder::new()
        .with_hl(0x07)
        .with_memory(vec![0xcb, 0x2e, 0, 0, 0, 0, 0, 0b1011_1000])
        .build();

    cpu.sra_hli();

    Assertor::new(cpu)
        .memory_at_address_is(0x07, 0b1101_1100)
        .carry_flag_is_reset()
        .sign_is_negative()
        .zero_flag_is_reset()
        .parity_overflow_flag_is_set()
        .half_carry_flag_is_reset()
        .add_subtract_flag_is_reset()
        .program_counter_is(2);
}

#[test]
fn sra_ixdi() {
    let mut cpu = CpuBuilder::new()
        .with_ix(0x06)
        .with_memory(vec![0xdd, 0xcb, 0x01, 0x2e, 0, 0, 0, 0b1011_1000])
        .build();

    cpu.sra_ixdi();

    Assertor::new(cpu)
        .memory_at_address_is(0x07, 0b1101_1100)
        .carry_flag_is_reset()
        .sign_is_negative()
        .zero_flag_is_reset()
        .parity_overflow_flag_is_set()
        .half_carry_flag_is_reset()
        .add_subtract_flag_is_reset()
        .program_counter_is(4);
}

#[test]
fn sra_iydi() {
    let mut cpu = CpuBuilder::new()
        .with_iy(0x06)
        .with_memory(vec![0xfd, 0xcb, 0x01, 0x2e, 0, 0, 0, 0b1011_1000])
        .build();

    cpu.sra_iydi();

    Assertor::new(cpu)
        .memory_at_address_is(0x07, 0b1101_1100)
        .carry_flag_is_reset()
        .sign_is_negative()
        .zero_flag_is_reset()
        .parity_overflow_flag_is_set()
        .half_carry_flag_is_reset()
        .add_subtract_flag_is_reset()
        .program_counter_is(4);
}

#[test]
fn srl_r() {
    let mut cpu = CpuBuilder::new()
        .with_l(0b1000_1111)
        .with_memory(vec![0xcb, 0b0011_1101, 0, 0, 0, 0, 0, 0])
        .build();

    cpu.srl_r();

    Assertor::new(cpu)
        .register_l_is(0b0100_0111)
        .carry_flag_is_set()
        .sign_is_positive()
        .zero_flag_is_reset()
        .parity_overflow_flag_is_reset()
        .half_carry_flag_is_reset()
        .add_subtract_flag_is_reset()
        .program_counter_is(2);
}

#[test]
fn srl_hli() {
    let mut cpu = CpuBuilder::new()
        .with_hl(0x07)
        .with_memory(vec![0xcb, 0xcb, 0x01, 0x3e, 0, 0, 0, 0b1000_1111])
        .build();

    cpu.srl_hli();

    Assertor::new(cpu)
        .memory_at_address_is(0x07, 0b0100_0111)
        .carry_flag_is_set()
        .sign_is_positive()
        .zero_flag_is_reset()
        .parity_overflow_flag_is_reset()
        .half_carry_flag_is_reset()
        .add_subtract_flag_is_reset()
        .program_counter_is(2);
}

#[test]
fn srl_ixdi() {
    let mut cpu = CpuBuilder::new()
        .with_ix(0x06)
        .with_memory(vec![0xdd, 0xcb, 0x01, 0x3e, 0, 0, 0, 0b1000_1111])
        .build();

    cpu.srl_ixdi();

    Assertor::new(cpu)
        .memory_at_address_is(0x07, 0b0100_0111)
        .carry_flag_is_set()
        .sign_is_positive()
        .zero_flag_is_reset()
        .parity_overflow_flag_is_reset()
        .half_carry_flag_is_reset()
        .add_subtract_flag_is_reset()
        .program_counter_is(4);
}

#[test]
fn srl_iydi() {
    let mut cpu = CpuBuilder::new()
        .with_iy(0x06)
        .with_memory(vec![0xfd, 0xcb, 0x01, 0x3e, 0, 0, 0, 0b1000_1111])
        .build();

    cpu.srl_iydi();

    Assertor::new(cpu)
        .memory_at_address_is(0x07, 0b0100_0111)
        .carry_flag_is_set()
        .sign_is_positive()
        .zero_flag_is_reset()
        .parity_overflow_flag_is_reset()
        .half_carry_flag_is_reset()
        .add_subtract_flag_is_reset()
        .program_counter_is(4);
}
