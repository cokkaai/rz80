// === Call and Return Group ===

use cpu::CpuBuilder;
use cpu::Assertor;

#[test]
fn call_nn() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![0xcd, 0x04, 0x00, 0x00, 0xaa, 0xbb, 0xcc, 0xdd])
        .with_sp(8)
        .build();

    cpu.call_nn();

    Assertor::new(cpu)
        .memory_at_address_is(7, 0)
        .memory_at_address_is(6, 3)
        .program_counter_is(4);
}

#[test]
fn call_cc_nn() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![0b11_001_100, 0x04, 0x00, 0x00, 0xaa, 0xbb, 0xcc, 0xdd])
        .with_sp(8)
        .build();

    cpu.call_nn();

    Assertor::new(cpu)
        .memory_at_address_is(7, 0)
        .memory_at_address_is(6, 3)
        .program_counter_is(4);
}

#[test]
fn ret() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![0xc9, 0x04, 0x00, 0x00, 0xaa, 0xbb, 0x04, 0x00])
        .with_sp(6)
        .build();

    cpu.ret();

    Assertor::new(cpu)
        .stack_pointer_is(8)
        .program_counter_is(4);
}

#[test]
fn ret_cc() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![0b11_000_000, 0x04, 0x00, 0x00, 0xaa, 0xbb, 0x04, 0x00])
        .with_sp(6)
        .build();

    cpu.ret_cc();

    Assertor::new(cpu).program_counter_is(4);
}

#[test]
fn reti() {
    unimplemented!();
}

#[test]
fn retn() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![0xc9, 0x04, 0x00, 0x00, 0xaa, 0xbb, 0x04, 0x00])
        .with_sp(6)
        .with_iff1(false)
        .with_iff2(true)
        .build();

    cpu.retn();

    Assertor::new(cpu)
        .interrupt_flip_flop_1_is_set()
        .interrupt_flip_flop_1_is_set()
        .stack_pointer_is(8)
        .program_counter_is(4);
}

#[test]
fn rst_p() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![0b11_001_111, 0, 0, 0])
        .build();

    cpu.rst_p();

    Assertor::new(cpu).program_counter_is(8);
}
