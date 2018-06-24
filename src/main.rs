mod cpu;

pub fn promote(h: u8, l: u8) -> u16 {
    ((h as u16) << 8) + l as u16
}

fn main() {
    let a = promote(0x00, 0x04);

    println!("{:?}", a);

}

