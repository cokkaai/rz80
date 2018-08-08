// === Rotate and Shift Group ===

use cpu::CpuBuilder;
use cpu::Assertor;

#[test]
fn rlca() {
    let mut cpu = CpuBuilder::new()
        .with_a(0b1010_1010)
        .with_memory_size(16)
        .build();

    cpu.rlca();

    Assertor::new(cpu)
        .register_a_is(0b0101_0101)

        // S, Z, P/V are not affected.
        .sign_is_positive()
        .zero_flag_is_reset()
        .parity_overflow_flag_is_reset()
        
        // H, N are reset.
        .half_carry_flag_is_reset()
        .add_subtract_flag_is_reset()
        
        // C is data from bit 7 of Accumulator.
        .carry_flag_is_set()
   
        // The contents of the Accumulator (Register A) are rotated left 1 bit position.
        .register_a_is(0b0101_0101)
    
        .program_counter_is(1);
}

#[test]
fn rla() {
    let mut cpu = CpuBuilder::new()
        .with_a(0b0111_0110)
        .with_flag_c(true)
        .with_memory_size(16)
        .build();

    cpu.rla();

    Assertor::new(cpu)
    
        // S, Z, P/V are not affected.
        .sign_is_positive()
        .zero_flag_is_reset()
        .parity_overflow_flag_is_reset()
        
        // H, N are reset.
        .half_carry_flag_is_reset()
        .add_subtract_flag_is_reset()
        
        // C is data from bit 7 of Accumulator.
        .carry_flag_is_reset()

        // The contents of the Accumulator (Register A) are rotated left 1 bit position.
        .register_a_is(0b1110_1101)

        .program_counter_is(1);
}

#[test]
fn rrca() {
    let mut cpu = CpuBuilder::new()
        .with_a(0b0001_0001)
        .with_flag_c(false)
        .with_memory_size(16)
        .build();

    cpu.rrca();

    Assertor::new(cpu)
        .register_a_is(0b1000_1000)
    
        // S, Z, P/V are not affected.
        .sign_is_positive()
        .zero_flag_is_reset()
        .parity_overflow_flag_is_reset()
        
        // H, N are reset.
        .half_carry_flag_is_reset()
        .add_subtract_flag_is_reset()
        
        // C is data from bit 0 of Accumulator.
        .carry_flag_is_set()
    
        // The contents of the Accumulator (Register A) are rotated right 1 bit position.
        .register_a_is(0b1000_1000)
        
        .program_counter_is(1);
}

#[test]
fn rra() {
    let mut cpu = CpuBuilder::new()
        .with_a(0b0001_0001)
        .with_flag_c(false)
        .with_memory_size(16)
        .build();

    cpu.rra();

    Assertor::new(cpu)
        .register_a_is(0b0000_1000)

        // S, Z, P/V are not affected.
        .sign_is_positive()
        .zero_flag_is_reset()
        .parity_overflow_flag_is_reset()
        
        // H, N are reset.
        .half_carry_flag_is_reset()
        .add_subtract_flag_is_reset()
        
        // C is data from bit 0 of Accumulator.
        .carry_flag_is_set()
    
        // The contents of the Accumulator (Register A) are rotated right 1 bit
        // position through the Carry flag. The previous contents of the Carry
        // flag are copied to bit 7.
        .register_a_is(0b0000_1000)
        
        .program_counter_is(1);
}

#[test]
fn rlc_r() {
    let mut cpu = CpuBuilder::new()
        .with_l(0b1010_1010)
        .with_memory(vec!(0xcb, 0b0000_0101, 0, 0))
        .build();

    cpu.rlc_r();

    Assertor::new(cpu)
        .sign_is_positive()
        .zero_flag_is_reset()
        .parity_overflow_flag_is_reset()
        .half_carry_flag_is_reset()
        .add_subtract_flag_is_reset()
        .carry_flag_is_set()
    
        // The contents of register r are rotated left 1 bit position. The contents of bit 7 
        // are copied to the Carry flag and also to bit 0.
        .register_l_is(0b0101_0101)

        .program_counter_is(2);
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

    Assertor::new(cpu)
        .memory_at_address_is(0x06, 0b0111_0101)
        .sign_is_positive()
        .zero_flag_is_reset()

        // P/V is set if parity even; otherwise, it is reset.
        .parity_overflow_flag_is_reset()

        .half_carry_flag_is_reset()
        .add_subtract_flag_is_reset()
        .carry_flag_is_set()
        .program_counter_is(2);
}

#[test]
fn rlc_ixdi() {
    let mut cpu = CpuBuilder::new()
        .with_flag_c(false)
        .with_ix(4)
        .with_memory(vec!(0xdd, 0xcb, 0x02, 0x06, 0xca, 0xfe, 0b1000_1000, 0xbe))
        .build();

    cpu.rlc_ixdi();

    Assertor::new(cpu)
        .sign_is_positive()
        .zero_flag_is_reset()

        // P/V is set if parity even; otherwise, it is reset.
        .parity_overflow_flag_is_reset()

        .half_carry_flag_is_reset()
        .add_subtract_flag_is_reset()
        .carry_flag_is_set()
        .memory_at_address_is(0x06, 0b0001_0001)
        .program_counter_is(4);
}

