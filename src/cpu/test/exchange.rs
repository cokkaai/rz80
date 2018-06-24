// === Exchange, Block Transfer, and Search Group ===

use cpu::Register16;
use cpu::CPU;
use cpu::bytes;

#[test]
fn ex_de_hl() {
    let mut cpu = CPU::new(16);
    cpu.d = 1;
    cpu.e = 2;
    cpu.h = 3;
    cpu.l = 4;

    cpu.ex_de_hl();

    assert_eq!(cpu.d, 3);
    assert_eq!(cpu.e, 4);
    assert_eq!(cpu.h, 1);
    assert_eq!(cpu.l, 2);
    assert_eq!(cpu.pc, 1);
}

#[test]
fn ex_af_af1() {
    let mut cpu = CPU::new(16);
    cpu.a = 1;
    cpu.f = 2;
    cpu.a1 = 3;
    cpu.f1 = 4;

    cpu.ex_af_af1();

    assert_eq!(cpu.a, 3);
    assert_eq!(cpu.f, 4);
    assert_eq!(cpu.a1, 1);
    assert_eq!(cpu.f1, 2);
    assert_eq!(cpu.pc, 1);
}

#[test]
fn exx() {
    let mut cpu = CPU::new(16);

    cpu.b = 1;
    cpu.c = 2;
    cpu.d = 3;
    cpu.e = 4;
    cpu.h = 5;
    cpu.l = 6;

    cpu.b1 = 11;
    cpu.c1 = 12;
    cpu.d1 = 13;
    cpu.e1 = 14;
    cpu.h1 = 15;
    cpu.l1 = 16;

    cpu.exx();

    assert_eq!(cpu.b, 11);
    assert_eq!(cpu.c, 12);
    assert_eq!(cpu.d, 13);
    assert_eq!(cpu.e, 14);
    assert_eq!(cpu.h, 15);
    assert_eq!(cpu.l, 16);

    assert_eq!(cpu.b1, 1);
    assert_eq!(cpu.c1, 2);
    assert_eq!(cpu.d1, 3);
    assert_eq!(cpu.e1, 4);
    assert_eq!(cpu.h1, 5);
    assert_eq!(cpu.l1, 6);

    assert_eq!(cpu.pc, 1);
}

#[test]
fn ex_spi_hl() {
    // H ↔ (SP+1), L ↔ (SP)
    let mut cpu = CPU::with_memory(vec!(1, 2, 3, 4));
    cpu.h = 9;
    cpu.l = 8;

    cpu.ex_spi_hl();

    assert_eq!(cpu.h, 2);
    assert_eq!(cpu.l, 1);

    assert_eq!(cpu.memory[0], 8);
    assert_eq!(cpu.memory[1], 9);

    assert_eq!(cpu.pc, 1);
}

#[test]
fn ex_spi_ix() {
    // IXH ↔ (SP+1), IXL ↔ (SP)
    let mut cpu = CPU::with_memory(vec!(0x90, 0x48, 3, 4));
    cpu.ix = 0x3988;

    cpu.ex_spi_ix();

    assert_eq!(bytes::high(cpu.ix), 0x48);
    assert_eq!(bytes::low(cpu.ix), 0x90);

    assert_eq!(cpu.memory[0], 0x88);
    assert_eq!(cpu.memory[1], 0x39);

    assert_eq!(cpu.pc, 2);
}

#[test]
fn ex_spi_iy() {
    // IYH ↔ (SP+1), IYL ↔ (SP)
    let mut cpu = CPU::with_memory(vec!(0x90, 0x48, 3, 4));
    cpu.iy = 0x3988;

    cpu.ex_spi_iy();

    assert_eq!(bytes::high(cpu.iy), 0x48);
    assert_eq!(bytes::low(cpu.iy), 0x90);

    assert_eq!(cpu.memory[0], 0x88);
    assert_eq!(cpu.memory[1], 0x39);

    assert_eq!(cpu.pc, 2);
}

#[test]
fn ldi() {
    let mut cpu = CPU::with_memory(vec!(0x90, 0x48, 3, 4));
    cpu.write16(Register16::bc, 3);
    cpu.write16(Register16::de, 0);
    cpu.write16(Register16::hl, 1);

    cpu.ldi();

    // (DE) ← (HL)
    assert_eq!(cpu.memory[0], 0x48);

    // DE ← DE + 1
    assert_eq!(cpu.read16(Register16::de), 1);

    // HL ← HL + 1
    assert_eq!(cpu.read16(Register16::hl), 2);

    // BC ← BC – 1
    assert_eq!(cpu.read16(Register16::bc), 2);

    // P/V is set if BC – 1 ≠ 0; otherwise, it is reset.
    assert_eq!(cpu.get_pv(), true);
}

#[test]
fn ldir() {
    let mut cpu = CPU::with_memory(
        vec!(0x90, 0x91, 0x92, 0x93, 
             0x44, 0x45, 0x46, 0x47,
             3, 4),
    );

    cpu.write16(Register16::bc, 3);
    cpu.write16(Register16::de, 0);
    cpu.write16(Register16::hl, 4);

    cpu.ldir();

    // (DE) ← (HL)
    assert_eq!(cpu.memory[0], 0x44);
    assert_eq!(cpu.memory[1], 0x45);
    assert_eq!(cpu.memory[2], 0x46);
    assert_eq!(cpu.memory[3], 0x93);

    // DE ← DE + 1
    assert_eq!(cpu.read16(Register16::de), 3);

    // HL ← HL + 1
    assert_eq!(cpu.read16(Register16::hl), 7);

    // BC ← BC – 1
    assert_eq!(cpu.read16(Register16::bc), 0);

    // P/V is set if BC – 1 ≠ 0; otherwise, it is reset.
    assert_eq!(cpu.get_pv(), false);
}

