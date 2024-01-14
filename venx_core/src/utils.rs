/// Level to size
/// If you always forget how to calculate size from level, you are welcome to use it
pub fn l2s(lvl: u8) -> u32 {
    1 << lvl
}
/// Size to level
/// 2^(level) = size
pub fn s2l(size: u32) -> anyhow::Result<u8> {
    todo!()
}
