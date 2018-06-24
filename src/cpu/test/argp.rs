// === General-Purpose Arithmetic and CPU Control Groups ===

use cpu::CPU;

#[test]
fn daa() {
    unimplemented!();
}

#[test]
fn cpl() {
    let mut cpu = CPU::new(10);
    cpu.a = 0b1011_0100;

    cpu.cpl();

    assert_eq!(cpu.a, 0b0100_1011);
    assert_eq!(cpu.get_h(), true);
    assert_eq!(cpu.get_n(), false);
}

#[test]
fn neg() {
    let mut cpu = CPU::new(10);
    cpu.a = 0b1001_1000;

    cpu.neg();

    assert_eq!(cpu.a, 0b0110_1000);

    // S is set if result is negative; otherwise, it is reset.
    assert_eq!(cpu.get_s(), false);

    // Z is set if result is 0; otherwise, it is reset.
    assert_eq!(cpu.get_z(), false);

    // H is set if borrow from bit 4; otherwise, it is reset.
    assert_eq!(cpu.get_h(), false);

    // P/V is set if Accumulator was 80h before operation; otherwise, it is reset.
    assert_eq!(cpu.get_pv(), false);

    // N is set.
    assert_eq!(cpu.get_n(), true);

    // C is set if Accumulator was not 00h before operation; otherwise, it is reset.
    assert_eq!(cpu.get_c(), true);
}

#[test]
fn ccf() {
    let mut cpu = CPU::new(10);
    cpu.set_c(true);

    // CY ← !CY
    cpu.ccf();

    // S is not affected.
    // Z is not affected.
    // P/V is not affected.
    
    // H, previous carry is copied.
    assert_eq!(cpu.get_h(), true);

    // N is reset.
    assert_eq!(cpu.get_n(), false);

    // C is set if CY was 0 before operation; otherwise, it is reset.
    assert_eq!(cpu.get_c(), false);
}

#[test]
fn scf() {
    let mut cpu = CPU::new(10);
   
    cpu.scf();

    assert_eq!(cpu.get_c(), true);
    assert_eq!(cpu.get_h(), false);
    assert_eq!(cpu.get_n(), false);
    assert_eq!(cpu.pc, 1);
}

#[test]
fn nop() {
    let mut cpu = CPU::new(10);
   
    cpu.nop();

    assert_eq!(cpu.pc, 1);
}

#[test]
fn halt() {
    unimplemented!();
}

#[test]
fn di() {
    let mut cpu = CPU::new(10);

    cpu.di();

    assert_eq!(cpu.iff1, false);
    assert_eq!(cpu.iff2, false);
    assert_eq!(cpu.pc, 1);
}

#[test]
fn ei() {
    let mut cpu = CPU::new(10);

    cpu.ei();

    assert_eq!(cpu.iff1, true);
    assert_eq!(cpu.iff2, true);
    assert_eq!(cpu.pc, 1);
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
