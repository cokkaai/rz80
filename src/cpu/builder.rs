use cpu::CPU;
use cpu::RegisterDemote;

#[derive(Debug)]
pub struct CpuBuilder {
    pub pc: u16,
    pub sp: u16,
    pub ix: u16,
    pub iy: u16,

    pub i: u8,
    pub r: u8,

    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,

    pub a1: u8,
    pub b1: u8,
    pub c1: u8,
    pub d1: u8,
    pub e1: u8,
    pub f1: u8,
    pub h1: u8,
    pub l1: u8,

    pub iff1: bool,
    pub iff2: bool,

    pub flag_s: bool,
    pub flag_z: bool,
    pub flag_c: bool,
    pub flag_h: bool,
    pub flag_n: bool,
    pub flag_pv: bool,

    pub memory: Option<Vec<u8>>,

}

#[allow(dead_code)]
impl CpuBuilder {
    pub fn new() -> CpuBuilder {
        return CpuBuilder {
            pc: 0,
            sp: 0,
            ix: 0,
            iy: 0,
            i: 0,
            r: 0,
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
            h: 0,
            l: 0,
            a1: 0,
            b1: 0,
            c1: 0,
            d1: 0,
            e1: 0,
            f1: 0,
            h1: 0,
            l1: 0,
            iff1: false,
            iff2: false,
            flag_s: false,
            flag_z: false,
            flag_c: false,
            flag_h: false,
            flag_n: false,
            flag_pv: false,
            memory: None,
        }
    }

    pub fn with_pc(mut self, value: u16) -> CpuBuilder {
        self.pc = value;
        self
    }

    pub fn with_sp(mut self, value: u16) -> CpuBuilder {
        self.sp = value;
        self
    }

    pub fn with_ix(mut self, value: u16) -> CpuBuilder {
        self.ix = value;
        self
    }

    pub fn with_iy(mut self, value: u16) -> CpuBuilder {
        self.iy = value;
        self
    }

    pub fn with_i(mut self, value: u8) -> CpuBuilder {
        self.i = value;
        self
    }

    pub fn with_r(mut self, value: u8) -> CpuBuilder {
        self.r = value;
        self
    }

    pub fn with_a(mut self, value: u8) -> CpuBuilder {
        self.a = value;
        self
    }

    pub fn with_b(mut self, value: u8) -> CpuBuilder {
        self.b = value;
        self
    }

    pub fn with_c(mut self, value: u8) -> CpuBuilder {
        self.c = value;
        self
    }

    pub fn with_d(mut self, value: u8) -> CpuBuilder {
        self.d = value;
        self
    }

    pub fn with_e(mut self, value: u8) -> CpuBuilder {
        self.e = value;
        self
    }

    pub fn with_h(mut self, value: u8) -> CpuBuilder {
        self.h = value;
        self
    }

    pub fn with_l(mut self, value: u8) -> CpuBuilder {
        self.l = value;
        self
    }

    pub fn with_iff1(mut self, value: bool) -> CpuBuilder {
        self.iff1 = value;
        self
    }

    pub fn with_iff2(mut self, value: bool) -> CpuBuilder {
        self.iff2 = value;
        self
    }

    pub fn with_bc(mut self, value: u16) -> CpuBuilder {
        self.b = value.high();
        self.c = value.low();
        self
    }

    pub fn with_de(mut self, value: u16) -> CpuBuilder {
        self.d = value.high();
        self.e = value.low();
        self
    }

    pub fn with_hl(mut self, value: u16) -> CpuBuilder {
        self.h = value.high();
        self.l = value.low();
        self
    }

    pub fn with_flag_s(mut self, value: bool) -> CpuBuilder {
        self.flag_s = value;
        self
    }

    pub fn with_flag_z(mut self, value: bool) -> CpuBuilder {
        self.flag_z = value;
        self
    }

    pub fn with_flag_c(mut self, value: bool) -> CpuBuilder {
        self.flag_c = value;
        self
    }

    pub fn with_flag_h(mut self, value: bool) -> CpuBuilder {
        self.flag_h = value;
        self
    }

    pub fn with_flag_pv(mut self, value: bool) -> CpuBuilder {
        self.flag_pv = value;
        self
    }

    pub fn with_flag_n(mut self, value: bool) -> CpuBuilder {
        self.flag_n = value;
        self
    }

    pub fn with_memory_size(mut self, size: u16) -> CpuBuilder {
        self.memory = Some(Vec::with_capacity(size as usize));
        self
    }

    pub fn with_memory(mut self, memory: Vec<u8>) -> CpuBuilder {
        self.memory = Some(memory);
        self
    }

    pub fn build(self) -> CPU {
        let mut cpu = CPU {
            pc: self.pc,
            sp: self.sp,
            ix: self.ix,
            iy: self.iy,
            i: self.i,
            r: self.r,
            a: self.a,
            b: self.b,
            c: self.c,
            d: self.d,
            e: self.e,
            f: self.f,
            h: self.h,
            l: self.l,
            a1: 0,
            b1: 0,
            c1: 0,
            d1: 0,
            e1: 0,
            f1: 0,
            h1: 0,
            l1: 0,
            iff1: self.iff1,
            iff2: self.iff2,
            memory: self.memory.unwrap(),
        };

        cpu.set_s(self.flag_s);
        cpu.set_z(self.flag_z);
        cpu.set_c(self.flag_c);
        cpu.set_h(self.flag_h);
        cpu.set_pv(self.flag_pv);
        cpu.set_n(self.flag_n);

        cpu
    }
}
