use cpu::Assertor;
use cpu::CpuBuilder;

// === 8-Bit Arithmetic Group / ADD ===

#[test]
fn add_a_r() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![0b10000_001, 0x66, 0x66, 0x66])
        .with_a(7)
        .with_c(4)
        .build();

    cpu.add_a_r();

    Assertor::new(cpu)
        .register_a_is(11)
        .sign_flag_is_positive()
        .zero_flag_is_reset()
        .parity_overflow_flag_is_reset()
        .add_subtract_flag_is_reset()
        .program_counter_is(1);
}

#[test]
fn add_a_n() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![0xc6, 0x04, 0x66, 0x66])
        .with_a(7)
        .build();

    cpu.add_a_n();

    Assertor::new(cpu)
        .register_a_is(11)
        .zero_flag_is_reset()
        .program_counter_is(2);
}

#[test]
fn add_a_hli() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![0x86, 0x66, 0x66, 0x04])
        .with_a(7)
        .with_hl(3)
        .build();

    cpu.add_a_hli();

    Assertor::new(cpu)
        .register_a_is(11)
        .zero_flag_is_reset()
        .program_counter_is(1);
}

#[test]
fn add_a_ixdi() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![0xdd, 0x86, 0x02, 0x04])
        .with_a(7)
        .with_ix(1)
        .build();

    cpu.add_a_ixdi();

    Assertor::new(cpu)
        .register_a_is(11)
        .zero_flag_is_reset()
        .program_counter_is(3);
}

#[test]
fn add_a_iydi() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![0xdd, 0x86, 0x02, 0x04])
        .with_a(7)
        .with_iy(1)
        .build();

    cpu.add_a_iydi();

    Assertor::new(cpu)
        .register_a_is(11)
        .zero_flag_is_reset()
        .program_counter_is(3);
}
