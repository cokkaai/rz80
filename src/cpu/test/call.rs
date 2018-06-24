// === Call and Return Group ===

use cpu::CPU;

#[test]
fn call_nn() {
    let mut cpu = CPU::with_memory(vec![
        0xcd, 0x04, 0x00, 0x00,
        0xaa, 0xbb, 0xcc, 0xdd,
    ]);
    cpu.sp = 8;

    cpu.call_nn();

    assert_eq!(cpu.memory[7], 0x00);
    assert_eq!(cpu.memory[6], 0x03);
    assert_eq!(cpu.pc, 0x04);
}

#[test]
fn call_cc_nn() {
    let mut cpu = CPU::with_memory(vec![
        0b11_001_100, 0x04, 0x00, 0x00,
        0xaa, 0xbb, 0xcc, 0xdd,
    ]);
    cpu.sp = 8;

    cpu.call_nn();

    assert_eq!(cpu.memory[7], 0x00);
    assert_eq!(cpu.memory[6], 0x03);
    assert_eq!(cpu.pc, 0x04);
}

#[test]
fn ret() {
    let mut cpu = CPU::with_memory(vec![
        0xc9, 0x04, 0x00, 0x00,
        0xaa, 0xbb, 0x04, 0x00,
    ]);
    cpu.sp = 6;

    cpu.ret();

    assert_eq!(cpu.pc, 0x04);
    assert_eq!(cpu.sp, 0x08);
}

#[test]
fn ret_cc() {
    let mut cpu = CPU::with_memory(vec![
        0b11_000_000, 0x04, 0x00, 0x00,
        0xaa, 0xbb, 0x04, 0x00,
    ]);
    cpu.sp = 6;

    cpu.ret_cc();

    assert_eq!(cpu.pc, 0x04);
}

#[test]
fn reti() {
    unimplemented!();
}

#[test]
fn retn() {
    let mut cpu = CPU::with_memory(vec![
        0xc9, 0x04, 0x00, 0x00,
        0xaa, 0xbb, 0x04, 0x00,
    ]);
    cpu.sp = 6;
    cpu.iff1 = false;
    cpu.iff2 = true;

    cpu.retn();

    assert_eq!(cpu.pc, 0x04);
    assert_eq!(cpu.sp, 0x08);
    assert_eq!(cpu.iff1, true);
    assert_eq!(cpu.iff2, true);
}

#[test]
fn rst_p() {
    let mut cpu = CPU::with_memory(vec![0b11_001_111, 0, 0, 0]);
    cpu.rst_p();
    assert_eq!(cpu.pc, 0x08);
}
