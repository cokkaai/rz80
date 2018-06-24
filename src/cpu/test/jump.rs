// === Jump Group ===

use cpu::CPU;

#[test]
fn jp_nn() {
    let mut cpu = CPU::with_memory(
        vec!(0xc3, 0x07, 0x00, 0x93, 
                     0x44, 0x45, 0x46, 0x47,
                     3, 4),
    );

    cpu.jp_nn();

    // PC ← nn
    assert_eq!(cpu.pc, 0x0007);
}

#[test]
fn jp_cc_nn() {
    // Check non-zero flag
    let mut cpu = CPU::with_memory(vec!(0b11_000_101, 0x07, 0));
    cpu.jp_cc_nn();
    assert_eq!(cpu.pc, 0x0007);
}

#[test]
fn jr_e() {
    let mut cpu = CPU::with_memory(vec!(0x18, 0x04, 0, 0, 0, 0, 0, 0));
    cpu.jr_e();
    assert_eq!(cpu.pc, 0x04);
}

#[test]
fn jr_c_e() {
    let mut cpu = CPU::with_memory(vec!(0x38, 0x04, 0, 0, 0, 0, 0, 0));
    cpu.set_c(true);
    cpu.jr_c_e();
    assert_eq!(cpu.pc, 0x04);
}

#[test]
fn jr_nc_e() {
    let mut cpu = CPU::with_memory(vec!(0x38, 0x04, 0, 0, 0, 0, 0, 0));
    cpu.set_c(false);
    cpu.jr_nc_e();
    assert_eq!(cpu.pc, 0x04);
}

#[test]
fn jr_z_e() {
    let mut cpu = CPU::with_memory(vec!(0x38, 0x04, 0, 0, 0, 0, 0, 0));
    cpu.set_z(true);
    cpu.jr_z_e();
    assert_eq!(cpu.pc, 0x04);
}

#[test]
fn jr_nz_e() {
    let mut cpu = CPU::with_memory(vec!(0x38, 0x04, 0, 0, 0, 0, 0, 0));
    cpu.set_z(false);
    cpu.jr_nz_e();
    assert_eq!(cpu.pc, 0x04);
}

#[test]
fn jp_hl() {
    let mut cpu = CPU::with_memory(
        vec!(0xc3, 0x07, 0x00, 0x93, 
                     0x44, 0x45, 0x46, 0x47,
                     3, 4),
    );

    cpu.h = 0x00;
    cpu.l = 0x09;

    cpu.jp_hl();

    // PC ← HL
    assert_eq!(cpu.pc, 0x0009);
}

#[test]
fn jp_ix() {
    let mut cpu = CPU::with_memory(
        vec!(0xc3, 0x07, 0x00, 0x93, 
                     0x44, 0x45, 0x46, 0x47,
                     3, 4),
    );

    cpu.ix = 0x0009;

    cpu.jp_ix();

    // PC ← IX
    assert_eq!(cpu.pc, 0x0009);
}

#[test]
fn jp_iy() {
    let mut cpu = CPU::with_memory(
        vec!(0xc3, 0x07, 0x00, 0x93, 
                     0x44, 0x45, 0x46, 0x47,
                     3, 4),
    );

    cpu.iy = 0x0009;

    cpu.jp_iy();

    // PC ← IY
    assert_eq!(cpu.pc, 0x0009);
}

#[test]
fn djnz_e() {
    let mut cpu = CPU::with_memory(
        vec!(0x10, 0x04, 0, 0,
             0x10, 0x04, 0, 0,
             0, 0, 3, 4),
    );
    cpu.b = 2;

    cpu.djnz_e(); 
    assert_eq!(cpu.pc, 4);

    cpu.djnz_e(); 
    assert_eq!(cpu.pc, 6);
}
