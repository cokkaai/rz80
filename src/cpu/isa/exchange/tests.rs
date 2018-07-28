// === Exchange, Block Transfer, and Search Group ===

use cpu::CpuBuilder;
use cpu::Assertor;

#[test]
fn ex_de_hl() {
    let mut cpu = CpuBuilder::new()
        .with_d(1)
        .with_e(2)
        .with_h(3)
        .with_l(4)
        .with_memory_size(16)
        .build();

    cpu.ex_de_hl();

    Assertor::new(cpu)
        .register_d_is(3)
        .register_e_is(4)
        .register_h_is(1)
        .register_l_is(2)
        .program_counter_is(1);
}

#[test]
fn ex_af_af1() {
    let mut cpu = CpuBuilder::new()
        .with_a(1)
        .with_flag_n(true)
        .with_memory_size(16)
        .build();


    cpu.ex_af_af1();

    cpu.a = 3;
    cpu.f = 4;

    cpu.ex_af_af1();

    Assertor::new(cpu)
        .register_a_is(1)
        .register_f_is(2)
        .register_a1_is(3)
        .register_f1_is(4)
        .program_counter_is(2);
}

#[test]
fn exx() {
    let mut cpu = CpuBuilder::new()
        .with_b(1)
        .with_c(2)
        .with_d(3)
        .with_e(4)
        .with_h(5)
        .with_l(6)
        .with_memory_size(16)
        .build();

    cpu.exx();

    cpu.b = 11;
    cpu.c = 12;
    cpu.d = 13;
    cpu.e = 14;
    cpu.h = 15;
    cpu.l = 16;

    cpu.exx();

    Assertor::new(cpu)
        .register_b_is(1)
        .register_c_is(2)
        .register_d_is(3)
        .register_e_is(4)
        .register_h_is(5)
        .register_l_is(6)
        .register_b1_is(11)
        .register_c1_is(12)
        .register_d1_is(13)
        .register_e1_is(14)
        .register_h1_is(15)
        .register_l1_is(16)
        .program_counter_is(2);
}

#[test]
fn ex_spi_hl() {
    let mut cpu = CpuBuilder::new()
        .with_h(9)
        .with_l(8)
        .with_memory(vec![1, 2, 3, 4])
        .build();

    cpu.ex_spi_hl();

    Assertor::new(cpu)
        // H ↔ (SP+1), L ↔ (SP)
        .register_h_is(2)
        .memory_at_address_is(1, 9)
        // H ↔ (SP+1), L ↔ (SP)
        .register_l_is(1)
        .memory_at_address_is(0, 8)
        .program_counter_is(1);
}

#[test]
fn ex_spi_ix() {
    let mut cpu = CpuBuilder::new()
        .with_ix(0x3988)
        .with_l(8)
        .with_memory(vec![0x90, 0x48, 3, 4])
        .build();

    cpu.ex_spi_ix();

    // IXH ↔ (SP+1) IXL ↔ (SP)
    Assertor::new(cpu)
        .index_register_ix_is(0x4890)
        .memory_at_address_is(1, 0x39)
        .memory_at_address_is(0, 0x88)
        .program_counter_is(2);
}

#[test]
fn ex_spi_iy() {
    let mut cpu = CpuBuilder::new()
        .with_iy(0x3988)
        .with_l(8)
        .with_memory(vec![0x90, 0x48, 3, 4])
        .build();

    cpu.ex_spi_iy();

    // IYH ↔ (SP+1) IYL ↔ (SP)
    Assertor::new(cpu)
        .index_register_iy_is(0x4890)
        .memory_at_address_is(1, 0x39)
        .memory_at_address_is(0, 0x88)
        .program_counter_is(2);
}

#[test]
fn ldi() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![0x90, 0x48, 3, 4])
        .with_bc(3)
        .with_de(0)
        .with_hl(1)
        .build();

    cpu.ldi();

    Assertor::new(cpu)
        .memory_at_address_is(0, 0x48)      // (DE) ← (HL)
        .register_de_is(1)              // DE ← DE + 1
        .register_hl_is(2)              // HL ← HL + 1
        .register_bc_is(2)              // BC ← BC – 1
        .parity_overflow_flag_is_set()  // P/V is set if BC – 1 ≠ 0
        .program_counter_is(2);
}

