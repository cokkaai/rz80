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
    unimplemented!();
}

// INC (HL)
#[test]
fn inc_hli() {
    unimplemented!();
}

// INC (IX+d)
#[test]
fn inc_ixd() {
    unimplemented!();
}

// INC (IY+d)
#[test]
fn inc_iyd() {
    unimplemented!();
}

// DEC m
#[test]
fn dec_m() {
    unimplemented!();
}
