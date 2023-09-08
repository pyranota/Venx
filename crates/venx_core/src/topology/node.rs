struct Node {
    link_to: u32, // 0 is not link,
    ref_count: u32,
    attr_count: u32,
    inter_idx: u32,
    children: [u32; 8],
}
