struct Graph {
    compression_level: u32, // 0..128
    // First is always root and second is always leaf
    depth: u32,
    nodes: array<Node>,
    
}
struct Node {
    ref_count: u32,
    attr_count: u32,
    metadata: u32, // 8 bits for mirroring/offsetting, else for global and segement merged indication
    children: array<u32, 8> // 0 - No child, 1 - leaf
}
