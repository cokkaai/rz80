// === 16-Bit Load Group ===

use cpu::CpuBuilder;
use cpu::Assertor;

#[test]
fn ld_dd_nn() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![
            0b00_00_0001, // LD BC 0x0402
            0x02,
            0xa4,
            0b00_11_0001, // LD SP 0xcaf3
            0xf3,
            0xca,
            0x66,
            0x66,
        ])
        .build();

    cpu.ld_dd_nn();

    Assertor::new(cpu)
        .register_bc_is(0xa402)
        .program_counter_is(3);
}

#[test]
fn ld_ix_nn() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![
            0xdd, // LD IX, 0xcaf3
            0x21, 0xf3, 0xca, 0xdd, // LD IX, 0xaaff
            0x21, 0xff, 0xaa, 0x66, 0x66,
        ])
        .build();

    cpu.ld_ix_nn();

    Assertor::new(cpu)
        .index_register_ix_is(0xcaf3)
        .program_counter_is(4);
}

#[test]
fn ld_iy_nn() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![
            0xfd, // LD IY, 0xcaf3
            0x21, 0xf3, 0xca, 0xfd, // LD IY, 0xaaff
            0x21, 0xff, 0xaa, 0x66, 0x66,
        ])
        .build();

    cpu.ld_iy_nn();

    Assertor::new(cpu)
        .index_register_iy_is(0xcaf3)
        .program_counter_is(4);
}

#[test]
fn ld_hl_nni() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![
            0x2a, // LD HL, (0x000a)
            0x04, 0x0, 0x66, 0xf3, 0xca, 0x66, 0x66,
        ])
        .build();

    cpu.ld_hl_nni();

    Assertor::new(cpu)
        .register_h_is(0xca)
        .register_l_is(0xf3)
        .program_counter_is(3);
}

#[test]
fn ld_dd_nni() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![
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
        ])
        .build();

    cpu.ld_dd_nni();

    Assertor::new(cpu)
        .register_bc_is(0x0201)
        .program_counter_is(4);
}

#[test]
fn ld_ix_nni() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![
            0xdd, // LD IX, (0x0006)
            0x2a, 0x06, 0x0, 0x66, 0x66, 0xf3, 0xca,
        ])
        .build();

    cpu.ld_ix_nni();

    Assertor::new(cpu)
        .index_register_ix_is(0xcaf3)
        .program_counter_is(4);
}

#[test]
fn ld_iy_nni() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![
            0xfd, // LD IY, (0x0006)
            0x2a, 0x06, 0x0, 0x66, 0x66, 0xf3, 0xca,
        ])
        .build();

    cpu.ld_iy_nni();

    Assertor::new(cpu)
        .index_register_iy_is(0xcaf3)
        .program_counter_is(4);
}

#[test]
fn ld_nni_hl() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![
            0x22, // LD (0x0006), HL
            0x06, 0x0, 0x66, 0x66, 0x66, 0x55, // Data
            0x55,
        ])
        .with_h(0xca)
        .with_l(0xf3)
        .build();

    cpu.ld_nni_hl();

    Assertor::new(cpu)
        .memory_at_address_is(6, 0xf3)
        .memory_at_address_is(7, 0xca)
        .program_counter_is(3);
}

#[test]
fn ld_nni_dd() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![
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
        ])
        .with_sp(0xcaf3)
        .with_h(0xba)
        .with_l(0xb3)
        .build();

    cpu.ld_nni_dd();

    Assertor::new(cpu)
        .memory_at_address_is(8, 0xf3)
        .memory_at_address_is(9, 0xca)
        .program_counter_is(4);
}

#[test]
fn ld_nni_ix() {
let mut cpu = CpuBuilder::new()
    .with_memory(vec![
        0xdd, // LD (0x0006), IX
        0x22, 0x06, 0x0, 0x66, 0x66, 0x55, 0x55,
    ])
    .with_ix(0xcaf3)
    .build();

cpu.ld_nni_ix();


Assertor::new(cpu)
    .memory_at_address_is(6, 0xf3)
    .memory_at_address_is(7, 0xca)
    .program_counter_is(4);
}

