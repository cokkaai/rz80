use cpu::CpuBuilder;
use cpu::Assertor;

// === 8-Bit Arithmetic Group ===

#[test]
fn add_a_r() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![
            0b10000_001, // ADD A, C
            0x66,
            0x66,
            0x66,
        ])
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

#[test]
fn adc_a_r() {
    unimplemented!();
}

#[test]
fn adc_a_n() {
    unimplemented!();
}

#[test]
fn adc_a_hli() {
    unimplemented!();
}

#[test]
fn adc_a_ixdi() {
    unimplemented!();
}

#[test]
fn adc_a_iydi() {
    unimplemented!();
}

#[test]
fn sub_r() {
    unimplemented!();
}

#[test]
fn sub_n() {
    unimplemented!();
}

#[test]
fn sub_hli() {
    unimplemented!();
}

#[test]
fn sub_ixdi() {
    unimplemented!();
}

#[test]
fn sub_iydi() {
    unimplemented!();
}

#[test]
fn sbc_a_r() {
    unimplemented!();
}

#[test]
fn sbc_a_n() {
    unimplemented!();
}

#[test]
fn sbc_a_hli() {
    unimplemented!();
}

#[test]
fn sbc_a_ixdi() {
    unimplemented!();
}

#[test]
fn sbc_a_iydi() {
    unimplemented!();
}

#[test]
fn and_r() {
    unimplemented!();
}

#[test]
fn and_n() {
    unimplemented!();
}

#[test]
fn and_hli() {
    unimplemented!();
}

#[test]
fn and_ixdi() {
    unimplemented!();
}

#[test]
fn and_iydi() {
    unimplemented!();
}

#[test]
fn or_r() {
    unimplemented!();
}

#[test]
fn or_n() {
    unimplemented!();
}

#[test]
fn or_hli() {
    unimplemented!();
}

#[test]
fn or_ixdi() {
    unimplemented!();
}

#[test]
fn or_iydi() {
    unimplemented!();
}

#[test]
fn xor_r() {
    unimplemented!();
}

#[test]
fn xor_n() {
    unimplemented!();
}

#[test]
fn xor_hli() {
    unimplemented!();
}

#[test]
fn xor_ixdi() {
    unimplemented!();
}

#[test]
fn xor_iydi() {
    unimplemented!();
}

#[test]
fn cp_r() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec!(0b10111_000))
        .with_a(0x63)
        .with_b(0x60)
        .build();
    
    cpu.cp_r();

    // TODO: opcode description is not clear.
    // Investigate docs other than the manual.
    Assertor::new(cpu)
        .parity_overflow_flag_is_reset()
        .add_substract_flag_is_reset();
}

#[test]
fn cp_n() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec!(0xfe, 0x09))
        .with_a(0x63)
        .build();
    
    cpu.cp_n();

    // TODO: opcode description is not clear.
    // Investigate docs other than the manual.
    assert_eq!(cpu.get_pv(), false);
    assert_eq!(cpu.get_n(), false);
}

#[test]
fn cp_hli() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec!(0xbe, 0, 0, 0x60))
        .with_hl(0x03)
        .with_a(0x63)
        .build();
    
    cpu.cp_hli();

    // TODO: opcode description is not clear.
    // Investigate docs other than the manual.
    assert_eq!(cpu.get_pv(), false);
    assert_eq!(cpu.get_n(), false);
}

#[test]
fn cp_ixdi() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec!(0xdd, 0xbe, 0x01, 0x60))
        .with_ix(0x02)
        .with_a(0x63)
        .build();
    
    cpu.cp_ixdi();

    // TODO: opcode description is not clear.
    // Investigate docs other than the manual.
    assert_eq!(cpu.get_pv(), false);
    assert_eq!(cpu.get_n(), false);
}

#[test]
fn cp_iydi() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec!(0xfd, 0xbe, 0x01, 0x60))
        .with_iy(0x02)
        .with_a(0x63)
        .build();
    
    cpu.cp_iydi();

    // TODO: opcode description is not clear.
    // Investigate docs other than the manual.
    assert_eq!(cpu.get_pv(), false);
    assert_eq!(cpu.get_n(), false);
}

// INC r
#[test]
fn inc_r() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![0b00_010_100])
        .with_d(0x28)
        .build();

    cpu.inc_r();

    // If the D Register contains 28h , then upon the execution of an INC D
    // instruction, the D Register contains 29h.
    assert_eq!(cpu.d, 0x29);

    // S is set if result is negative; otherwise, it is reset.
    assert_eq!(cpu.get_s(), false);

    // Z is set if result is 0; otherwise, it is reset.
    assert_eq!(cpu.get_z(), false);

    // H is set if carry from bit 3; otherwise, it is reset.
    assert_eq!(cpu.get_h(), false);

    // P/V is set if r was 7Fh before operation; otherwise, it is reset.
    assert_eq!(cpu.get_pv(), false);

    // N is reset.
    assert_eq!(cpu.get_n(), false);

    // C is not affected.
    assert_eq!(cpu.get_c(), false);
}

// INC (HL)
#[test]
fn inc_hli() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![0x34, 0x00, 0x00, 0x00])
        .with_hl(0x02)
        .build();

    cpu.inc_hli();

    // (HL) ← (HL) + 1
    assert_eq!(cpu.memory[0x02], 0x01);

    // S is set if result is negative; otherwise, it is reset.
    assert_eq!(cpu.get_s(), false);

    // Z is set if result is 0; otherwise, it is reset.
    assert_eq!(cpu.get_z(), false);

    // H is set if carry from bit 3; otherwise, it is reset.
    assert_eq!(cpu.get_h(), false);

    // P/V is set if (HL) was 7Fh before operation; otherwise, it is reset.
    assert_eq!(cpu.get_pv(), false);

    // N is reset.
    assert_eq!(cpu.get_n(), false);

    // C is not affected.

    // PC += 1
    assert_eq!(cpu.pc, 1);
}

