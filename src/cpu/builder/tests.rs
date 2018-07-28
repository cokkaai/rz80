use cpu::Assertor;
use cpu::CpuBuilder;

#[test]
fn create_system() {
    let builder = CpuBuilder::new()
        .with_a(1)
        .with_b(2)
        .with_c(3)
        .with_d(4)
        .with_e(5)
        .with_h(7)
        .with_i(8)
        .with_iff1(true)
        .with_iff2(false)
        .with_ix(9)
        .with_iy(10)
        .with_l(11)
        .with_memory_size(16384)
        .with_pc(12)
        .with_r(13)
        .with_sp(14)
        .with_flag_c(true)
        .with_flag_s(true)
        .with_flag_z(true)
        .with_flag_h(true)
        .with_flag_n(true)
        .with_flag_pv(true);

    let cpu = builder.build();

    Assertor::new(cpu)
        .register_a_is(1)
        .register_b_is(2)
        .register_c_is(3)
        .register_d_is(4)
        .register_e_is(5)
        .register_f_is(0b1101_0111)     // Position S Z X H X P/V N C
        .register_h_is(7)
        .register_l_is(11)
        .interrupt_flip_flop_1_is_set()
        .interrupt_flip_flop_2_is_reset()
        .interrupt_vector_is(8)
        .index_register_ix_is(9)
        .index_register_iy_is(10)
        .program_counter_is(12)
        .stack_pointer_is(14)
        .memory_refresh_register_is(13)
        .sign_flag_is_negative()
        .zero_flag_is_set()
        .half_carry_flag_is_set()
        .carry_flag_is_set()
        .parity_overflow_flag_is_set()
        .add_subtract_flag_is_set()
        .memory_size_is(16384);
}

#[test]
fn create_system_with_preset_memory() {
    let builder = CpuBuilder::new().with_memory(vec![1, 2, 3, 4]);

    let cpu = builder.build();

    Assertor::new(cpu)
        .memory_at_address_is(0, 1)
        .memory_at_address_is(1, 2)
        .memory_at_address_is(2, 3)
        .memory_at_address_is(3, 4);
}
