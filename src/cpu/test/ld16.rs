// === 16-Bit Load Group ===

use cpu::Register16;
use cpu::CPU;

#[test]
fn ld_dd_nn() {
    let mut cpu = CPU::with_memory(
        vec![
            0b00_00_0001, // LD BC 0x0402
            0x02, 
            0xa4, 
            0b00_11_0001, // LD SP 0xcaf3
            0xf3, 
            0xca,
            0x66,
            0x66,
        ],
    );

    cpu.ld_dd_nn();
    assert_eq!(cpu.read16(Register16::bc), 0xa402);
    assert_eq!(cpu.pc, 3);
    
    cpu.ld_dd_nn();
    assert_eq!(cpu.sp, 0xcaf3);
    assert_eq!(cpu.pc, 6);
}

#[test]
fn ld_ix_nn() {
    let mut cpu = CPU::with_memory(
        vec![
            0xdd, // LD IX, 0xcaf3
            0x21,
            0xf3,
            0xca,
            0xdd, // LD IX, 0xaaff
            0x21,
            0xff,
            0xaa,
            0x66,
            0x66,
        ],
    );

    cpu.ld_ix_nn();
    assert_eq!(cpu.ix, 0xcaf3);
    assert_eq!(cpu.pc, 4);

    cpu.ld_ix_nn();
    assert_eq!(cpu.ix, 0xaaff);
    assert_eq!(cpu.pc, 8);
}

#[test]
fn ld_iy_nn() {
    let mut cpu = CPU::with_memory(
        vec![
            0xfd, // LD IY, 0xcaf3
            0x21,
            0xf3,
            0xca,
            0xfd, // LD IY, 0xaaff
            0x21,
            0xff,
            0xaa,
            0x66,
            0x66,
        ],
    );

    cpu.ld_iy_nn();
    assert_eq!(cpu.iy, 0xcaf3);
    assert_eq!(cpu.pc, 4);
    
    cpu.ld_iy_nn();
    assert_eq!(cpu.iy, 0xaaff);
    assert_eq!(cpu.pc, 8);
}

#[test]
fn ld_hl_nni() {
    let mut cpu = CPU::with_memory(
        vec![
            0x2a, // LD HL, (0x000a)
            0x04,
            0x0,
            0x66,
            0xf3,
            0xca,
            0x66,
            0x66,
        ],
    );

    cpu.ld_hl_nni();

    assert_eq!(cpu.h, 0xca);
    assert_eq!(cpu.l, 0xf3);
    assert_eq!(cpu.pc, 3);
}

#[test]
fn ld_dd_nni() {
    let mut cpu = CPU::with_memory(
        vec![
            0xed, // LD BC, (0x000a)
            0b01_00_1011,
            0x0a,
            0x0,
            0xed, // LD SP, (0x00c0)
            0b01_11_1011,
            0x0c,
            0x0,
            0x66,
            0x66,
            0x01,
            0x02,
            0xf3,
            0xca,
        ],
    );
    
    cpu.ld_dd_nni();
    assert_eq!(cpu.read16(Register16::bc), 0x0201);
    assert_eq!(cpu.pc, 4);

    cpu.ld_dd_nni();
    assert_eq!(cpu.sp, 0xcaf3);
    assert_eq!(cpu.pc, 8);
}

#[test]
fn ld_ix_nni() {
    let mut cpu = CPU::with_memory(
        vec![
            0xdd, // LD IX, (0x0006)
            0x2a,
            0x06,
            0x0,
            0x66,
            0x66,
            0xf3,
            0xca,
        ],
    );

    cpu.ld_ix_nni();

    assert_eq!(cpu.ix, 0xcaf3);
    assert_eq!(cpu.pc, 4);
}

#[test]
fn ld_iy_nni() {
    let mut cpu = CPU::with_memory(
        vec![
            0xfd, // LD IY, (0x0006)
            0x2a,
            0x06,
            0x0,
            0x66,
            0x66,
            0xf3,
            0xca,
        ],
    );

    cpu.ld_iy_nni();

    assert_eq!(cpu.iy, 0xcaf3);
    assert_eq!(cpu.pc, 4);
}

#[test]
fn ld_nni_hl() {
    let mut cpu = CPU::with_memory(
        vec![
            0x22, // LD (0x0006), HL
            0x06,
            0x0,
            0x66,
            0x66,
            0x66,
            0x55, // Data
            0x55,
        ],
    );
    cpu.h = 0xca;
    cpu.l = 0xf3;

    cpu.ld_nni_hl();

    assert_eq!(cpu.memory[0x06], 0xf3);
    assert_eq!(cpu.memory[0x07], 0xca);
    assert_eq!(cpu.pc, 3);
}

#[test]
fn ld_nni_dd() {
    let mut cpu = CPU::with_memory(
        vec![
            0xed, // LD (0x0008), SP
            0b01_11_0011,
            0x08,
            0x0,
            0xed, // LD (0x000a), HL
            0b01_10_0011,
            0x0a,
            0x0,
            0x44, // Data
            0x44,
            0x55,
            0x55,
        ],
    );
    cpu.sp = 0xcaf3;
    cpu.h = 0xba;
    cpu.l = 0xb3;

    cpu.ld_nni_dd();
    assert_eq!(cpu.memory[0x0008], 0xf3);
    assert_eq!(cpu.memory[0x0009], 0xca);
    assert_eq!(cpu.pc, 4);

    cpu.ld_nni_dd();
    assert_eq!(cpu.memory[0x000a], 0xb3);
    assert_eq!(cpu.memory[0x000b], 0xba);
    assert_eq!(cpu.pc, 8);
}

