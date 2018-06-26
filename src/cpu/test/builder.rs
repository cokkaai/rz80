use cpu::CpuBuilder;

#[test]
fn create_system() {
    let builder = CpuBuilder::new()
        .with_a(1)
        .with_b(2)
        .with_c(3)
        .with_d(4)
        .with_e(5)
        .with_f(6)
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
        .with_sp(14);
    
    let cpu = builder.build();

    assert_eq!(cpu.a, 1);
    assert_eq!(cpu.b, 2);
    assert_eq!(cpu.c, 3);
    assert_eq!(cpu.d, 4);
    assert_eq!(cpu.e, 5);
    assert_eq!(cpu.f, 6);
    assert_eq!(cpu.h, 7);
    assert_eq!(cpu.i, 8);
    assert_eq!(cpu.iff1, true);
    assert_eq!(cpu.iff2, false);
    assert_eq!(cpu.ix, 9);
    assert_eq!(cpu.iy, 10);
    assert_eq!(cpu.l, 11);
    assert_eq!(cpu.memory.capacity(), 16384);
    assert_eq!(cpu.pc, 12);
    assert_eq!(cpu.r, 13);
    assert_eq!(cpu.sp, 14);
}

#[test]
fn create_system_with_preset_memory() {
    let builder = CpuBuilder::new()
        .with_memory(vec![1, 2, 3, 4]);
    
    let cpu = builder.build();

    assert_eq!(cpu.memory[0], 1);
    assert_eq!(cpu.memory[1], 2);
    assert_eq!(cpu.memory[2], 3);
    assert_eq!(cpu.memory[3], 4);
}