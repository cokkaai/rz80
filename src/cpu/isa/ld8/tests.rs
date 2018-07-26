// === 8-Bit Load Group ===

use cpu::Cpu;
use cpu::CpuBuilder;
use cpu::Assertor;

#[test]
fn memory_at_pc() {
    let cpu = CpuBuilder::new()
        .with_memory(vec![
            0x01, 0x02, 0xa4, 0xa8, 0x0f, 0x11, 0x12, 0x14, 0x18, 0x1f, 0x22, 0x33, 0x44, 0x55,
            0x66, 0x77,
        ])
        .build();

    Assertor::new(cpu)
        .memory_at_address_is(0, 0x01)
        .memory_at_address_is(1, 0x02)
        .memory_at_address_is(9, 0x1f)
        .memory_at_address_is(15, 0x77);
}

#[test]
fn twocmp() {
    assert_eq!(Cpu::compl2(0), 0);
    assert_eq!(Cpu::compl2(0b0000_0001), 0b1111_1111);
    assert_eq!(Cpu::compl2(0b0000_0010), 0b1111_1110);
    assert_eq!(Cpu::compl2(0b0111_1110), 0b1000_0010);
    assert_eq!(Cpu::compl2(0b0111_1111), 0b1000_0001);
    assert_eq!(Cpu::compl2(0b1111_1111), 0b0000_0001);
}

#[test]
fn incr_pc() {
    let mut cpu = CpuBuilder::new().with_memory_size(16).build();
    cpu.incr_pc(2);
    Assertor::new(cpu).program_counter_is(2);
}

#[test]
fn ld_r_r1() {
    // Set initial cpu status and memory
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![
            0b01_100_011, // LD H,E
            0b01_001_010, // LD C,D
            0,
            0,
        ])
        .with_d(19)
        .with_e(26)
        .build();

    // Load registers while pc = 0
    cpu.ld_r_r1();

    Assertor::new(cpu)
        .register_a_is(0)
        .register_b_is(0)
        .register_c_is(0)
        .register_d_is(19)
        .register_e_is(26)
        .register_f_is(0)
        .register_h_is(26)
        .register_l_is(0)
        .interrupt_vector_is(0)
        .program_counter_is(1);
}

#[test]
fn ld_r_n() {
    // Set initial cpu status and memory
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![
            0b00_100_110, // LD H, 0xFA
            0xfa,
            0b00_000_110, // LD B, 0xFB
            0xfb,
            0,
            0,
            0,
            0,
        ])
        .build();

    // Load registers while pc = 0
    cpu.ld_r_n();

    Assertor::new(cpu)
        .register_a_is(0)
        .register_b_is(0)
        .register_c_is(0)
        .register_d_is(0)
        .register_e_is(0)
        .register_f_is(0)
        .register_h_is(0xfa)
        .register_l_is(0)
        .interrupt_vector_is(0)
        .program_counter_is(2);

}

#[test]
fn ld_r_hl() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![
            0b01_000_110, // LD B, (HL)
            0x01,
            0xfb,
            0,
        ])
        .with_hl(2)
        .build();

    cpu.ld_r_hl();

    Assertor::new(cpu)
        .register_b_is(0xfb)
        .program_counter_is(1);
}

#[test]
fn ld_r_ixd() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![0xdd, 0b01_000_110, 0b1111_1111, 0, 0, 0xfb, 0, 0])
        .with_ix(4)
        .build();

    cpu.ld_r_ixd();

    Assertor::new(cpu)
        .register_b_is(0xfb)
        .program_counter_is(3);
}

#[test]
fn ld_r_iyd() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![0xfd, 0b01_000_110, 0b1111_1111, 0, 0, 0xfb, 0, 0])
        .with_iy(4)
        .build();

    cpu.ld_r_iyd();

    Assertor::new(cpu)
        .register_b_is(0xfb)
        .program_counter_is(3);
}

#[test]
fn ld_hl_r() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![
            0b01110_010, // LD (HL), D
            0,
            0,
            0,
        ])
        .with_d(0xfa)
        .with_hl(2)
        .build();

    cpu.ld_hl_r();

    Assertor::new(cpu)
        .memory_at_address_is(2, 0xfa)
        .program_counter_is(1);
}

