use cpu::CpuBuilder;
use cpu::Assertor;

// === 16-Bit Arithmetic Group ===

#[test]
fn add_hl_ss() {
    let mut cpu = CpuBuilder::new()
        .with_hl(0x1000)
        .with_de(0x0001)
        .with_memory(vec![0b0001_1001, 0, 0, 0])
        .build();

    cpu.add_hl_ss();

    Assertor::new(cpu)
        .register_hl_is(0x1001)
        .add_subtract_flag_is_reset()
        .program_counter_is(1);
}

#[test]
fn adc_hl_ss() {
    let mut cpu = CpuBuilder::new()
        .with_hl(0x1000)
        .with_de(0x0001)
        .with_memory(vec![0xed, 0b0101_1001, 0, 0])
        .build();

    cpu.add_hl_ss();

    Assertor::new(cpu)
        .register_hl_is(0x1001)
        .add_subtract_flag_is_reset()
        .parity_overflow_flag_is_reset()
        .program_counter_is(2);
}

#[test]
fn sbc_hl_ss() {
    unimplemented!();
}

#[test]
fn add_ix_pp() {
    let mut cpu = CpuBuilder::new()
        .with_ix(0x1000)
        .with_de(0x0001)
        .with_memory(vec![0xdd, 0b0001_1001, 0, 0])
        .build();

    cpu.add_ix_pp();

    Assertor::new(cpu)
        .index_register_ix_is(0x1001)
        .add_subtract_flag_is_reset()
        .program_counter_is(2);
}

#[test]
fn add_iy_rr() {
    let mut cpu = CpuBuilder::new()
        .with_iy(0x1000)
        .with_de(0x0001)
        .with_memory(vec![0xfd, 0b0001_1001, 0, 0])
        .build();

    cpu.add_iy_rr();

    Assertor::new(cpu)
        .index_register_iy_is(0x1001)
        .add_subtract_flag_is_reset()
        .program_counter_is(2);
}

#[test]
fn inc_ss() {
    let mut cpu = CpuBuilder::new()
        .with_hl(0x01000)
        .with_memory(vec![0b0010_0011, 0, 0, 0])
        .build();

    cpu.inc_ss();

    Assertor::new(cpu)
        .register_hl_is(0x1001)
        .parity_overflow_flag_is_reset()
        .program_counter_is(1);
}

#[test]
fn inc_ix() {
    let mut cpu = CpuBuilder::new()
        .with_ix(0x2977)
        .with_memory_size(16)
        .build();

    cpu.inc_ix();

    Assertor::new(cpu)
        .index_register_ix_is(0x2978)
        .parity_overflow_flag_is_reset()
        .program_counter_is(2);
}

#[test]
fn inc_iy() {
    let mut cpu = CpuBuilder::new()
        .with_iy(0x2977)
        .with_memory_size(16)
        .build();

    cpu.inc_iy();

    Assertor::new(cpu)
        .index_register_iy_is(0x2978)
        .parity_overflow_flag_is_reset()
        .program_counter_is(2);
}

#[test]
fn dec_ss() {
    let mut cpu = CpuBuilder::new()
        .with_hl(0x1001)
        .with_memory(vec![0b0010_1011, 0, 0, 0])
        .build();

    cpu.dec_ss();

    Assertor::new(cpu)
        .register_hl_is(0x1000)
        .parity_overflow_flag_is_reset()
        .program_counter_is(1);
}

#[test]
fn dec_ix() {
    let mut cpu = CpuBuilder::new()
        .with_ix(0x2006)
        .with_memory_size(16)
        .build();

    cpu.dec_ix();

    Assertor::new(cpu)
        .index_register_ix_is(0x2005)
        .parity_overflow_flag_is_reset()
        .program_counter_is(2);
}

#[test]
fn dec_iy() {
    let mut cpu = CpuBuilder::new()
        .with_iy(0x7649)
        .with_memory_size(16)
        .build();

    cpu.dec_iy();

    Assertor::new(cpu)
        .index_register_iy_is(0x7648)
        .parity_overflow_flag_is_reset()
        .program_counter_is(2);
}