#[test]
fn ld_nni_ix() {
    let mut cpu = CPU::with_memory(
        vec![
            0xdd, // LD (0x0006), IX
            0x22,
            0x06,
            0x0,
            0x66,
            0x66,
            0x55,
            0x55,
        ],
    );
    cpu.ix = 0xcaf3;

    cpu.ld_nni_ix();

    assert_eq!(cpu.memory[0x06], 0xf3);
    assert_eq!(cpu.memory[0x07], 0xca);
    assert_eq!(cpu.pc, 4);
}

#[test]
fn ld_nni_iy() {
    let mut cpu = CPU::with_memory(
        vec![
            0xfd, // LD (0x0006), IY
            0x22,
            0x06,
            0x0,
            0x66,
            0x66,
            0x55,
            0x55,
        ],
    );
    cpu.iy = 0xcaf3;

    cpu.ld_nni_iy();

    assert_eq!(cpu.memory[0x06], 0xf3);
    assert_eq!(cpu.memory[0x07], 0xca);
    assert_eq!(cpu.pc, 4);
}

#[test]
fn ld_sp_hl() {
    let mut cpu = CPU::new(16);
    cpu.write_hl(0xcaf3);
    cpu.sp = 0x1234;

    cpu.ld_sp_hl();
    
    assert_eq!(cpu.sp, 0xcaf3);
    assert_eq!(cpu.pc, 1);
}

#[test]
fn ld_sp_ix() {
    let mut cpu = CPU::new(16);
    cpu.ix = 0xcaf3;
    cpu.sp = 0x1234;

    cpu.ld_sp_ix();
    
    assert_eq!(cpu.sp, 0xcaf3);
    assert_eq!(cpu.pc, 2);
}

#[test]
fn ld_sp_iy() {
    let mut cpu = CPU::new(16);
    cpu.iy = 0xcaf3;
    cpu.sp = 0x1234;

    cpu.ld_sp_iy();
    
    assert_eq!(cpu.sp, 0xcaf3);
    assert_eq!(cpu.pc, 2);
}

#[test]
fn push_qq() {
    let mut cpu = CPU::with_memory(
        vec![
            0b11_00_0101, // PUSH BC
            0b11_10_0101, // PUSH HL
            0,
            0,
            0,
            0,
            0,
            0,
        ],
    );

    cpu.sp = 0x0008;
    cpu.b = 0xca; // BC = 0xcaf3
    cpu.c = 0xf3;
    cpu.h = 0x12; // HL = 0x1234
    cpu.l = 0x34;

    cpu.push_qq();
    assert_eq!(cpu.memory[7], 0xca);
    assert_eq!(cpu.memory[6], 0xf3);
    assert_eq!(cpu.sp, 6);
    assert_eq!(cpu.pc, 1);

    cpu.push_qq();
    assert_eq!(cpu.memory[5], 0x12);
    assert_eq!(cpu.memory[4], 0x34);
    assert_eq!(cpu.sp, 4);
    assert_eq!(cpu.pc, 2);
}

#[test]
fn push_ix() {
    let mut cpu = CPU::with_memory(
        vec![
            0xdd, // PUSH IX
            0xe5,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
    );

    cpu.sp = 0x0008;
    cpu.ix = 0xcaf3;

    cpu.push_ix();
    assert_eq!(cpu.memory[7], 0xca);
    assert_eq!(cpu.memory[6], 0xf3);
    assert_eq!(cpu.sp, 6);
    assert_eq!(cpu.pc, 2);
}

#[test]
fn push_iy() {
    let mut cpu = CPU::with_memory(
        vec![
            0xfd, // PUSH IY
            0xe5,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
    );

    cpu.sp = 0x0008;
    cpu.iy = 0xcaf3;

    cpu.push_iy();
    assert_eq!(cpu.memory[7], 0xca);
    assert_eq!(cpu.memory[6], 0xf3);
    assert_eq!(cpu.sp, 6);
    assert_eq!(cpu.pc, 2);
}

#[test]
fn pop_qq() {
    let mut cpu = CPU::with_memory(
        vec![
            0b11_11_0001, // POP AF
            0,
            0,
            0,
            0,
            0,
            0xf3,
            0xca,
        ],
    );

    cpu.sp = 0x0006;

    cpu.pop_qq();
    assert_eq!(cpu.a, 0xca);
    assert_eq!(cpu.f, 0xf3);
    assert_eq!(cpu.sp, 8);
    assert_eq!(cpu.pc, 1);
}

#[test]
fn pop_ix() {
    let mut cpu = CPU::with_memory(
        vec![
            0xdd, // POP IX
            0xe1,
            0,
            0,
            0,
            0,
            0xf3,
            0xca,
        ],
    );

    cpu.sp = 0x0006;

    cpu.pop_ix();
    assert_eq!(cpu.ix, 0xcaf3);
    assert_eq!(cpu.sp, 8);
    assert_eq!(cpu.pc, 2);
}

#[test]
fn pop_iy() {
    let mut cpu = CPU::with_memory(
        vec![
            0xfd, // POP IY
            0xe1,
            0,
            0,
            0,
            0,
            0xf3,
            0xca,
        ],
    );

    cpu.sp = 0x0006;

    cpu.pop_iy();
    assert_eq!(cpu.iy, 0xcaf3);
    assert_eq!(cpu.sp, 8);
    assert_eq!(cpu.pc, 2);
}