#[test]
fn ld_ixd_r() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![
            0xdd, // LD (IX+D), R
            0b01110_111,
            0xff, // D
            1,
            2,
            3,
            4,
            5,
        ])
        .with_ix(4)
        .with_a(0xfb)
        .build();

    cpu.ld_ixd_r();
    Assertor::new(cpu)
        .memory_at_address_is(5, 0xfb)
        .program_counter_is(3);
}

#[test]
fn ld_iyd_r() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![
            0xfd, // LD (IY+D), R
            0b01110_111,
            0xff, // D
            1,
            2,
            3,
            4,
            5,
        ])
        .with_iy(4)
        .with_a(0xfb)
        .build();

    cpu.ld_iyd_r();

    Assertor::new(cpu)
        .memory_at_address_is(5, 0xfb)
        .program_counter_is(3);
}

#[test]
fn ld_hl_n() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![
            0x36, // LD (HL), N
            0xfa, 0, 0,
        ])
        .with_hl(3)
        .build();

    cpu.ld_hl_n();

    Assertor::new(cpu)
        .memory_at_address_is(3, 0xfa)
        .program_counter_is(2);
}

#[test]
fn ld_ixd_n() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![
            0xdd, // LD (IX+D), N
            0x36, 0x02, // D
            0xfb, // N
            1, 2, 3, 4,
        ])
        .with_ix(4)
        .build();

    cpu.ld_ixd_n();

    Assertor::new(cpu)
        .memory_at_address_is(6, 0xfb)
        .program_counter_is(4);
}

#[test]
fn ld_iyd_n() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![
            0xfd, // LD (IY+D), N
            0x36, 0x02, // D
            0xfb, // N
            1, 2, 3, 4,
        ])
        .with_iy(4)
        .build();

    cpu.ld_iyd_n();

    Assertor::new(cpu)
        .memory_at_address_is(6, 0xfb)
        .program_counter_is(4);
}

#[test]
fn ld_a_bc() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![
            0x0a, // LD A, (BC)
            0x01, 0xf2, 0x03,
        ])
        .with_bc(2)
        .build();

    cpu.ld_a_bc();

    Assertor::new(cpu)
        .register_a_is(0xf2)
        .program_counter_is(1);
}

#[test]
fn ld_a_de() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![
            0x1a, // LD A, (DE)
            0x01, 0xf2, 0x03,
        ])
        .with_de(2)
        .build();

    cpu.ld_a_de();

    Assertor::new(cpu)
        .register_a_is(0xf2)
        .program_counter_is(1);
}

#[test]
fn ld_a_nn() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![
            0x3a, // LD A, NN
            0x03, 0, 0x13,
        ])
        .build();

    cpu.ld_a_nn();

    Assertor::new(cpu)
        .register_a_is(0x13)
        .program_counter_is(3);
}

#[test]
fn ld_a_i() {
    let mut cpu = CpuBuilder::new()
        .with_memory_size(16)
        .with_i(0x3b)
        .with_iff2(false)
        .with_flag_c(true)
        .build();

    cpu.ld_a_i();

    Assertor::new(cpu)
        .register_a_is(0x3b)
        .sign_flag_is_positive()
        .half_carry_flag_is_reset()
        .add_subtract_flag_is_reset()
        .carry_flag_is_set()
        .zero_flag_is_reset()
        .parity_overflow_flag_is_reset()
        .program_counter_is(2);
}

#[test]
fn ld_a_r() {
    let mut cpu = CpuBuilder::new()
        .with_memory_size(16)
        .with_r(0x3b)
        .with_iff2(false)
        .with_flag_c(true)
        .build();

    cpu.ld_a_r();

    Assertor::new(cpu)
        .register_a_is(0x3b)
        .sign_flag_is_positive()
        .half_carry_flag_is_reset()
        .add_subtract_flag_is_reset()
        .carry_flag_is_set()
        .zero_flag_is_reset()
        .parity_overflow_flag_is_reset()
        .program_counter_is(2);
}

#[test]
fn ld_i_a() {
    let mut cpu = CpuBuilder::new().with_memory_size(16).with_a(12).build();

    cpu.ld_i_a();

    Assertor::new(cpu)
        .interrupt_vector_is(12)
        .program_counter_is(2);
}

#[test]
fn ld_r_a() {
    let mut cpu = CpuBuilder::new().with_memory_size(16).with_a(12).build();

    cpu.ld_r_a();

    Assertor::new(cpu)
        .memory_refresh_register_is(12)
        .program_counter_is(2);
}
