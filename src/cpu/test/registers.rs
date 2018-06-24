use cpu::Register;
use cpu::Register16;
use cpu::CPU;
use cpu::{C_MASK, H_MASK, N_MASK, PV_MASK, S_MASK, Z_MASK};

#[test]
fn get_s() {
    let mut cpu = CPU::new(16);

    // Initial value is 0
    assert_eq!(cpu.get_s(), false);
    assert!(cpu.sign_is_positive());

    // Value is true
    cpu.set_s(true);
    assert_eq!(cpu.get_s(), true);
    assert!(cpu.sign_is_negative());

    // All other flags are still false
    assert_eq!(cpu.f, S_MASK);

    // Set all flags and clear s
    cpu.f = 0xff;
    cpu.set_s(false);
    // Value is not true
    assert_eq!(cpu.get_s(), false);
    // All other flags are unchanged
    assert_eq!(cpu.f, !S_MASK);
}

#[test]
fn status_flag_z() {
    let mut cpu = CPU::new(16);

    assert_eq!(cpu.get_z(), false);

    cpu.set_z(true);
    assert_eq!(cpu.get_z(), true);
    assert_eq!(cpu.f, Z_MASK);

    cpu.f = 0xff;
    cpu.set_z(false);
    assert_eq!(cpu.get_z(), false);
    assert_eq!(cpu.f, !Z_MASK);
}

#[test]
fn status_flag_h() {
    let mut cpu = CPU::new(16);

    assert_eq!(cpu.get_h(), false);

    cpu.set_h(true);
    assert_eq!(cpu.get_h(), true);
    assert_eq!(cpu.f, H_MASK);

    cpu.f = 0xff;
    cpu.set_h(false);
    assert_eq!(cpu.get_h(), false);
    assert_eq!(cpu.f, !H_MASK);
}

#[test]
fn status_flag_pv() {
    let mut cpu = CPU::new(16);

    assert_eq!(cpu.get_pv(), false);

    cpu.set_pv(true);
    assert_eq!(cpu.get_pv(), true);
    assert_eq!(cpu.f, PV_MASK);

    cpu.f = 0xff;
    cpu.set_pv(false);
    assert_eq!(cpu.get_pv(), false);
    assert_eq!(cpu.f, !PV_MASK);
}

#[test]
fn status_flag_n() {
    let mut cpu = CPU::new(16);

    assert_eq!(cpu.get_n(), false);

    cpu.set_n(true);
    assert_eq!(cpu.get_n(), true);
    assert_eq!(cpu.f, N_MASK);

    cpu.f = 0xff;
    cpu.set_n(false);
    assert_eq!(cpu.get_n(), false);
    assert_eq!(cpu.f, !N_MASK);
}

#[test]
fn status_flag_c() {
    let mut cpu = CPU::new(16);

    assert_eq!(cpu.get_c(), false);

    cpu.set_c(true);
    assert_eq!(cpu.get_c(), true);
    assert_eq!(cpu.f, C_MASK);

    cpu.f = 0xff;
    cpu.set_c(false);
    assert_eq!(cpu.get_c(), false);
    assert_eq!(cpu.f, !C_MASK);
}

#[test]
fn select_src() {
    assert_eq!(CPU::select_src(0b00_000_111), Register::a);
    assert_eq!(CPU::select_src(0b00_000_000), Register::b);
    assert_eq!(CPU::select_src(0b00_000_001), Register::c);
    assert_eq!(CPU::select_src(0b00_000_010), Register::d);
    assert_eq!(CPU::select_src(0b00_000_011), Register::e);
    assert_eq!(CPU::select_src(0b00_000_100), Register::h);
    assert_eq!(CPU::select_src(0b00_000_101), Register::l);
}

#[test]
fn select_dest() {
    assert_eq!(CPU::select_dest(0b00_111_000), Register::a);
    assert_eq!(CPU::select_dest(0b00_000_000), Register::b);
    assert_eq!(CPU::select_dest(0b00_001_000), Register::c);
    assert_eq!(CPU::select_dest(0b00_010_000), Register::d);
    assert_eq!(CPU::select_dest(0b00_011_000), Register::e);
    assert_eq!(CPU::select_dest(0b00_100_000), Register::h);
    assert_eq!(CPU::select_dest(0b00_101_000), Register::l);
}

#[test]
fn read() {
    let mut cpu = CPU::new(16);

    cpu.a = 1;
    cpu.b = 2;
    cpu.c = 3;
    cpu.d = 4;
    cpu.e = 5;
    cpu.h = 6;
    cpu.l = 7;

    assert_eq!(cpu.read(Register::a), 1);
    assert_eq!(cpu.read(Register::b), 2);
    assert_eq!(cpu.read(Register::c), 3);
    assert_eq!(cpu.read(Register::d), 4);
    assert_eq!(cpu.read(Register::e), 5);
    assert_eq!(cpu.read(Register::h), 6);
    assert_eq!(cpu.read(Register::l), 7);
}

#[test]
fn write() {
    let mut cpu = CPU::new(16);

    cpu.write(Register::a, 1);

    assert_eq!(cpu.a, 1);
    assert_eq!(cpu.b, 0);
    assert_eq!(cpu.c, 0);
    assert_eq!(cpu.d, 0);
    assert_eq!(cpu.e, 0);
    assert_eq!(cpu.h, 0);
    assert_eq!(cpu.l, 0);

    cpu.write(Register::b, 2);
    cpu.write(Register::c, 3);
    cpu.write(Register::d, 4);
    cpu.write(Register::e, 5);
    cpu.write(Register::h, 6);
    cpu.write(Register::l, 7);

    assert_eq!(cpu.b, 2);
    assert_eq!(cpu.c, 3);
    assert_eq!(cpu.d, 4);
    assert_eq!(cpu.e, 5);
    assert_eq!(cpu.h, 6);
    assert_eq!(cpu.l, 7);
}

#[test]
fn read_hl() {
    let mut cpu = CPU::new(16);

    cpu.h = 0xab;
    cpu.l = 0xcd;

    assert_eq!(cpu.read16(Register16::hl), 0xabcd as u16);
}

#[test]
fn write_hl() {
    let mut cpu = CPU::new(16);

    cpu.write_hl(0xabcd);

    assert_eq!(cpu.h, 0xab);
    assert_eq!(cpu.l, 0xcd);
}

#[test]
fn read_bc() {
    let mut cpu = CPU::new(16);

    cpu.b = 0xab;
    cpu.c = 0xcd;

    assert_eq!(cpu.read16(Register16::bc), 0xabcd as u16);
}

#[test]
fn write_bc() {
    let mut cpu = CPU::new(16);

    cpu.write_bc(0xabcd);

    assert_eq!(cpu.b, 0xab);
    assert_eq!(cpu.c, 0xcd);
}

#[test]
fn read_de() {
    let mut cpu = CPU::new(16);

    cpu.d = 0xab;
    cpu.e = 0xcd;

    assert_eq!(cpu.read16(Register16::de), 0xabcd as u16);
}

#[test]
fn write_de() {
    let mut cpu = CPU::new(16);

    cpu.write16(Register16::de, 0xabcd);

    assert_eq!(cpu.d, 0xab);
    assert_eq!(cpu.e, 0xcd);
}
