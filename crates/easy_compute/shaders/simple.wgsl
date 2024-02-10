


@group(0)@binding(0)
var<storage, read_write> list: array<i32, 5>;


@compute @workgroup_size(5)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
    list[gid.x] = 998;
}