#[test]
fn ldir() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![0x90, 0x91, 0x92, 0x93, 0x44, 0x45, 0x46, 0x47, 3, 4])
        .with_bc(3)
        .with_de(0)
        .with_hl(4)
        .build();

    cpu.ldir();

    Assertor::new(cpu)
        .memory_at_address_is(0, 0x44)      // (DE) ← (HL)
        .memory_at_address_is(1, 0x45)
        .memory_at_address_is(2, 0x46)
        .memory_at_address_is(3, 0x93)
        .register_de_is(3)              // DE ← DE + 1
        .register_hl_is(7)              // HL ← HL + 1
        .register_bc_is(0)              // BC ← BC – 1
        .parity_overflow_flag_is_reset()  // P/V is set if BC – 1 ≠ 0
        .program_counter_is(2);
}

#[test]
fn ldd() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![0x90, 0x48, 3, 4])
        .with_bc(3)
        .with_de(3)
        .with_hl(1)
        .build();

    cpu.ldd();

    Assertor::new(cpu)
        .memory_at_address_is(0, 0x90)      // (DE) ← (HL)
        .memory_at_address_is(1, 0x48)
        .memory_at_address_is(2, 0x3)
        .memory_at_address_is(3, 0x48)
        .register_de_is(2)              // DE ← DE + 1
        .register_hl_is(0)              // HL ← HL + 1
        .register_bc_is(2)              // BC ← BC – 1
        .parity_overflow_flag_is_set()  // P/V is set if BC – 1 ≠ 0
        .program_counter_is(2);
}

#[test]
fn lddr() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![0x90, 0x91, 0x92, 0x93, 0x44, 0x45, 0x46, 0x47, 3, 4])
        .with_bc(3)
        .with_de(3)
        .with_hl(7)
        .build();

    cpu.lddr();

    Assertor::new(cpu)
        .memory_at_address_is(0, 0x90)      // (DE) ← (HL)
        .memory_at_address_is(1, 0x45)
        .memory_at_address_is(2, 0x46)
        .memory_at_address_is(3, 0x47)
        .register_de_is(0)              // DE ← DE - 1
        .register_hl_is(4)              // HL ← HL - 1
        .register_bc_is(0)              // BC ← BC - 1
        .parity_overflow_flag_is_reset()  // P/V is set if BC – 1 ≠ 0
        .program_counter_is(2);
}

#[test]
fn cpi() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![0x90, 0x91, 0x92, 0x93, 0x3b, 0x45, 0x46, 0x47, 3, 4])
        .with_a(0x3b)
        .with_bc(1)
        .with_hl(4)
        .build();

    cpu.cpi();

    Assertor::new(cpu)
        .register_bc_is(0)          // BC ← BC-1
        .register_hl_is(5)          // HL ← HL+1
        .zero_flag_is_set()         // Z is set if A is (HL)
        .parity_overflow_flag_is_reset()  // P/V is set if BC-1 != 0
        .sign_flag_is_positive()
        .carry_flag_is_reset()
        .half_carry_flag_is_reset()
        .add_subtract_flag_is_set()
        .program_counter_is(2);
}

#[test]
fn cpir() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![0x90, 0x91, 0x92, 0x93, 0x3b, 0x45, 0x46, 0x47, 3, 4])
        .with_a(0x3b)
        .with_bc(2)
        .with_hl(4)
        .build();

    cpu.cpir();

    Assertor::new(cpu)
        .register_bc_is(0)  // BC ← BC-1
        .register_hl_is(6)  // HL ← HL+1
        .zero_flag_is_reset()// Z is set if A is (HL)
        .parity_overflow_flag_is_set()  // P/V is set if BC-1 != 0
        .sign_flag_is_positive()    // S is set if result is negative
        .carry_flag_is_reset()// C is not affected
        .half_carry_flag_is_reset()// H is set if borrow from bit 4
        .add_subtract_flag_is_set()
        .program_counter_is(2);
}

#[test]
fn cpd() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![0x90, 0x91, 0x92, 0x93, 0x3b, 0x45, 0x46, 0x47, 3, 4])
        .with_a(0x3b)
        .with_bc(1)
        .with_hl(4)
        .build();

    cpu.cpd();

    Assertor::new(cpu)
        .register_bc_is(0)  // BC ← BC-1
        .register_hl_is(3)  // HL ← HL-1
        .program_counter_is(2);
}

#[test]
fn cpdr() {
    let mut cpu = CpuBuilder::new()
        .with_memory(vec![0x90, 0x91, 0x92, 0x93, 0x3b, 0x45, 0x46, 0x47, 3, 4])
        .with_a(0x3b)
        .with_bc(2)
        .with_hl(4)
        .build();

    cpu.cpdr();

    Assertor::new(cpu)
        .register_bc_is(0)  // BC ← BC-1
        .register_hl_is(2)  // HL ← HL-1
        .program_counter_is(2);
}
