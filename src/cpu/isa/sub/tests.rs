use cpu::Assertor;
use cpu::CpuBuilder;

// === 8-Bit Arithmetic Group / SUb ===

#[test]
pub fn sub_r() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![0b10010_010, 0x66, 0x66, 0x66])
        .with_a(0x29)
        .with_d(0x11)
        .build();

    cpu.sub_r();

    Assertor::new(cpu)
        .register_a_is(0x18)
        .sign_flag_is_positive()
        .zero_flag_is_reset()
        //.parity_overflow_flag_is_reset()
        .add_subtract_flag_is_set()
        .program_counter_is(1);
}

#[test]
pub fn sub_n() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![0xd6, 0x11, 0x66, 0x66])
        .with_a(0x29)
        .build();

    cpu.sub_n();

    Assertor::new(cpu)
        .register_a_is(0x18)
        .sign_flag_is_positive()
        .zero_flag_is_reset()
        //.parity_overflow_flag_is_reset()
        .add_subtract_flag_is_set()
        .program_counter_is(2);
}

#[test]
pub fn sub_hli() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![0x96, 0x11, 0x66, 0x66])
        .with_a(0x29)
        .with_hl(0x1)
        .build();

    cpu.sub_hli();

    Assertor::new(cpu)
        .register_a_is(0x18)
        .sign_flag_is_positive()
        .zero_flag_is_reset()
        //.parity_overflow_flag_is_reset()
        .add_subtract_flag_is_set()
        .program_counter_is(1);
}

#[test]
pub fn sub_ixdi() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![0xdd, 0x96, 0x02, 0x11])
        .with_a(0x29)
        .with_ix(0x01)
        .build();

    cpu.sub_ixdi();

    Assertor::new(cpu)
        .register_a_is(0x18)
        .sign_flag_is_positive()
        .zero_flag_is_reset()
        //.parity_overflow_flag_is_reset()
        .add_subtract_flag_is_set()
        .program_counter_is(3);
}

#[test]
pub fn sub_iydi() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![0xdd, 0x96, 0x02, 0x11])
        .with_a(0x29)
        .with_iy(0x01)
        .build();

    cpu.sub_iydi();

    Assertor::new(cpu)
        .register_a_is(0x18)
        .sign_flag_is_positive()
        .zero_flag_is_reset()
        //.parity_overflow_flag_is_reset()
        .add_subtract_flag_is_set()
        .program_counter_is(3);
}

#[test]
fn sbc_a_r() {
    unimplemented!();
}

#[test]
fn sbc_a_n() {
    unimplemented!();
}

#[test]
fn sbc_a_hli() {
    unimplemented!();
}

#[test]
fn sbc_a_ixdi() {
    unimplemented!();
}

#[test]
fn sbc_a_iydi() {
    unimplemented!();
}
