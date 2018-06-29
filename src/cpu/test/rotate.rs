// === Rotate and Shift Group ===

use cpu::CpuBuilder;

#[test]
fn rlca() {
    let mut cpu = CpuBuilder::new()
        .with_a(0b1010_1010)
        .with_memory_size(16)
        .build();

    cpu.rlca();

    // S, Z, P/V are not affected.
    assert_eq!(cpu.get_s(), false);
    assert_eq!(cpu.get_z(), false);
    assert_eq!(cpu.get_pv(), false);

    // H, N are reset.
    assert_eq!(cpu.get_h(), false);
    assert_eq!(cpu.get_h(), false);

    // C is data from bit 7 of Accumulator.
    assert_eq!(cpu.get_c(), true);
    
    // The contents of the Accumulator (Register A) are rotated left 1 bit position.
    assert_eq!(cpu.a, 0b0101_0101);

    assert_eq!(cpu.pc, 1);
}

#[test]
fn rla() {
    let mut cpu = CpuBuilder::new()
        .with_a(0b0111_0110)
        .with_flag_c(true)
        .with_memory_size(16)
        .build();

    cpu.rla();

    // S, Z, P/V are not affected.
    assert_eq!(cpu.get_s(), false);
    assert_eq!(cpu.get_z(), false);
    assert_eq!(cpu.get_pv(), false);

    // H, N are reset.
    assert_eq!(cpu.get_h(), false);
    assert_eq!(cpu.get_n(), false);

    // C is data from bit 7 of Accumulator.
    assert_eq!(cpu.get_c(), false);
    
    // The contents of the Accumulator (Register A) are rotated left 1 bit position.
    assert_eq!(cpu.a, 0b1110_1101);

    assert_eq!(cpu.pc, 1);
}

#[test]
fn rrca() {
    let mut cpu = CpuBuilder::new()
        .with_a(0b0001_0001)
        .with_flag_c(false)
        .with_memory_size(16)
        .build();

    cpu.rrca();

    // S, Z, P/V are not affected.
    assert_eq!(cpu.get_s(), false);
    assert_eq!(cpu.get_z(), false);
    assert_eq!(cpu.get_pv(), false);

    // H, N are reset.
    assert_eq!(cpu.get_h(), false);
    assert_eq!(cpu.get_n(), false);

    // C is data from bit 0 of Accumulator.
    assert_eq!(cpu.get_c(), true);
    
    // The contents of the Accumulator (Register A) are rotated right 1 bit position.
    assert_eq!(cpu.a, 0b1000_1000);

    assert_eq!(cpu.pc, 1);
}

#[test]
fn rra() {
    let mut cpu = CpuBuilder::new()
        .with_a(0b0001_0001)
        .with_flag_c(false)
        .with_memory_size(16)
        .build();

    cpu.rra();

    // S, Z, P/V are not affected.
    assert_eq!(cpu.get_s(), false);
    assert_eq!(cpu.get_z(), false);
    assert_eq!(cpu.get_pv(), false);

    // H, N are reset.
    assert_eq!(cpu.get_h(), false);
    assert_eq!(cpu.get_n(), false);

    // C is data from bit 0 of Accumulator.
    assert_eq!(cpu.get_c(), true);
    
    // The contents of the Accumulator (Register A) are rotated right 1 bit
    // position through the Carry flag. The previous contents of the Carry
    // flag are copied to bit 7.
    assert_eq!(cpu.a, 0b0000_1000);

    assert_eq!(cpu.pc, 1);
}

#[test]
fn rlc_r() {
    let mut cpu = CpuBuilder::new()
        .with_l(0b1010_1010)
        .with_memory(vec!(0xcb, 0b0000_0101, 0, 0))
        .build();

    cpu.rlc_r();

    // S is set if result is negative; otherwise, it is reset.
    assert_eq!(cpu.get_s(), false);

    // Z is set if result is 0; otherwise, it is reset.
    assert_eq!(cpu.get_z(), false);
    
    // P/V is set if parity even; otherwise, it is reset.
    assert_eq!(cpu.get_pv(), false);

    // H, N are reset.
    assert_eq!(cpu.get_h(), false);
    assert_eq!(cpu.get_n(), false);

    // C is data from bit 7 of source register.
    assert_eq!(cpu.get_c(), true);
    
    // The contents of register r are rotated left 1 bit position. The contents of bit 7 
    // are copied to the Carry flag and also to bit 0.
    assert_eq!(cpu.l, 0b0101_0101);

    assert_eq!(cpu.pc, 2);
}

