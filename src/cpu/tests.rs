use cpu::CpuBuilder;
use cpu::Assertor;
use cpu::RegisterOperations;

#[test]
fn pc_incr() {
    let mut cpu = CpuBuilder::new()
        .with_memory_size(1024)
        .build();
    
    cpu.pc.reg_add(2);
    assert_eq!(cpu.pc, 2);
    
    cpu.pc.reg_add(0);
    assert_eq!(cpu.pc, 2);
    
    /*
    cpu.pc_incr(-1);
    assertor_eq!(cpu.pc, 1);
    
    cpu.pc_incr(1022);
    assertor_eq!(cpu.pc, 1023);
    
    cpu.pc_incr(-1000);
    assertor_eq!(cpu.pc, 1);
    */
}

#[test]
fn hl_addr() {
    // Check regular offset increment
    // Check overflowing increment
    // Check underflowing decrement
    unimplemented!();
}

#[test]
fn ix_addr() {
    // Check regular offset increment
    // Check overflowing increment
    // Check underflowing decrement
    unimplemented!();
}

#[test]
fn iy_addr() {
    // Check regular offset increment
    // Check overflowing increment
    // Check underflowing decrement
    unimplemented!();
}

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

