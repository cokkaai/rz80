pub fn low(value: u16) -> u8 {
    (value & 0x00ff) as u8
}

pub fn high(value: u16) -> u8 {
    ((value & 0xff00) >> 8) as u8
}

pub fn promote(h: u8, l: u8) -> u16 {
    ((h as u16) << 8) + l as u16
}

/// Compute two's complement of data.
pub fn compl2(data: u8) -> u8 {
    (!data).wrapping_add(1)
}