#[test]
fn rlc_hli() {
    let mut cpu = CpuBuilder::new()
        .with_flag_c(false)
        .with_hl(6)
        .with_memory(vec!(0xcb, 0x06, 0, 0, 0xca, 0xfe, 0xba, 0xbe))
        .build();

    // 0xba = 0b1011_1010
    cpu.rlc_hli();

    // S is set if result is negative; otherwise, it is reset.
    assert_eq!(cpu.get_s(), false);

    // Z is set if result is 0; otherwise, it is reset.
    assert_eq!(cpu.get_z(), false);
    
    // P/V is set if parity even; otherwise, it is reset.
    assert_eq!(cpu.get_pv(), false);

    // H, N are reset.
    assert_eq!(cpu.get_h(), false);
    assert_eq!(cpu.get_n(), false);

    // C is data from bit 7 of source register.
    assert_eq!(cpu.get_c(), true);
    
    assert_eq!(cpu.memory[6], 0b0111_0101);

    assert_eq!(cpu.pc, 2);
}

#[test]
fn rlc_ixdi() {
    let mut cpu = CpuBuilder::new()
        .with_flag_c(false)
        .with_ix(4)
        .with_memory(vec!(0xdd, 0xcb, 0x02, 0x06, 0xca, 0xfe, 0b1000_1000, 0xbe))
        .build();

    cpu.rlc_ixdi();

    // S is set if result is negative; otherwise, it is reset.
    assert_eq!(cpu.get_s(), false);

    // Z is set if result is 0; otherwise, it is reset.
    assert_eq!(cpu.get_z(), false);
    
    // P/V is set if parity even; otherwise, it is reset.
    assert_eq!(cpu.get_pv(), false);

    // H, N are reset.
    assert_eq!(cpu.get_h(), false);
    assert_eq!(cpu.get_n(), false);

    // C is data from bit 7 of source register.
    assert_eq!(cpu.get_c(), true);
    
    assert_eq!(cpu.memory[6], 0b0001_0001);

    assert_eq!(cpu.pc, 4);
}

#[test]
fn rlc_iydi() {
    let mut cpu = CpuBuilder::new()
        .with_flag_c(false)
        .with_iy(4)
        .with_memory(vec!(0xdd, 0xcb, 0x02, 0x06, 0xca, 0xfe, 0b1000_1000, 0xbe))
        .build();

    cpu.rlc_iydi();

    // S is set if result is negative; otherwise, it is reset.
    assert_eq!(cpu.get_s(), false);

    // Z is set if result is 0; otherwise, it is reset.
    assert_eq!(cpu.get_z(), false);
    
    // P/V is set if parity even; otherwise, it is reset.
    assert_eq!(cpu.get_pv(), false);

    // H, N are reset.
    assert_eq!(cpu.get_h(), false);
    assert_eq!(cpu.get_n(), false);

    // C is data from bit 7 of source register.
    assert_eq!(cpu.get_c(), true);
    
    assert_eq!(cpu.memory[6], 0b0001_0001);

    assert_eq!(cpu.pc, 4);
}

#[test]
fn rl_r() {
    unimplemented!();
}

#[test]
fn rl_hli() {
    unimplemented!();
}

#[test]
fn rl_ixdi() {
    unimplemented!();
}

#[test]
fn rl_iydi() {
    unimplemented!();
}

#[test]
fn rrc_r() {
    let mut cpu = CpuBuilder::new()
        .with_l(0b1010_1010)
        .with_memory(vec!(0xcb, 0b0000_1101, 0, 0))
        .build();

    cpu.rrc_r();

    // S is set if result is negative; otherwise, it is reset.
    assert_eq!(cpu.get_s(), false);

    // Z is set if result is 0; otherwise, it is reset.
    assert_eq!(cpu.get_z(), false);
    
    // P/V is set if parity even; otherwise, it is reset.
    assert_eq!(cpu.get_pv(), false);

    // H, N are reset.
    assert_eq!(cpu.get_h(), false);
    assert_eq!(cpu.get_n(), false);

    // C is data from bit 0 of source register.
    assert_eq!(cpu.get_c(), false);
    
    // The contents of register r are rotated left 1 bit position. The contents of bit 7 
    // are copied to the Carry flag and also to bit 0.
    assert_eq!(cpu.l, 0b0101_0101);

    assert_eq!(cpu.pc, 2);
}

