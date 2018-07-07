/// Compute two's complement of data.
pub fn compl2(data: u8) -> u8 {
    (!data).wrapping_add(1)
}
