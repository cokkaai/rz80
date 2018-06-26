// === 8-Bit Load Group ===

use cpu::bytes;
use cpu::CpuBuilder;

#[test]
fn memory_at_pc() {
    let cpu = CpuBuilder::new()
        .with_memory(vec![
            0x01, 0x02, 0xa4, 0xa8, 0x0f, 0x11, 0x12, 0x14, 0x18, 0x1f, 0x22, 0x33, 0x44, 0x55,
            0x66, 0x77,
        ])
        .build();

    assert_eq!(cpu.memory_at_pc(0), 0x01);
    assert_eq!(cpu.memory_at_pc(1), 0x02);
    assert_eq!(cpu.memory_at_pc(9), 0x1f);
    assert_eq!(cpu.memory_at_pc(15), 0x77);
}

#[test]
fn twocmp() {
    assert_eq!(bytes::compl2(0), 0);
    assert_eq!(bytes::compl2(0b0000_0001), 0b1111_1111);
    assert_eq!(bytes::compl2(0b0000_0010), 0b1111_1110);
    assert_eq!(bytes::compl2(0b0111_1110), 0b1000_0010);
    assert_eq!(bytes::compl2(0b0111_1111), 0b1000_0001);
    assert_eq!(bytes::compl2(0b1111_1111), 0b0000_0001);
}

#[test]
fn incr_pc() {
    let mut cpu = CpuBuilder::new().with_memory_size(16).build();

    cpu.incr_pc(2);
    assert_eq!(cpu.pc, 2);

    cpu.incr_pc(12);
    assert_eq!(cpu.pc, 14);

    cpu.incr_pc(2);
    assert_eq!(cpu.pc, 0);

    cpu.incr_pc(16);
    assert_eq!(cpu.pc, 0);

    cpu.incr_pc(17);
    assert_eq!(cpu.pc, 1);
}

#[test]
fn ld_r_r1() {
    // Set initial cpu status and memory
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![
            0b01_100_011, // LD H,E
            0b01_001_010, // LD C,D
            0,
            0,
        ])
        .with_d(19)
        .with_e(26)
        .build();

    // Load registers while pc = 0
    cpu.ld_r_r1();
    assert_eq!(cpu.a, 0);
    assert_eq!(cpu.b, 0);
    assert_eq!(cpu.c, 0);
    assert_eq!(cpu.f, 0);
    assert_eq!(cpu.h, 26);
    assert_eq!(cpu.l, 0);
    assert_eq!(cpu.f, 0);
    assert_eq!(cpu.i, 0);
    assert_eq!(cpu.pc, 1);

    // Load registers while pc = 1
    cpu.ld_r_r1();
    assert_eq!(cpu.c, 19);
    assert_eq!(cpu.pc, 2);
}

#[test]
fn ld_r_n() {
    // Set initial cpu status and memory
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![
            0b00_100_110, // LD H, 0xFA
            0xfa,
            0b00_000_110, // LD B, 0xFB
            0xfb,
            0,
            0,
            0,
            0,
        ])
        .build();

    // Load registers while pc = 0
    cpu.ld_r_n();
    assert_eq!(cpu.a, 0);
    assert_eq!(cpu.b, 0);
    assert_eq!(cpu.c, 0);
    assert_eq!(cpu.f, 0);
    assert_eq!(cpu.h, 0xfa);
    assert_eq!(cpu.l, 0);
    assert_eq!(cpu.f, 0);
    assert_eq!(cpu.i, 0);
    assert_eq!(cpu.pc, 2);

    // Load registers while pc = 2
    cpu.ld_r_n();
    assert_eq!(cpu.b, 0xfb);
    assert_eq!(cpu.pc, 4);
}

#[test]
fn ld_r_hl() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![
            0b01_000_110, // LD B, (HL)
            0x01,
            0xfb,
            0,
        ])
        .with_hl(2)
        .build();

    cpu.ld_r_hl();

    assert_eq!(cpu.b, 0xfb);
    assert_eq!(cpu.pc, 1);
}

#[test]
fn ld_r_ixd() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![0xdd, 0b01_000_110, 0b1111_1111, 0, 0, 0xfb, 0, 0])
        .with_ix(4)
        .build();

    cpu.ld_r_ixd();

    assert_eq!(cpu.b, 0xfb);
    assert_eq!(cpu.pc, 3);
}

#[test]
fn ld_r_iyd() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![0xfd, 0b01_000_110, 0b1111_1111, 0, 0, 0xfb, 0, 0])
        .with_iy(4)
        .build();

    cpu.ld_r_iyd();

    assert_eq!(cpu.b, 0xfb);
    assert_eq!(cpu.pc, 3);
}

#[test]
fn ld_hl_r() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![
            0b01110_010, // LD (HL), D
            0,
            0,
            0,
        ])
        .with_d(0xfa)
        .with_hl(2)
        .build();

    cpu.ld_hl_r();

    assert_eq!(cpu.memory[2], 0xfa);
    assert_eq!(cpu.pc, 1);
}

