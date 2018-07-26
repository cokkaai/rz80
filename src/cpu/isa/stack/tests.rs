// === 16-Bit Load Group ===

use cpu::CpuBuilder;
use cpu::Assertor;

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