#[test]
fn ld_nni_iy() {
let mut cpu = CpuBuilder::new()
    .with_memory(vec![
        0xfd, // LD (0x0006), IY
        0x22, 0x06, 0x0, 0x66, 0x66, 0x55, 0x55,
    ])
    .with_iy(0xcaf3)
    .build();

cpu.ld_nni_iy();

Assertor::new(cpu)
    .memory_at_address_is(6, 0xf3)
    .memory_at_address_is(7, 0xca)
    .program_counter_is(4);
}

#[test]
fn ld_sp_hl() {
let mut cpu = CpuBuilder::new()
    .with_memory_size(16)
    .with_hl(0xcaf3)
    .with_sp(0x1234)
    .build();

cpu.ld_sp_hl();


Assertor::new(cpu)
    .stack_pointer_is(0xcaf3)
    .program_counter_is(1);
}

#[test]
fn ld_sp_ix() {
let mut cpu = CpuBuilder::new()
    .with_memory_size(16)
    .with_ix(0xcaf3)
    .with_sp(0x1234)
    .build();

cpu.ld_sp_ix();

Assertor::new(cpu)
    .stack_pointer_is(0xcaf3)
    .program_counter_is(2);
}

#[test]
fn ld_sp_iy() {
let mut cpu = CpuBuilder::new()
    .with_memory_size(16)
    .with_iy(0xcaf3)
    .with_sp(0x1234)
    .build();

cpu.ld_sp_iy();

Assertor::new(cpu)
    .stack_pointer_is(0xcaf3)
    .program_counter_is(2);
}

#[test]
fn push_qq() {
let mut cpu = CpuBuilder::new()
.with_memory(
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
)
.with_sp(0x0008)
.with_b(0xca) // BC = 0xcaf3
.with_c(0xf3)
.with_h(0x12) // HL = 0x1234
.with_l(0x34)
.build();

cpu.push_qq();

Assertor::new(cpu)
    .memory_at_address_is(7, 0xca)
    .memory_at_address_is(6, 0xf3)
    .stack_pointer_is(6)
    .program_counter_is(1);
}

#[test]
fn push_ix() {
let mut cpu = CpuBuilder::new()
    .with_memory(vec![
        0xdd, // PUSH IX
        0xe5, 0, 0, 0, 0, 0, 0,
    ])
    .with_sp(0x0008)
    .with_ix(0xcaf3)
    .build();

cpu.push_ix();


Assertor::new(cpu)
    .memory_at_address_is(7, 0xca)
    .memory_at_address_is(6, 0xf3)
    .stack_pointer_is(6)
    .program_counter_is(2);
}

#[test]
fn push_iy() {
let mut cpu = CpuBuilder::new()
    .with_memory(vec![
        0xfd, // PUSH IY
        0xe5, 0, 0, 0, 0, 0, 0,
    ])
    .with_sp(0x0008)
    .with_iy(0xcaf3)
    .build();

cpu.push_iy();

Assertor::new(cpu)
    .memory_at_address_is(7, 0xca)
    .memory_at_address_is(6, 0xf3)
    .stack_pointer_is(6)
    .program_counter_is(2);
}

#[test]
fn pop_qq() {
let mut cpu = CpuBuilder::new()
    .with_memory(vec![
        0b11_11_0001, // POP AF
        0,
        0,
        0,
        0,
        0,
        0xf3,
        0xca,
    ])
    .with_sp(0x0006)
    .build();

cpu.pop_qq();

Assertor::new(cpu)
    .register_a_is(0xca)
    .register_f_is(0xf3)
    .stack_pointer_is(8)
    .program_counter_is(1);
}

#[test]
fn pop_ix() {
let mut cpu = CpuBuilder::new()
    .with_memory(vec![
        0xdd, // POP IX
        0xe1, 0, 0, 0, 0, 0xf3, 0xca,
    ])
    .with_sp(0x0006)
    .build();

cpu.pop_ix();

Assertor::new(cpu)
    .index_register_ix_is(0xcaf3)
    .stack_pointer_is(8)
    .program_counter_is(2);
}

#[test]
fn pop_iy() {
let mut cpu = CpuBuilder::new()
    .with_memory(vec![
        0xfd, // POP IY
        0xe1, 0, 0, 0, 0, 0xf3, 0xca,
    ])
    .with_sp(0x0006)
    .build();

cpu.pop_iy();

Assertor::new(cpu)
    .index_register_iy_is(0xcaf3)
        .stack_pointer_is(8)
        .program_counter_is(2);
}