#[test]
fn ld_ixd_r() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![
            0xdd, // LD (IX+D), R
            0b01110_111,
            0xff, // D
            1,
            2,
            3,
            4,
            5,
        ])
        .with_ix(4)
        .with_a(0xfb)
        .build();

    cpu.ld_ixd_r();

    assert_eq!(cpu.memory[5], 0xfb);
    assert_eq!(cpu.pc, 3);
}

#[test]
fn ld_iyd_r() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![
            0xfd, // LD (IY+D), R
            0b01110_111,
            0xff, // D
            1,
            2,
            3,
            4,
            5,
        ])
        .with_iy(4)
        .with_a(0xfb)
        .build();

    cpu.ld_iyd_r();

    assert_eq!(cpu.memory[5], 0xfb);
    assert_eq!(cpu.pc, 3);
}

#[test]
fn ld_hl_n() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![
            0x36, // LD (HL), N
            0xfa, 0, 0,
        ])
        .with_hl(3)
        .build();

    cpu.ld_hl_n();

    assert_eq!(cpu.memory[3], 0xfa);
    assert_eq!(cpu.pc, 2);
}

#[test]
fn ld_ixd_n() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![
            0xdd, // LD (IX+D), N
            0x36, 0x02, // D
            0xfb, // N
            1, 2, 3, 4,
        ])
        .with_ix(4)
        .build();

    cpu.ld_ixd_n();

    assert_eq!(cpu.memory[6], 0xfb);
    assert_eq!(cpu.pc, 4);
}

#[test]
fn ld_iyd_n() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![
            0xfd, // LD (IY+D), N
            0x36, 0x02, // D
            0xfb, // N
            1, 2, 3, 4,
        ])
        .with_iy(4)
        .build();

    cpu.ld_iyd_n();
    assert_eq!(cpu.memory[6], 0xfb);
    assert_eq!(cpu.pc, 4);
}

#[test]
fn ld_a_bc() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![
            0x0a, // LD A, (BC)
            0x01, 0xf2, 0x03,
        ])
        .with_bc(2)
        .build();

    cpu.ld_a_bc();

    assert_eq!(cpu.a, 0xf2);
    assert_eq!(cpu.pc, 1);
}

#[test]
fn ld_a_de() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![
            0x1a, // LD A, (DE)
            0x01, 0xf2, 0x03,
        ])
        .with_de(2)
        .build();

    cpu.ld_a_de();

    assert_eq!(cpu.a, 0xf2);
    assert_eq!(cpu.pc, 1);
}

#[test]
fn ld_a_nn() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![
            0x3a, // LD A, NN
            0x03, 0, 0x13,
        ])
        .build();

    cpu.ld_a_nn();

    assert_eq!(cpu.a, 0x13);
    assert_eq!(cpu.pc, 3);
}

#[test]
fn ld_a_i() {
    let mut cpu = CpuBuilder::new()
        .with_memory_size(16)
        .with_i(0x3b)
        .with_iff2(false)
        .with_flag_c(true)
        .build();

    cpu.ld_a_i();

    assert_eq!(cpu.a, 0x3b);
    assert_eq!(cpu.pc, 2);
    assert_eq!(cpu.get_s(), (cpu.i as i8) < 0);
    assert_eq!(cpu.get_z(), cpu.i == 0);
    assert_eq!(cpu.get_h(), false);
    assert_eq!(cpu.get_pv(), cpu.iff2);
    assert_eq!(cpu.get_n(), false);
    assert_eq!(cpu.get_c(), true);
}

#[test]
fn ld_a_r() {
    let mut cpu = CpuBuilder::new()
        .with_memory_size(16)
        .with_r(0x3b)
        .with_iff2(false)
        .with_flag_c(true)
        .build();

    cpu.ld_a_r();

    assert_eq!(cpu.r, 0x3b);
    assert_eq!(cpu.pc, 2);
    assert_eq!(cpu.get_s(), (cpu.r as i8) < 0);
    assert_eq!(cpu.get_z(), cpu.r == 0);
    assert_eq!(cpu.get_h(), false);
    assert_eq!(cpu.get_pv(), cpu.iff2);
    assert_eq!(cpu.get_n(), false);
    assert_eq!(cpu.get_c(), true);
}

#[test]
fn ld_i_a() {
    let mut cpu = CpuBuilder::new().with_memory_size(16).with_a(12).build();

    cpu.ld_i_a();

    assert_eq!(cpu.i, 12);
    assert_eq!(cpu.pc, 2);
}

#[test]
fn ld_r_a() {
    let mut cpu = CpuBuilder::new().with_memory_size(16).with_a(12).build();

    cpu.ld_r_a();

    assert_eq!(cpu.r, 12);
    assert_eq!(cpu.pc, 2);
}
