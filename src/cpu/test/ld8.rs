use cpu::CPU;
use cpu::bytes;

#[test]
fn memory_at_pc() {
    let cpu = CPU::with_memory(
        vec![
            0x01, 0x02, 0xa4, 0xa8, 0x0f, 0x11, 0x12, 0x14, 0x18, 0x1f, 0x22, 0x33, 0x44, 0x55,
            0x66, 0x77,
        ],
    );

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
    let mut cpu = CPU::new(16);

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
    let mut cpu = CPU::with_memory(
        vec![
            0b01_100_011, // LD H,E
            0b01_001_010, // LD C,D
            0,
            0,
        ],
    );
    cpu.d = 19;
    cpu.e = 26;

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
    let mut cpu = CPU::with_memory(
        vec![
            0b00_100_110, // LD H, 0xFA
            0xfa,
            0b00_000_110, // LD B, 0xFB
            0xfb,
            0,
            0,
            0,
            0,
        ],
        );

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
    let mut cpu = CPU::with_memory(
        vec![
            0b01_000_110, // LD B, (HL)
            0x01,
            0xfb,
            0,
        ],
    );
    cpu.write_hl(2);

    cpu.ld_r_hl();
    assert_eq!(cpu.b, 0xfb);
    assert_eq!(cpu.pc, 1);
}

#[test]
fn ld_r_ixd() {
    let mut cpu = CPU::with_memory(
        vec![0xdd, 0b01_000_110, 0b1111_1111, 0, 0, 0xfb, 0, 0],
    );
    cpu.ix = 4;
    cpu.ld_r_ixd();
    assert_eq!(cpu.b, 0xfb);
    assert_eq!(cpu.pc, 3);
}

#[test]
fn ld_r_iyd() {
    let mut cpu = CPU::with_memory(
        vec![0xfd, 0b01_000_110, 0b1111_1111, 0, 0, 0xfb, 0, 0],
    );
    cpu.iy = 4;
    cpu.ld_r_iyd();
    assert_eq!(cpu.b, 0xfb);
    assert_eq!(cpu.pc, 3);
}

#[test]
fn ld_hl_r() {
    let mut cpu = CPU::with_memory(
        vec![
            0b01110_010, // LD (HL), D
            0,
            0,
            0,
        ],
    );
    cpu.d = 0xfa;
    cpu.write_hl(2);

    cpu.ld_hl_r();
    assert_eq!(cpu.memory[2], 0xfa);
    assert_eq!(cpu.pc, 1);
}

#[test]
fn ld_ixd_r() {
    let mut cpu = CPU::with_memory(
        vec![
            0xdd, // LD (IX+D), R
            0b01110_111,
            0xff, // D
            1,
            2,
            3,
            4,
            5,
        ],
    );
    cpu.ix = 4;
    cpu.a = 0xfb;
    cpu.ld_ixd_r();
    assert_eq!(cpu.memory[5], 0xfb);
    assert_eq!(cpu.pc, 3);
}

#[test]
fn ld_iyd_r() {
    let mut cpu = CPU::with_memory(
        vec![
            0xfd, // LD (IY+D), R
            0b01110_111,
            0xff, // D
            1,
            2,
            3,
            4,
            5,
        ],
    );
    cpu.iy = 4;
    cpu.a = 0xfb;
    cpu.ld_iyd_r();
    assert_eq!(cpu.memory[5], 0xfb);
    assert_eq!(cpu.pc, 3);
}

#[test]
fn ld_hl_n() {
    let mut cpu = CPU::with_memory(
        vec![
            0x36, // LD (HL), N
            0xfa,
            0,
            0,
        ],
    );
    cpu.write_hl(3);

    cpu.ld_hl_n();
    assert_eq!(cpu.memory[3], 0xfa);
    assert_eq!(cpu.pc, 2);
}

#[test]
fn ld_ixd_n() {
    let mut cpu = CPU::with_memory(
        vec![
            0xdd, // LD (IX+D), N
            0x36,
            0x02, // D
            0xfb, // N
            1,
            2,
            3,
            4,
        ],
    );
    cpu.ix = 4;
    cpu.ld_ixd_n();
    assert_eq!(cpu.memory[6], 0xfb);
    assert_eq!(cpu.pc, 4);
}

#[test]
fn ld_iyd_n() {
    let mut cpu = CPU::with_memory(
        vec![
            0xfd, // LD (IY+D), N
            0x36,
            0x02, // D
            0xfb, // N
            1,
            2,
            3,
            4,
        ],
    );
    cpu.iy = 4;
    cpu.ld_iyd_n();
    assert_eq!(cpu.memory[6], 0xfb);
    assert_eq!(cpu.pc, 4);
}

#[test]
fn ld_a_bc() {
    let mut cpu = CPU::with_memory(
        vec![
            0x0a, // LD A, (BC)
            0x01,
            0xf2,
            0x03,
        ],
    );
    cpu.write_bc(2);
    cpu.ld_a_bc();
    assert_eq!(cpu.a, 0xf2);
    assert_eq!(cpu.pc, 1);
}

#[test]
fn ld_a_de() {
    let mut cpu = CPU::with_memory(
        vec![
            0x1a, // LD A, (DE)
            0x01,
            0xf2,
            0x03,
        ],
    );
    cpu.write_de(2);
    cpu.ld_a_de();
    assert_eq!(cpu.a, 0xf2);
    assert_eq!(cpu.pc, 1);
}

#[test]
fn ld_a_nn() {
    let mut cpu = CPU::with_memory(
        vec![
            0x3a, // LD A, NN
            0x03,
            0,
            0x13,
        ],
    );
    cpu.ld_a_nn();
    assert_eq!(cpu.a, 0x13);
    assert_eq!(cpu.pc, 3);
}

#[test]
fn ld_a_i() {
    let mut cpu = CPU::new(16);
    cpu.i = 0x3b;
    cpu.iff2 = false;
    cpu.set_c(true);
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
    let mut cpu = CPU::new(16);
    cpu.r = 0x3b;
    cpu.iff2 = false;
    cpu.set_c(true);
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
    let mut cpu = CPU::new(16);
    cpu.a = 12;
    cpu.ld_i_a();
    assert_eq!(cpu.i, 12);
    assert_eq!(cpu.pc, 2);
}

#[test]
fn ld_r_a() {
    let mut cpu = CPU::new(16);
    cpu.a = 12;
    cpu.ld_r_a();
    assert_eq!(cpu.r, 12);
    assert_eq!(cpu.pc, 2);
}
