#[repr(C)]
#[derive(
    Copy, Debug, Clone, Default, PartialEq, PartialOrd, bytemuck::Pod, bytemuck::Zeroable, Hash, Eq,
)]
#[cfg_attr(feature = "bitcode_support", derive(bitcode::Encode, bitcode::Decode))]
/// Nodes on level_2
pub struct NodeL2 {
    packed_children: [u32; 2],
}
