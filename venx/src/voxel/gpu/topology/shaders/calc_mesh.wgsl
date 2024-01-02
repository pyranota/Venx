
@group(2)
@binding(0)
var<storage, read_write> mesh_sizes: array<u32>;

@group(3)
@binding(0)
var<storage, read_write> meshes: array<Vertex>;

struct Vertex {
    color: vec4<f32>
}

@compute
@workgroup_size(1)
fn greedy_sizes(@builtin(global_invocation_id) global_id: vec3<u32>) {
}

@compute
@workgroup_size(1)
fn naive_sizes(@builtin(global_invocation_id) global_id: vec3<u32>) {
}

@compute
@workgroup_size(1)
fn greedy(@builtin(global_invocation_id) global_id: vec3<u32>) {
}

@compute
@workgroup_size(1)
fn naive(@builtin(global_invocation_id) global_id: vec3<u32>) {
}