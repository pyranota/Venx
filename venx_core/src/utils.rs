/// Level to size
/// If you always forget how to calculate size from level, you are welcome to use it
pub fn l2s(lvl: u8) -> u32 {
    1 << lvl
}
/// Size to level
/// 2^(level) = size
pub fn s2l(size: u32) -> u8 {
    size.ilog(2) as u8
}

pub type Grid<const SIZE: usize> = [[[u32; SIZE]; SIZE]; SIZE];
