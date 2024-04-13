#import bevy_pbr::mesh_functions::{get_model_matrix, mesh_position_local_to_clip}

struct Vertex {
    @location(0) position: vec3<f32>,
    // @location(1) ty: u32,
    // @location(2) normal: u32,
};
// For each 12 vertices we have single FaceMeta
// NOTE: Due to alignment issues, i cannot have just ty and normal in single FaceMeta
// Not to waste space, i combine 2 Faces in single
// struct FaceMeta{
//     // First face
//     ty: u32,
//     normal: u32,

//     // Second face
//     ty: u32,
//     normal: u32,
// }
// @group(0) @binding(2) var<uniform> faces_meta: array<FaceMeta>;

// struct BCView{
//     ty: u32,
//     normal: u32,
// }
// @group(0) @binding(2) var<uniform> faces_meta: arr<FaceMeta>;
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    let position = vertex.position;
    var out: VertexOutput;

    out.clip_position = mesh_position_local_to_clip(
        get_model_matrix(0u),
        vec4<f32>(position, 1.0)
    );
    out.color = vec4((position % 256. ) / 256., 1.);
    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}
