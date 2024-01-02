struct Graph {
    head_holder_idx: u32,
    segment_size: u32,
    compression_level: u32,
    nodes: array<GNode>,
}

struct GNode {
    ref_count: u32
}