#[test]
fn ldd() {
    let mut cpu = CPU::with_memory(vec!(0x90, 0x48, 3, 4));
    cpu.write16(Register16::bc, 3);
    cpu.write16(Register16::de, 3);
    cpu.write16(Register16::hl, 1);

    cpu.ldd();

    // (DE) ← (HL)
    assert_eq!(cpu.memory[0], 0x90);
    assert_eq!(cpu.memory[1], 0x48);
    assert_eq!(cpu.memory[2], 3);
    assert_eq!(cpu.memory[3], 0x48);

    // DE ← DE + 1
    assert_eq!(cpu.read16(Register16::de), 2);

    // HL ← HL + 1
    assert_eq!(cpu.read16(Register16::hl), 0);

    // BC ← BC – 1
    assert_eq!(cpu.read16(Register16::bc), 2);

    // P/V is set if BC – 1 ≠ 0; otherwise, it is reset.
    assert_eq!(cpu.get_pv(), true);
}

#[test]
fn lddr() {
    let mut cpu = CPU::with_memory(
        vec!(0x90, 0x91, 0x92, 0x93, 
             0x44, 0x45, 0x46, 0x47,
             3, 4),
    );

    cpu.write16(Register16::bc, 3);
    cpu.write16(Register16::de, 3);
    cpu.write16(Register16::hl, 7);

    cpu.lddr();

    // (DE) ← (HL)
    assert_eq!(cpu.memory[0], 0x90);
    assert_eq!(cpu.memory[1], 0x45);
    assert_eq!(cpu.memory[2], 0x46);
    assert_eq!(cpu.memory[3], 0x47);

    // DE ← DE - 1
    assert_eq!(cpu.read16(Register16::de), 0);

    // HL ← HL - 1
    assert_eq!(cpu.read16(Register16::hl), 4);

    // BC ← BC – 1
    assert_eq!(cpu.read16(Register16::bc), 0);

    // P/V is set if BC – 1 ≠ 0; otherwise, it is reset.
    assert_eq!(cpu.get_pv(), false);
}

#[test]
fn cpi() {
    let mut cpu = CPU::with_memory(
        vec!(0x90, 0x91, 0x92, 0x93, 
             0x3b, 0x45, 0x46, 0x47,
             3, 4),
    );

    cpu.a = 0x3b;
    cpu.write_bc(1);
    cpu.write_hl(4);

    cpu.cpi();

    // BC ← BC – 1
    assert_eq!(cpu.read_bc(), 0);

    // HL ← HL +1
    assert_eq!(cpu.read_hl(), 5);
    
    // Z is set if A is (HL); otherwise, it is reset.
    assert_eq!(cpu.get_z(), true);

    // P/V is set if BC – 1 is not 0; otherwise, it is reset.
    assert_eq!(cpu.get_pv(), false);

    // S is set if result is negative; otherwise, it is reset.
    assert_eq!(cpu.get_s(), false);

    // C is not affected.
    assert_eq!(cpu.get_c(), false);

    // H is set if borrow from bit 4; otherwise, it is reset.
    assert_eq!(cpu.get_h(), false);

    // N is set.
    assert_eq!(cpu.get_n(), false);

    // PC += 2
    assert_eq!(cpu.pc, 2);
}

#[test]
fn cpir() {
    let mut cpu = CPU::with_memory(
        vec!(0x90, 0x91, 0x92, 0x93, 
             0x3b, 0x45, 0x46, 0x47,
             3, 4),
    );

    cpu.a = 0x3b;
    cpu.write16(Register16::bc, 2);
    cpu.write16(Register16::hl, 4);

    cpu.cpir();

    // BC ← BC – 1
    assert_eq!(cpu.read_bc(), 0);

    // HL ← HL +1
    assert_eq!(cpu.read16(Register16::hl), 6);
    
    // Z is set if A is (HL); otherwise, it is reset.
    assert_eq!(cpu.get_z(), false);

    // P/V is set if BC – 1 is not 0; otherwise, it is reset.
    assert_eq!(cpu.get_pv(), true);

    // S is set if result is negative; otherwise, it is reset.
    assert_eq!(cpu.get_s(), false);

    // C is not affected.
    assert_eq!(cpu.get_c(), false);

    // H is set if borrow from bit 4; otherwise, it is reset.
    assert_eq!(cpu.get_h(), false);

    // N is set.
    assert_eq!(cpu.get_n(), false);
}

#[test]
fn cpd() {
    let mut cpu = CPU::with_memory(
        vec!(0x90, 0x91, 0x92, 0x93, 
             0x3b, 0x45, 0x46, 0x47,
             3, 4),
    );

    cpu.a = 0x3b;
    cpu.write16(Register16::bc, 1);
    cpu.write16(Register16::hl, 4);

    cpu.cpd();

    // BC ← BC – 1
    assert_eq!(cpu.read16(Register16::bc), 0);

    // HL ← HL - 1
    assert_eq!(cpu.read16(Register16::hl), 3);
    
    // PC += 2
    assert_eq!(cpu.pc, 2);
}

#[test]
fn cpdr() {
    let mut cpu = CPU::with_memory(
        vec!(0x90, 0x91, 0x92, 0x93, 
             0x3b, 0x45, 0x46, 0x47,
             3, 4),
    );

    cpu.a = 0x3b;
    cpu.write16(Register16::bc, 2);
    cpu.write16(Register16::hl, 4);

    cpu.cpdr();

    // BC ← BC – 1
    assert_eq!(cpu.read_bc(), 0);

    // HL ← HL - 1
    assert_eq!(cpu.read_hl(), 2);
    
    // PC += 2
    assert_eq!(cpu.pc, 2);
}