#[test]
fn rlc_iydi() {
    let mut cpu = CpuBuilder::new()
        .with_flag_c(false)
        .with_iy(4)
        .with_memory(vec!(0xdd, 0xcb, 0x02, 0x06, 0xca, 0xfe, 0b1000_1000, 0xbe))
        .build();

    cpu.rlc_iydi();

    Assertor::new(cpu)
        .sign_is_positive()
        .zero_flag_is_reset()

        // P/V is set if parity even; otherwise, it is reset.
        .parity_overflow_flag_is_reset()

        .half_carry_flag_is_reset()
        .add_subtract_flag_is_reset()
        .carry_flag_is_set()
        .memory_at_address_is(0x06, 0b0001_0001)
        .program_counter_is(4);
}

#[test]
fn rl_r() {
    let mut cpu = CpuBuilder::new()
        .with_flag_c(false)
        .with_a(0b1101_1101)
        .with_memory(vec!(0xcb, 0b0000_1111, 0xba, 0xbe))
        .build();

    cpu.rl_r();

    Assertor::new(cpu)
        .register_a_is(0b1011_1010)
        .sign_is_negative()
        .zero_flag_is_reset()
        .parity_overflow_flag_is_set()
        .half_carry_flag_is_reset()
        .add_subtract_flag_is_reset()
        .carry_flag_is_set()
        .program_counter_is(2);
}

#[test]
fn rl_hli() {
    let mut cpu = CpuBuilder::new()
        .with_flag_c(false)
        .with_hl(0x06)
        .with_memory(vec!(0xcb, 0x16, 0xba, 0xbe, 0x00, 0x00, 0b1101_1101))
        .build();

    cpu.rl_hli();

    Assertor::new(cpu)
        .memory_at_address_is(0x06, 0b1011_1010)
        .sign_is_negative()
        .zero_flag_is_reset()
        .parity_overflow_flag_is_set()
        .half_carry_flag_is_reset()
        .add_subtract_flag_is_reset()
        .carry_flag_is_set()
        .program_counter_is(2);
}

#[test]
fn rl_ixdi() {
    let mut cpu = CpuBuilder::new()
        .with_flag_c(false)
        .with_ix(0x05)
        .with_memory(vec!(0xdd, 0xcb, 0x01, 0x16, 0x00, 0x00, 0b1101_1101))
        .build();

    cpu.rl_ixdi();

    Assertor::new(cpu)
        .memory_at_address_is(0x06, 0b1011_1010)
        .sign_is_negative()
        .zero_flag_is_reset()
        .parity_overflow_flag_is_set()
        .half_carry_flag_is_reset()
        .add_subtract_flag_is_reset()
        .carry_flag_is_set()
        .program_counter_is(4);
}

#[test]
fn rl_iydi() {
    let mut cpu = CpuBuilder::new()
        .with_flag_c(false)
        .with_iy(0x05)
        .with_memory(vec!(0xfd, 0xcb, 0x01, 0x16, 0x00, 0x00, 0b1101_1101))
        .build();

    cpu.rl_iydi();

    Assertor::new(cpu)
        .memory_at_address_is(0x06, 0b1011_1010)
        .sign_is_negative()
        .zero_flag_is_reset()
        .parity_overflow_flag_is_set()
        .half_carry_flag_is_reset()
        .add_subtract_flag_is_reset()
        .carry_flag_is_set()
        .program_counter_is(4);
}

#[test]
fn rrc_r() {
    let mut cpu = CpuBuilder::new()
        .with_l(0b1010_1010)
        .with_memory(vec!(0xcb, 0b0000_1101, 0, 0))
        .build();

    cpu.rrc_r();

    Assertor::new(cpu)
        .sign_is_positive()
        .zero_flag_is_reset()
        .parity_overflow_flag_is_reset()
        .half_carry_flag_is_reset()
        .add_subtract_flag_is_reset()
        .carry_flag_is_reset()
        .register_l_is(0b0101_0101)
        .program_counter_is(2);
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

    Assertor::new(cpu)
        .sign_is_positive()
        .zero_flag_is_reset()
        .parity_overflow_flag_is_reset()
        .half_carry_flag_is_reset()
        .add_subtract_flag_is_reset()
        .carry_flag_is_reset()
        .memory_at_address_is(0x06, 0b0101_1101)
        .program_counter_is(2);
}

#[test]
fn rrc_ixdi() {
    let mut cpu = CpuBuilder::new()
        .with_flag_c(false)
        .with_ix(4)
        .with_memory(vec!(0xdd, 0xcb, 0x02, 0x06, 0xca, 0xfe, 0b1000_1000, 0xbe))
        .build();

    cpu.rrc_ixdi();

    Assertor::new(cpu)
        .sign_is_positive()
        .zero_flag_is_reset()
        .parity_overflow_flag_is_set()
        .half_carry_flag_is_reset()
        .add_subtract_flag_is_reset()
        .carry_flag_is_reset()
        .memory_at_address_is(0x06, 0b0100_0100)
        .program_counter_is(4);
}

