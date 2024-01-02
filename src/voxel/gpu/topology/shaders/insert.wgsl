// @group(0)
// @binding(0)
// var<storage, read_write> graph: Graph;

@group(1)
@binding(0)
var<storage, read> segment: Segment;

struct Segment {
    size: u32,
    line: array<u32>
}

@compute @workgroup_size(1, 1, 1)
fn insert_segment(@builtin(global_invocation_id) gid: vec3<u32>) {
}


