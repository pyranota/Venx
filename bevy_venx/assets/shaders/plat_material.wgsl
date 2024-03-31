#import bevy_pbr::mesh_functions::{get_model_matrix, mesh_position_local_to_clip}

struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

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
    out.color = vec4(1., 1., 1., 1.);
    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}
