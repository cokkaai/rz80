// === Exchange, Block Transfer, and Search Group ===

use cpu::bytes;
use cpu::CpuBuilder;
use cpu::Register16;

#[test]
fn ex_de_hl() {
    let mut cpu = CpuBuilder::new()
        .with_d(1)
        .with_e(2)
        .with_h(3)
        .with_l(4)
        .with_memory_size(16)
        .build();

    cpu.ex_de_hl();

    assert_eq!(cpu.d, 3);
    assert_eq!(cpu.e, 4);
    assert_eq!(cpu.h, 1);
    assert_eq!(cpu.l, 2);
    assert_eq!(cpu.pc, 1);
}

#[test]
fn ex_af_af1() {
    let mut cpu = CpuBuilder::new()
        .with_a(1)
        .with_flag_n(true)
        .with_memory_size(16)
        .build();


    cpu.ex_af_af1();

    cpu.a = 3;
    cpu.f = 4;

    cpu.ex_af_af1();

    assert_eq!(cpu.a, 1);
    assert_eq!(cpu.f, 2);
    assert_eq!(cpu.a1, 3);
    assert_eq!(cpu.f1, 4);
    assert_eq!(cpu.pc, 2);
}

#[test]
fn exx() {
    let mut cpu = CpuBuilder::new()
        .with_b(1)
        .with_c(2)
        .with_d(3)
        .with_e(4)
        .with_h(5)
        .with_l(6)
        .with_memory_size(16)
        .build();

    cpu.exx();

    cpu.b = 11;
    cpu.c = 12;
    cpu.d = 13;
    cpu.e = 14;
    cpu.h = 15;
    cpu.l = 16;

    cpu.exx();

    assert_eq!(cpu.b, 1);
    assert_eq!(cpu.c, 2);
    assert_eq!(cpu.d, 3);
    assert_eq!(cpu.e, 4);
    assert_eq!(cpu.h, 5);
    assert_eq!(cpu.l, 6);

    assert_eq!(cpu.b1, 11);
    assert_eq!(cpu.c1, 12);
    assert_eq!(cpu.d1, 13);
    assert_eq!(cpu.e1, 14);
    assert_eq!(cpu.h1, 15);
    assert_eq!(cpu.l1, 16);

    assert_eq!(cpu.pc, 2);
}

#[test]
fn ex_spi_hl() {
    let mut cpu = CpuBuilder::new()
        .with_h(9)
        .with_l(8)
        .with_memory(vec![1, 2, 3, 4])
        .build();

    cpu.ex_spi_hl();

    // H ↔ (SP+1), L ↔ (SP)
    assert_eq!(cpu.h, 2);
    assert_eq!(cpu.memory[1], 9);

    // H ↔ (SP+1), L ↔ (SP)
    assert_eq!(cpu.l, 1);
    assert_eq!(cpu.memory[0], 8);

    assert_eq!(cpu.pc, 1);
}

#[test]
fn ex_spi_ix() {
    let mut cpu = CpuBuilder::new()
        .with_ix(0x3988)
        .with_l(8)
        .with_memory(vec![0x90, 0x48, 3, 4])
        .build();

    cpu.ex_spi_ix();

    // IXH ↔ (SP+1)
    assert_eq!(bytes::high(cpu.ix), 0x48);
    assert_eq!(cpu.memory[1], 0x39);

    // IXL ↔ (SP)
    assert_eq!(bytes::low(cpu.ix), 0x90);
    assert_eq!(cpu.memory[0], 0x88);

    assert_eq!(cpu.pc, 2);
}

#[test]
fn ex_spi_iy() {
    let mut cpu = CpuBuilder::new()
        .with_iy(0x3988)
        .with_l(8)
        .with_memory(vec![0x90, 0x48, 3, 4])
        .build();

    cpu.ex_spi_iy();

    // IYH ↔ (SP+1)
    assert_eq!(bytes::high(cpu.iy), 0x48);
    assert_eq!(cpu.memory[1], 0x39);

    // IYL ↔ (SP)
    assert_eq!(bytes::low(cpu.iy), 0x90);
    assert_eq!(cpu.memory[0], 0x88);

    assert_eq!(cpu.pc, 2);
}

#[test]
fn ldi() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![0x90, 0x48, 3, 4])
        .with_bc(3)
        .with_de(0)
        .with_hl(1)
        .build();

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
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![0x90, 0x91, 0x92, 0x93, 0x44, 0x45, 0x46, 0x47, 3, 4])
        .with_bc(3)
        .with_de(0)
        .with_hl(4)
        .build();

    cpu.ldir();

    // (DE) ← (HL)
    assert_eq!(cpu.memory[0], 0x44);
    assert_eq!(cpu.memory[1], 0x45);
    assert_eq!(cpu.memory[2], 0x46);
    assert_eq!(cpu.memory[3], 0x93);

    // DE ← DE + 1
    assert_eq!(cpu.read_de(), 3);

    // HL ← HL + 1
    assert_eq!(cpu.read_hl(), 7);

    // BC ← BC – 1
    assert_eq!(cpu.read_bc(), 0);

    // P/V is set if BC – 1 ≠ 0; otherwise, it is reset.
    assert_eq!(cpu.get_pv(), false);
}

#[test]
fn ldd() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![0x90, 0x48, 3, 4])
        .with_bc(3)
        .with_de(3)
        .with_hl(1)
        .build();

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
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![0x90, 0x91, 0x92, 0x93, 0x44, 0x45, 0x46, 0x47, 3, 4])
        .with_bc(3)
        .with_de(3)
        .with_hl(7)
        .build();

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
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![0x90, 0x91, 0x92, 0x93, 0x3b, 0x45, 0x46, 0x47, 3, 4])
        .with_a(0x3b)
        .with_bc(1)
        .with_hl(4)
        .build();

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
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![0x90, 0x91, 0x92, 0x93, 0x3b, 0x45, 0x46, 0x47, 3, 4])
        .with_a(0x3b)
        .with_bc(2)
        .with_hl(4)
        .build();

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
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![0x90, 0x91, 0x92, 0x93, 0x3b, 0x45, 0x46, 0x47, 3, 4])
        .with_a(0x3b)
        .with_bc(1)
        .with_hl(4)
        .build();

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
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![0x90, 0x91, 0x92, 0x93, 0x3b, 0x45, 0x46, 0x47, 3, 4])
        .with_a(0x3b)
        .with_bc(2)
        .with_hl(4)
        .build();

    cpu.cpdr();

    // BC ← BC – 1
    assert_eq!(cpu.read_bc(), 0);

    // HL ← HL - 1
    assert_eq!(cpu.read_hl(), 2);

    // PC += 2
    assert_eq!(cpu.pc, 2);
}