#[test]
fn rrc_hli() {
    let mut cpu = CpuBuilder::new()
        .with_flag_c(false)
        .with_hl(6)
        .with_memory(vec!(0xcb, 0x0e, 0, 0, 0xca, 0xfe, 0xba, 0xbe))
        .build();

    // 0xba = 0b1011_1010
    cpu.rrc_hli();

    // S is set if result is negative; otherwise, it is reset.
    assert_eq!(cpu.get_s(), false);

    // Z is set if result is 0; otherwise, it is reset.
    assert_eq!(cpu.get_z(), false);
    
    // P/V is set if parity even; otherwise, it is reset.
    assert_eq!(cpu.get_pv(), false);

    // H, N are reset.
    assert_eq!(cpu.get_h(), false);
    assert_eq!(cpu.get_n(), false);

    // C is data from bit 0 of source register.
    assert_eq!(cpu.get_c(), false);
    
    assert_eq!(cpu.memory[6], 0b0101_1101);

    assert_eq!(cpu.pc, 2);
}

#[test]
fn rrc_ixdi() {
    let mut cpu = CpuBuilder::new()
        .with_flag_c(false)
        .with_ix(4)
        .with_memory(vec!(0xdd, 0xcb, 0x02, 0x06, 0xca, 0xfe, 0b1000_1000, 0xbe))
        .build();

    cpu.rrc_ixdi();

    // S is set if result is negative; otherwise, it is reset.
    assert_eq!(cpu.get_s(), false);

    // Z is set if result is 0; otherwise, it is reset.
    assert_eq!(cpu.get_z(), false);
    
    // P/V is set if parity even; otherwise, it is reset.
    assert_eq!(cpu.get_pv(), true);

    // H, N are reset.
    assert_eq!(cpu.get_h(), false);
    assert_eq!(cpu.get_n(), false);

    // C is data from bit 0 of source register.
    assert_eq!(cpu.get_c(), false);
    
    assert_eq!(cpu.memory[6], 0b0100_0100);

    assert_eq!(cpu.pc, 4);
}

#[test]
fn rrc_iydi() {
    let mut cpu = CpuBuilder::new()
        .with_flag_c(false)
        .with_iy(4)
        .with_memory(vec!(0xdd, 0xcb, 0x02, 0x06, 0xca, 0xfe, 0b1000_1000, 0xbe))
        .build();

    cpu.rrc_iydi();

    // S is set if result is negative; otherwise, it is reset.
    assert_eq!(cpu.get_s(), false);

    // Z is set if result is 0; otherwise, it is reset.
    assert_eq!(cpu.get_z(), false);
    
    // P/V is set if parity even; otherwise, it is reset.
    assert_eq!(cpu.get_pv(), true);

    // H, N are reset.
    assert_eq!(cpu.get_h(), false);
    assert_eq!(cpu.get_n(), false);

    // C is data from bit 0 of source register.
    assert_eq!(cpu.get_c(), false);
    
    assert_eq!(cpu.memory[6], 0b0100_0100);

    assert_eq!(cpu.pc, 4);
}

#[test]
fn rr_r() {
    unimplemented!();
}

#[test]
fn rr_hli() {
    unimplemented!();
}

#[test]
fn rr_ixdi() {
    unimplemented!();
}

#[test]
fn rr_iydi() {
    unimplemented!();
}

#[test]
fn sla_r() {
    unimplemented!();
}

#[test]
fn sla_hli() {
    unimplemented!();
}

#[test]
fn sla_ixdi() {
    unimplemented!();
}

#[test]
fn sla_iydi() {
    unimplemented!();
}

#[test]
fn sra_r() {
    unimplemented!();
}

#[test]
fn sra_hli() {
    unimplemented!();
}

#[test]
fn sra_ixdi() {
    unimplemented!();
}

#[test]
fn sra_iydi() {
    unimplemented!();
}

#[test]
fn srl_r() {
    unimplemented!();
}

#[test]
fn srl_hli() {
    unimplemented!();
}

#[test]
fn srl_ixdi() {
    unimplemented!();
}

#[test]
fn srl_iydi() {
    unimplemented!();
}

#[test]
fn rld() {
    unimplemented!();
}

#[test]
fn rrd() {
    unimplemented!();
}
