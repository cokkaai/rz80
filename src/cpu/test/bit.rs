// === Bit Set, Reset, and Test Group ===

use cpu::CpuBuilder;
use cpu::Assertor;

#[test]
fn bit_b_r() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![
            0xcb, // BIT 7, E
            0b01_111_011,
        ])
        .with_e(0b1000_0000)
        .build();

    cpu.bit_b_r();

    Assertor::new(cpu)
        .zero_flag_is_reset()
        .half_carry_flag_is_set()
        .add_subtract_flag_is_reset()
        .program_counter_is(2);
}

#[test]
fn bit_b_hli() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![
            0xcb, // BIT 5, (HL)
            0b01_101_110,
        ])
        .build();

    cpu.bit_b_hli();

    Assertor::new(cpu)
        .zero_flag_is_set()
        .half_carry_flag_is_set()
        .add_subtract_flag_is_reset()
        .program_counter_is(2);
}

#[test]
fn bit_b_ixdi() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![
            0xdd, // BIT 6, (IX + 4)
            0xcb,
            0x04,
            0b01_110_110,
            0x45,
            0x46,
            0b1110_1111,
            0x01,
        ])
        .with_ix(3)
        .build();

    cpu.bit_b_ixdi();

    Assertor::new(cpu)
        .zero_flag_is_set()
        .half_carry_flag_is_set()
        .add_subtract_flag_is_reset()
        .program_counter_is(4);
}

#[test]
fn bit_b_iydi() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![
            0xfd, // BIT 6, (IY + 4)
            0xcb,
            0x04,
            0b01_110_110,
            0x45,
            0x46,
            0b1110_1111,
            0x01,
        ])
        .with_iy(3)
        .build();

    cpu.bit_b_iydi();

    Assertor::new(cpu)
        .zero_flag_is_set()
        .half_carry_flag_is_set()
        .add_subtract_flag_is_reset()
        .program_counter_is(4);
}

#[test]
fn set_b_r() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![
            0xcb, // SET 2, E
            0b11_010_011,
        ])
        .with_e(0b1000_0000)
        .build();

    cpu.set_b_r();

    Assertor::new(cpu)
        .register_e_is(0b1000_0100)
        .program_counter_is(2);
}

#[test]
fn set_b_hli() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![
            0xcb, // SET 5, (HL)
            0b11_101_110,
            0,
            0,
        ])
        .with_hl(3)
        .build();

    cpu.set_b_hli();

    Assertor::new(cpu)
        .memory_at_address_is(3, 0b0010_0000)
        .program_counter_is(2);
}

#[test]
fn set_b_ixdi() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![
            0xdd, // SET 6, (IX + 4)
            0xcb,
            0x04,
            0b11_110_110,
            0x45,
            0x46,
            0b1110_1111,
            0x01,
        ])
        .with_ix(3)
        .build();

    cpu.set_b_ixdi();

    Assertor::new(cpu)
        .memory_at_address_is(7, 0b0100_0001)
        .program_counter_is(4);
}

#[test]
fn set_b_iydi() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![
            0xfd, // SET 6, (IY + 4)
            0xcb,
            0x04,
            0b11_110_110,
            0x45,
            0x46,
            0b1110_1111,
            0x01,
        ])
        .with_iy(3)
        .build();

    cpu.set_b_iydi();

    Assertor::new(cpu)
        .memory_at_address_is(7, 0b0100_0001)
        .program_counter_is(4);
}

#[test]
pub fn res_b_r() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![
            0xcb, // RES 2, E
            0b10_010_011,
        ])
        .with_e(0b1000_0100)
        .build();

    cpu.res_b_r();

    Assertor::new(cpu)
        .register_e_is(0b1000_0000)
        .program_counter_is(2);
}

#[test]
pub fn res_b_hli() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![
            0xcb, // RES 5, (HL)
            0b10_101_110,
            0,
            0b0010_0000,
        ])
        .with_hl(3)
        .build();

    cpu.res_b_hli();

    Assertor::new(cpu)
        .memory_at_address_is(3, 0)
        .program_counter_is(2);
}

#[test]
pub fn res_b_ixdi() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![
            0xdd, // RES 6, (IX + 4)
            0xcb,
            0x04,
            0b10_110_110,
            0x45,
            0x46,
            0b1110_1111,
            0xff,
        ])
        .with_ix(3)
        .build();

    cpu.res_b_ixdi();

    Assertor::new(cpu)
        .memory_at_address_is(7, 0b1011_1111)
        .program_counter_is(4);
}

#[test]
pub fn res_b_iydi() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![
            0xdd, // RES 6, (IX + 4)
            0xcb,
            0x04,
            0b10_110_110,
            0x45,
            0x46,
            0b1110_1111,
            0xff,
        ])
        .with_iy(3)
        .build();

    cpu.res_b_iydi();

    Assertor::new(cpu)
        .memory_at_address_is(7, 0b1011_1111)
        .program_counter_is(4);
}
