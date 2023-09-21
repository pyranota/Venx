use glam::{uvec3, vec4, UVec3, Vec3, Vec4};

use crate::chunk::chunk::Chunk;

use super::voxel::Voxel;

pub type Mesh = Vec<(Vec3, Vec4)>; // Position, Color

impl Voxel {
    pub fn to_mesh(&self, chunk: &Chunk) -> Mesh {
        let mut mesh = vec![];

        chunk.iter(|pos, block| {
            if block {
                let cube = cube::FULL;
                for vertex in cube {
                    mesh.push(((vertex + pos.as_vec3()), vec4(1., 1., 1., 1.)))
                }
            }
        });
        mesh
    }
}

#[test]
fn test_mesh_creation() {
    let mut vx = Voxel::new(5, 3, 5);

    vx.topology.set(uvec3(0, 0, 0), true);
    vx.topology.set(uvec3(0, 5, 0), true);
    vx.topology.set(uvec3(1, 1, 0), true);
    // second chunk
    vx.topology.set(uvec3(0, 8, 0), true);

    let chunk = vx.load_chunk(UVec3::ZERO, 0);
    let mesh = vx.to_mesh(&chunk);
    assert_eq!(mesh.len(), 36 * 3);

    let chunk = vx.load_chunk(uvec3(0, 1, 0), 0);
    let mesh = vx.to_mesh(&chunk);
    assert_eq!(mesh.len(), 36);
}

pub mod cube {
    use glam::Vec3;

    pub const FRONT: [Vec3; 6] = [
        Vec3::new(-0., -0., 1.0),
        Vec3::new(1.0, -0., 1.0),
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(-0., 1.0, 1.0),
        Vec3::new(-0., -0., 1.0),
    ];

    pub const BACK: [Vec3; 6] = [
        Vec3::new(1.0, -0., -0.),
        Vec3::new(-0., -0., -0.),
        Vec3::new(-0., 1.0, -0.),
        Vec3::new(-0., 1.0, -0.),
        Vec3::new(1.0, 1.0, -0.),
        Vec3::new(1.0, -0., -0.),
    ];

    pub const TOP: [Vec3; 6] = [
        Vec3::new(-0., 1.0, 1.0),
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(1.0, 1.0, -0.),
        Vec3::new(1.0, 1.0, -0.),
        Vec3::new(-0., 1.0, -0.),
        Vec3::new(-0., 1.0, 1.0),
    ];

    pub const BOTTOM: [Vec3; 6] = [
        Vec3::new(1.0, -0., 1.0),
        Vec3::new(-0., -0., 1.0),
        Vec3::new(-0., -0., -0.),
        Vec3::new(-0., -0., -0.),
        Vec3::new(1.0, -0., -0.),
        Vec3::new(1.0, -0., 1.0),
    ];

    pub const RIGHT: [Vec3; 6] = [
        Vec3::new(1.0, -0., 1.0),
        Vec3::new(1.0, -0., -0.),
        Vec3::new(1.0, 1.0, -0.),
        Vec3::new(1.0, 1.0, -0.),
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(1.0, -0., 1.0),
    ];

    pub const LEFT: [Vec3; 6] = [
        Vec3::new(-0., -0., -0.),
        Vec3::new(-0., -0., 1.0),
        Vec3::new(-0., 1.0, 1.0),
        Vec3::new(-0., 1.0, 1.0),
        Vec3::new(-0., 1.0, -0.),
        Vec3::new(-0., -0., -0.),
    ];

    pub const FULL: [Vec3; 36] = [
        // front face
        Vec3::new(-0., -0., 1.0),
        Vec3::new(1.0, -0., 1.0),
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(-0., 1.0, 1.0),
        Vec3::new(-0., -0., 1.0),
        // back face
        Vec3::new(1.0, -0., -0.),
        Vec3::new(-0., -0., -0.),
        Vec3::new(-0., 1.0, -0.),
        Vec3::new(-0., 1.0, -0.),
        Vec3::new(1.0, 1.0, -0.),
        Vec3::new(1.0, -0., -0.),
        // top face
        Vec3::new(-0., 1.0, 1.0),
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(1.0, 1.0, -0.),
        Vec3::new(1.0, 1.0, -0.),
        Vec3::new(-0., 1.0, -0.),
        Vec3::new(-0., 1.0, 1.0),
        // bottom face
        Vec3::new(1.0, -0., 1.0),
        Vec3::new(-0., -0., 1.0),
        Vec3::new(-0., -0., -0.),
        Vec3::new(-0., -0., -0.),
        Vec3::new(1.0, -0., -0.),
        Vec3::new(1.0, -0., 1.0),
        // right face
        Vec3::new(1.0, -0., 1.0),
        Vec3::new(1.0, -0., -0.),
        Vec3::new(1.0, 1.0, -0.),
        Vec3::new(1.0, 1.0, -0.),
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(1.0, -0., 1.0),
        // left face
        Vec3::new(-0., -0., -0.),
        Vec3::new(-0., -0., 1.0),
        Vec3::new(-0., 1.0, 1.0),
        Vec3::new(-0., 1.0, 1.0),
        Vec3::new(-0., 1.0, -0.),
        Vec3::new(-0., -0., -0.),
    ];
}