#[test]
fn rrc_iydi() {
    let mut cpu = CpuBuilder::new()
        .with_flag_c(false)
        .with_iy(4)
        .with_memory(vec!(0xdd, 0xcb, 0x02, 0x06, 0xca, 0xfe, 0b1000_1000, 0xbe))
        .build();

    cpu.rrc_iydi();

    Assertor::new(cpu)
        .sign_is_positive()
        .zero_flag_is_reset()
        .parity_overflow_flag_is_set()
        .half_carry_flag_is_reset()
        .add_subtract_flag_is_reset()
        .carry_flag_is_reset()
        .memory_at_address_is(0x06, 0b0100_0100)
        .program_counter_is(4);
}

#[test]
fn rr_r() {
    let mut cpu = CpuBuilder::new()
        .with_flag_c(false)
        .with_a(0b1101_1101)
        .with_memory(vec!(0xcb, 0b0000_1111, 0xba, 0xbe))
        .build();

    cpu.rr_r();

    Assertor::new(cpu)
        .register_a_is(0b0110_1110)
        .sign_is_positive()
        .zero_flag_is_reset()
        .parity_overflow_flag_is_set()
        .half_carry_flag_is_reset()
        .add_subtract_flag_is_reset()
        .carry_flag_is_set()
        .program_counter_is(2);
}

#[test]
fn rr_hli() {
    let mut cpu = CpuBuilder::new()
        .with_flag_c(false)
        .with_hl(0x03)
        .with_memory(vec!(0xcb, 0x1e, 0xba, 0b1101_1101))
        .build();

    cpu.rr_hli();

    Assertor::new(cpu)
        .memory_at_address_is(0x03, 0b0110_1110)
        .sign_is_positive()
        .zero_flag_is_reset()
        .parity_overflow_flag_is_set()
        .half_carry_flag_is_reset()
        .add_subtract_flag_is_reset()
        .carry_flag_is_set()
        .program_counter_is(2);
}

#[test]
fn rr_ixdi() {
    let mut cpu = CpuBuilder::new()
        .with_flag_c(false)
        .with_ix(0x06)
        .with_memory(vec!(0xdd, 0xcb, 0x01, 0x1e, 0xba, 0xbe, 0x00, 0b1101_1101))
        .build();

    cpu.rr_ixdi();

    Assertor::new(cpu)
        .memory_at_address_is(0x07, 0b0110_1110)
        .sign_is_positive()
        .zero_flag_is_reset()
        .parity_overflow_flag_is_set()
        .half_carry_flag_is_reset()
        .add_subtract_flag_is_reset()
        .carry_flag_is_set()
        .program_counter_is(4);
}

#[test]
fn rr_iydi() {
    let mut cpu = CpuBuilder::new()
        .with_flag_c(false)
        .with_iy(0x06)
        .with_memory(vec!(0xfd, 0xcb, 0x01, 0x1e, 0xba, 0xbe, 0x00, 0b1101_1101))
        .build();

    cpu.rr_iydi();

    Assertor::new(cpu)
        .memory_at_address_is(0x07, 0b0110_1110)
        .sign_is_positive()
        .zero_flag_is_reset()
        .parity_overflow_flag_is_set()
        .half_carry_flag_is_reset()
        .add_subtract_flag_is_reset()
        .carry_flag_is_set()
        .program_counter_is(4);
}

#[test]
fn rld() {
    let mut cpu = CpuBuilder::new()
        .with_flag_c(false)
        .with_iy(0x06)
        .with_memory(vec!(0xed, 0x6f))
        .build();

    cpu.rld();

    // Assertor::new(cpu)
    //     .memory_at_address_is(0x07, 0b0110_1110)
    //     .sign_is_positive()
    //     .zero_flag_is_reset()
    //     .parity_overflow_flag_is_set()
    //     .half_carry_flag_is_reset()
    //     .add_subtract_flag_is_reset()
    //     .carry_flag_is_set()
    //     .program_counter_is(4);
}

#[test]
fn rrd() {
    let mut cpu = CpuBuilder::new()
        .with_flag_c(false)
        .with_iy(0x06)
        .with_memory(vec!(0xfd, 0xcb, 0x01, 0x1e, 0xba, 0xbe, 0x00, 0b1101_1101))
        .build();

    cpu.rrd();

    // Assertor::new(cpu)
    //     .memory_at_address_is(0x07, 0b0110_1110)
    //     .sign_is_positive()
    //     .zero_flag_is_reset()
    //     .parity_overflow_flag_is_set()
    //     .half_carry_flag_is_reset()
    //     .add_subtract_flag_is_reset()
    //     .carry_flag_is_set()
    //     .program_counter_is(4);
}
