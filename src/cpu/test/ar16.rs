use cpu::Cpu;
use cpu::CpuBuilder;
use cpu::RegisterPromote;
use cpu::assertor;

// === 16-Bit Arithmetic Group ===

// ADD HL, ss
#[test]
fn add_hl_ss() {
    unimplemented!();
}

// ADC HL, ss
#[test]
fn adc_hl_ss() {
    unimplemented!();
}

// SBC HL, ss
#[test]
fn sbc_hl_ss() {
    unimplemented!();
}

// ADD IX, pp
#[test]
fn add_ix_pp() {
    unimplemented!();
}

// ADD IY, rr
#[test]
fn add_iy_rr() {
    unimplemented!();
}

#[test]
fn inc_ss() {
    let mut cpu = CpuBuilder::new()
        .with_hl(0x1000)
        .with_memory(vec![0b0010_0011])
        .build();

    cpu.inc_ss();

    assert_eq!((cpu.h, cpu.l).promote(), 0x1001);
}

#[test]
fn inc_ix() {
    let mut cpu = CpuBuilder::new()
        .with_ix(0x2977)
        .with_memory_size(16)
        .build();

    cpu.inc_ix();

    assert_eq!(cpu.ix, 0x2978);
}

#[test]
fn inc_iy() {
    let mut cpu = CpuBuilder::new()
        .with_iy(0x2977)
        .with_memory_size(16)
        .build();

    cpu.inc_iy();

    assert_eq!(cpu.iy, 0x2978);
}

#[test]
fn dec_ss() {
    let mut cpu = CpuBuilder::new()
        .with_hl(0x1001)
        .with_memory(vec![0b0010_1011])
        .build();

    cpu.dec_ss();

    assert_eq!((cpu.h, cpu.l).promote(), 0x1000);
}

#[test]
fn dec_ix() {
    let mut cpu = CpuBuilder::new()
        .with_ix(0x2006)
        .with_memory_size(16)
        .build();

    cpu.dec_ix();

    assert_eq!(cpu.ix, 0x2005);
}

#[test]
fn dec_iy() {
    let mut cpu = CpuBuilder::new()
        .with_iy(0x7649)
        .with_memory_size(16)
        .build();

    cpu.dec_iy();

    assert_eq!(cpu.iy, 0x7648);
}
