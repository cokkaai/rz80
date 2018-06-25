use cpu::CPU;

// === 8-Bit Arithmetic Group ===

// ADD A, r
#[test]
fn add_a_r() {
    let mut cpu = CPU::with_memory(
        vec![
            0b10000_001, // ADD A, C
            0x66, 
            0x66, 
            0x66,
        ],
    );
    cpu.a = 7;
    cpu.c = 4;

    cpu.add_a_r();

    assert_eq!(cpu.a, 11);
    assert_eq!(cpu.pc, 1);
    // S is set if result is negative; otherwise, it is reset.
    assert_eq!(cpu.get_z(), false);
    
}

// ADD A, n
#[test]
fn add_d_n() {
    unimplemented!();
}

// ADD A, (HL)
#[test]
fn add_a_hli() {
    unimplemented!();
}

// ADD A, (IX + d)
#[test]
fn add_a_ixd() {
    unimplemented!();
}

// ADD A, (IY + d)
#[test]
fn add_a_iyd() {
    unimplemented!();
}

// ADC A, s
#[test]
fn adc_a_s() {
    unimplemented!();
}

// SUB s
#[test]
fn sub_s() {
    unimplemented!();
}

// SBC A, s
#[test]
fn sbc_a_s() {
    unimplemented!();
}

// AND s
#[test]
fn and_s() {
    unimplemented!();
}

// OR s
#[test]
fn or_s() {
    unimplemented!();
}

// XOR s
#[test]
fn xor_s() {
    unimplemented!();
}

// CP s
#[test]
fn cp_s() {
    unimplemented!();
}

// INC r
#[test]
fn inc_r() {
    let mut cpu = CPU::with_memory(vec![0b00_010_100]);
    cpu.d = 0x28;

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
    let mut cpu = CPU::with_memory(vec![0x34, 0x00, 0x00, 0x00]);
    cpu.write_hl(0x02);
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
    let mut cpu = CPU::with_memory(vec![0xfd, 0x34, 0x8, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x06]);
    cpu.ix = 0x01;
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
    let mut cpu = CPU::with_memory(vec![0xfd, 0x34, 0x8, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x06]);
    cpu.iy = 0x01;
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
    let mut cpu = CPU::with_memory(vec![0b00_010_101]);
    cpu.d = 0x2a;
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
    let mut cpu = CPU::with_memory(vec![0x34, 0x00, 0x0e, 0x00]);
    cpu.write_hl(0x02);
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
    let mut cpu = CPU::with_memory(vec![0xdd, 0x35, 0x8, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x06]);
    cpu.ix = 0x01;
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
    let mut cpu = CPU::with_memory(vec![0xfd, 0x35, 0x8, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x06]);
    cpu.iy = 0x01;
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
