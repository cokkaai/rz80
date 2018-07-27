use cpu::CpuBuilder;
use cpu::Assertor;

// === 8-Bit Arithmetic Group ===

#[test]
fn inc_r() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![0b00_010_100, 0, 0, 0])
        .with_d(0x28)
        .build();

    cpu.inc_r();

    Assertor::new(cpu)
        // If the D Register contains 28h , then upon the execution of an INC D
        // instruction, the D Register contains 29h.
        .register_d_is(0x29)
        .sign_flag_is_positive()
        .zero_flag_is_reset()
        .half_carry_flag_is_reset()
        .parity_overflow_flag_is_reset()
        .add_subtract_flag_is_reset()
        .carry_flag_is_reset()
        .program_counter_is(1);
}

#[test]
fn inc_hli() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![0x34, 0x00, 0x00, 0x00])
        .with_hl(0x02)
        .build();

    cpu.inc_hli();

    Assertor::new(cpu)
        // (HL) ← (HL) + 1
        .memory_at_address_is(2, 1)
        .sign_flag_is_positive()
        .zero_flag_is_reset()
        .half_carry_flag_is_reset()
        .parity_overflow_flag_is_reset()
        .add_subtract_flag_is_reset()
        .carry_flag_is_reset()
        .program_counter_is(1);
}

#[test]
fn inc_ixdi() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![
            0xfd, 0x34, 0x8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x06,
        ])
        .with_ix(0x01)
        .build();

    cpu.inc_ixdi();

    Assertor::new(cpu)
        // (IX+d) ← (IX+d) + 1
        .memory_at_address_is(9, 7)
        .sign_flag_is_positive()
        .zero_flag_is_reset()
        .half_carry_flag_is_reset()
        .parity_overflow_flag_is_reset()
        .add_subtract_flag_is_reset()
        .carry_flag_is_reset()
        .program_counter_is(3);
}

#[test]
fn inc_iydi() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![
            0xfd, 0x34, 0x8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x06,
        ])
        .with_iy(0x01)
        .build();

    cpu.inc_iydi();

    Assertor::new(cpu)
        // (IY+d) ← (IY+d) + 1
        .memory_at_address_is(9, 7)
        .sign_flag_is_positive()
        .zero_flag_is_reset()
        .half_carry_flag_is_reset()
        .parity_overflow_flag_is_reset()
        .add_subtract_flag_is_reset()
        .carry_flag_is_reset()
        .program_counter_is(3);
}

#[test]
fn dec_r() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![0b00_010_101, 0, 0, 0])
        .with_d(0x2a)
        .build();

    cpu.dec_r();

    Assertor::new(cpu)
        // If the D Register contains byte 2Ah , then upon the execution
        // of a DEC D instruction, the D Register contains 29h .
        .register_d_is(0x29)
        .sign_flag_is_positive()
        .zero_flag_is_reset()
        .half_carry_flag_is_reset()
        .parity_overflow_flag_is_reset()
        .add_subtract_flag_is_set()
        .carry_flag_is_reset()
        .program_counter_is(1);
}

#[test]
fn dec_hli() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![0x34, 0x00, 0x0e, 0x00])
        .with_hl(0x02)
        .build();

    cpu.dec_hli();

    Assertor::new(cpu)
        // (HL) ← (HL) - 1
        .memory_at_address_is(2, 0x0d)
        .sign_flag_is_positive()
        .zero_flag_is_reset()
        .half_carry_flag_is_reset()
        .parity_overflow_flag_is_reset()
        .add_subtract_flag_is_set()
        .carry_flag_is_reset()
        .program_counter_is(1);
}

#[test]
fn dec_ixdi() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![
            0xdd, 0x35, 0x8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x06,
        ])
        .with_ix(0x01)
        .build();

    cpu.dec_ixdi();

    Assertor::new(cpu)
        // (IX+d) ← (IX+d) - 1
        .memory_at_address_is(9, 5)
        .sign_flag_is_positive()
        .zero_flag_is_reset()
        .half_carry_flag_is_reset()
        .parity_overflow_flag_is_reset()
        .add_subtract_flag_is_set()
        .carry_flag_is_reset()
        .program_counter_is(3);
}

#[test]
fn dec_iydi() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![
            0xfd, 0x35, 0x8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x06,
        ])
        .with_iy(0x01)
        .build();

    cpu.dec_iydi();

    Assertor::new(cpu)
        // (IX+d) ← (IX+d) - 1
        .memory_at_address_is(9, 5)
        .sign_flag_is_positive()
        .zero_flag_is_reset()
        .half_carry_flag_is_reset()
        .parity_overflow_flag_is_reset()
        .add_subtract_flag_is_set()
        .carry_flag_is_reset()
        .program_counter_is(3);
}