// INC (IX+d)
#[test]
fn inc_ixdi() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![
            0xfd, 0x34, 0x8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x06,
        ])
        .with_ix(0x01)
        .build();

    cpu.inc_ixdi();

    // (IX+d) ← (IX+d) + 1
    assert_eq!(cpu.memory[9], 0x07);

    // S is set if result is negative; otherwise, it is reset.
    assert_eq!(cpu.get_s(), false);

    // Z is set if result is 0; otherwise, it is reset.
    assert_eq!(cpu.get_z(), false);

    // H is set if carry from bit 3; otherwise, it is reset.
    assert_eq!(cpu.get_h(), false);

    // P/V is set if (IX+d) was 7Fh before operation; otherwise, it is reset.
    assert_eq!(cpu.get_pv(), false);

    // N is reset.
    assert_eq!(cpu.get_n(), false);

    // C is not affected.

    // PC += 3
    assert_eq!(cpu.pc, 3);
}

// INC (IY+d)
#[test]
fn inc_iydi() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![
            0xfd, 0x34, 0x8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x06,
        ])
        .with_iy(0x01)
        .build();

    cpu.inc_iydi();

    // (IY+d) ← (IY+d) + 1
    assert_eq!(cpu.memory[9], 0x07);

    // S is set if result is negative; otherwise, it is reset.
    assert_eq!(cpu.get_s(), false);

    // Z is set if result is 0; otherwise, it is reset.
    assert_eq!(cpu.get_z(), false);

    // H is set if carry from bit 3; otherwise, it is reset.
    assert_eq!(cpu.get_h(), false);

    // P/V is set if (IX+d) was 7Fh before operation; otherwise, it is reset.
    assert_eq!(cpu.get_pv(), false);

    // N is reset.
    assert_eq!(cpu.get_n(), false);

    // C is not affected.

    // PC += 3
    assert_eq!(cpu.pc, 3);
}

#[test]
fn dec_r() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![0b00_010_101])
        .with_d(0x2a)
        .build();

    cpu.dec_r();

    // If the D Register contains byte 2Ah , then upon the execution
    // of a DEC D instruction, the D Register contains 29h .
    assert_eq!(cpu.d, 0x29);

    // S is set if result is negative; otherwise, it is reset.
    assert_eq!(cpu.get_s(), false);

    // Z is set if result is 0; otherwise, it is reset.
    assert_eq!(cpu.get_z(), false);

    // H is set if borrow from bit 4, otherwise, it is reset.
    assert_eq!(cpu.get_h(), false);

    // P/V is set if r was 7Fh before operation; otherwise, it is reset.
    assert_eq!(cpu.get_pv(), false);

    // N is reset.
    assert_eq!(cpu.get_n(), true);

    // C is not affected.
    assert_eq!(cpu.get_c(), false);
}

#[test]
fn dec_hli() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![0x34, 0x00, 0x0e, 0x00])
        .with_hl(0x02)
        .build();

    cpu.dec_hli();

    // (HL) ← (HL) - 1
    assert_eq!(cpu.memory[0x02], 0x0d);

    // S is set if result is negative; otherwise, it is reset.
    assert_eq!(cpu.get_s(), false);

    // Z is set if result is 0; otherwise, it is reset.
    assert_eq!(cpu.get_z(), false);

    // H is set if borrow from bit 4, otherwise, it is reset.
    assert_eq!(cpu.get_h(), false);

    // P/V is set if (HL) was 7Fh before operation; otherwise, it is reset.
    assert_eq!(cpu.get_pv(), false);

    // N is reset.
    assert_eq!(cpu.get_n(), true);

    // C is not affected.

    // PC += 1
    assert_eq!(cpu.pc, 1);
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

    // (IX+d) ← (IX+d) - 1
    assert_eq!(cpu.memory[9], 0x05);

    // S is set if result is negative; otherwise, it is reset.
    assert_eq!(cpu.get_s(), false);

    // Z is set if result is 0; otherwise, it is reset.
    assert_eq!(cpu.get_z(), false);

    // H is set if borrow from bit 4, otherwise, it is reset.
    assert_eq!(cpu.get_h(), false);

    // P/V is set if (HL) was 7Fh before operation; otherwise, it is reset.
    assert_eq!(cpu.get_pv(), false);

    // N is reset.
    assert_eq!(cpu.get_n(), true);

    // C is not affected.

    // PC += 3
    assert_eq!(cpu.pc, 3);
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

    // (IY+d) ← (IY+d) - 1
    assert_eq!(cpu.memory[9], 0x05);

    // S is set if result is negative; otherwise, it is reset.
    assert_eq!(cpu.get_s(), false);

    // Z is set if result is 0; otherwise, it is reset.
    assert_eq!(cpu.get_z(), false);

    // H is set if borrow from bit 4, otherwise, it is reset.
    assert_eq!(cpu.get_h(), false);

    // P/V is set if (HL) was 7Fh before operation; otherwise, it is reset.
    assert_eq!(cpu.get_pv(), false);

    // N is reset.
    assert_eq!(cpu.get_n(), true);

    // C is not affected.

    // PC += 3
    assert_eq!(cpu.pc, 3);
}
