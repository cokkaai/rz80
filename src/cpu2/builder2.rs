use cpu2::Cpu;
use cpu2::Reg8;
use cpu2::RegW;
use cpu2::bytes;

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
pub struct CpuBuilder2 {
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
impl CpuBuilder2 {
    pub fn new() -> CpuBuilder2 {
        return CpuBuilder2 {
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

    pub fn with_pc(mut self, value: u16) -> CpuBuilder2 {
        self.pc = value;
        self
    }

    pub fn with_sp(mut self, value: u16) -> CpuBuilder2 {
        self.sp = value;
        self
    }

    pub fn with_ix(mut self, value: u16) -> CpuBuilder2 {
        self.ix = value;
        self
    }

    pub fn with_iy(mut self, value: u16) -> CpuBuilder2 {
        self.iy = value;
        self
    }

    pub fn with_i(mut self, value: u8) -> CpuBuilder2 {
        self.i = value;
        self
    }

    pub fn with_r(mut self, value: u8) -> CpuBuilder2 {
        self.r = value;
        self
    }

    pub fn with_a(mut self, value: u8) -> CpuBuilder2 {
        self.a = value;
        self
    }

    pub fn with_b(mut self, value: u8) -> CpuBuilder2 {
        self.b = value;
        self
    }

    pub fn with_c(mut self, value: u8) -> CpuBuilder2 {
        self.c = value;
        self
    }

    pub fn with_d(mut self, value: u8) -> CpuBuilder2 {
        self.d = value;
        self
    }

    pub fn with_e(mut self, value: u8) -> CpuBuilder2 {
        self.e = value;
        self
    }

    pub fn with_h(mut self, value: u8) -> CpuBuilder2 {
        self.h = value;
        self
    }

    pub fn with_l(mut self, value: u8) -> CpuBuilder2 {
        self.l = value;
        self
    }

    pub fn with_iff1(mut self, value: bool) -> CpuBuilder2 {
        self.iff1 = value;
        self
    }

    pub fn with_iff2(mut self, value: bool) -> CpuBuilder2 {
        self.iff2 = value;
        self
    }

    pub fn with_bc(mut self, value: u16) -> CpuBuilder2 {
        self.b = bytes::high(value);
        self.c = bytes::low(value);
        self
    }

    pub fn with_de(mut self, value: u16) -> CpuBuilder2 {
        self.d = bytes::high(value);
        self.e = bytes::low(value);
        self
    }

    pub fn with_hl(mut self, value: u16) -> CpuBuilder2 {
        self.h = bytes::high(value);
        self.l = bytes::low(value);
        self
    }

    pub fn with_flag_s(mut self, value: bool) -> CpuBuilder2 {
        self.flag_s = value;
        self
    }

    pub fn with_flag_z(mut self, value: bool) -> CpuBuilder2 {
        self.flag_z = value;
        self
    }

    pub fn with_flag_c(mut self, value: bool) -> CpuBuilder2 {
        self.flag_c = value;
        self
    }

    pub fn with_flag_h(mut self, value: bool) -> CpuBuilder2 {
        self.flag_h = value;
        self
    }

    pub fn with_flag_pv(mut self, value: bool) -> CpuBuilder2 {
        self.flag_pv = value;
        self
    }

    pub fn with_flag_n(mut self, value: bool) -> CpuBuilder2 {
        self.flag_n = value;
        self
    }

    pub fn with_memory_size(mut self, size: u16) -> CpuBuilder2 {
        self.memory = Some(Vec::with_capacity(size as usize));
        self
    }

    pub fn with_memory(mut self, memory: Vec<u8>) -> CpuBuilder2 {
        self.memory = Some(memory);
        self
    }

    pub fn build(self) -> Cpu {
        let a = Rc::new(RefCell::new(self.a));
        let b = Rc::new(RefCell::new(self.b));
        let c = Rc::new(RefCell::new(self.c));
        let d = Rc::new(RefCell::new(self.d));
        let e = Rc::new(RefCell::new(self.e));
        let f = Rc::new(RefCell::new(self.f));
        let h = Rc::new(RefCell::new(self.h));
        let l = Rc::new(RefCell::new(self.l));
        let a1 = Rc::new(RefCell::new(self.a));
        let b1 = Rc::new(RefCell::new(self.b));
        let c1 = Rc::new(RefCell::new(self.c));
        let d1 = Rc::new(RefCell::new(self.d));
        let e1 = Rc::new(RefCell::new(self.e));
        let f1 = Rc::new(RefCell::new(self.f));
        let h1 = Rc::new(RefCell::new(self.h));
        let l1 = Rc::new(RefCell::new(self.l));
        
        let mut cpu = Cpu {
            pc: self.pc,
            sp: self.sp,
            ix: self.ix,
            iy: self.iy,

            i: self.i,
            r: self.r,
            
            iff1: self.iff1,
            iff2: self.iff2,

            a: a.clone(),
            b: b.clone(),
            c: c.clone(),
            d: d.clone(),
            e: e.clone(),
            f: f.clone(),
            h: h.clone(),
            l: l.clone(),
            
            a1: a1.clone(),
            b1: b1.clone(),
            c1: c1.clone(),
            d1: d1.clone(),
            e1: e1.clone(),
            f1: f1.clone(),
            h1: h1.clone(),
            l1: l1.clone(),
            
            reg_a: Reg8::new(a.clone()),
            reg_b: Reg8::new(b.clone()),
            reg_c: Reg8::new(c.clone()),
            reg_d: Reg8::new(d.clone()),
            reg_e: Reg8::new(e.clone()),
            reg_f: Reg8::new(f.clone()),
            reg_h: Reg8::new(h.clone()),
            reg_l: Reg8::new(l.clone()),

            reg_af: RegW::new(a.clone(), f.clone()),
            reg_bc: RegW::new(b.clone(), c.clone()),
            reg_de: RegW::new(d.clone(), e.clone()),
            reg_hl: RegW::new(h.clone(), l.clone()),

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
