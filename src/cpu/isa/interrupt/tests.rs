// === CPU Control Groups ===

use cpu::CpuBuilder;
use cpu::Assertor;

#[test]
fn nop() {
    let mut cpu = CpuBuilder::new().with_memory_size(10).build();

    cpu.nop();

    Assertor::new(cpu).program_counter_is(1);
}

#[test]
fn halt() {
    unimplemented!();
}

#[test]
fn di() {
    let mut cpu = CpuBuilder::new().with_memory_size(10).build();

    cpu.di();

    Assertor::new(cpu)
        .interrupt_flip_flop_1_is_reset()
        .interrupt_flip_flop_2_is_reset()
        .program_counter_is(1);
}

#[test]
fn ei() {
    let mut cpu = CpuBuilder::new().with_memory_size(10).build();

    cpu.ei();

    Assertor::new(cpu)
        .interrupt_flip_flop_1_is_set()
        .interrupt_flip_flop_2_is_set()
        .program_counter_is(1);
}

// IM 0
#[test]
fn im_0() {
    unimplemented!();
}

// IM 1
#[test]
fn im_1() {
    unimplemented!();
}

// IM 2
#[test]
fn im_2() {
    unimplemented!();
}
