use glam::{ivec3, ivec4, uvec3, vec4, UVec3, Vec3, Vec4};

use crate::chunk::chunk::Chunk;

use super::voxel::Voxel;

pub type Mesh = Vec<(Vec3, Vec4)>; // Position, Color

impl Voxel {
    /// No side culling
    pub fn to_mesh_no_culling(&self, chunk: &Chunk) -> Mesh {
        let mut mesh = vec![];

        chunk.iter(|pos, block| {
            if block != 0 {
                let cube = cube::FULL;
                for vertex in cube {
                    mesh.push(((vertex + pos.as_vec3()), vec4(1., 1., 1., 1.)))
                }
            }
        });
        mesh
    }
    /// Fast way to do it, with side culling
    pub fn to_mesh_naive(&self, chunk: &Chunk) -> Mesh {
        let mut mesh = vec![];

        chunk.iter(|pos, block| {
            if block != 0 {
                let block_color = match block {
                    1 => ivec3(111, 54, 55),           // Dirt
                    2 | 17 => ivec3(93, 189, 101),     // Grass
                    3 | 5 | 6 => ivec3(213, 213, 213), // Stone + Diorite + Andesite
                    4 => ivec3(255, 155, 155),         // Granite
                    7 => ivec3(0, 0, 0),               // Bedrock
                    8 => ivec3(131, 162, 255),         // Water
                    9 => ivec3(186, 186, 186),         // Gravel
                    10 => ivec3(255, 214, 9),          // Gold ore
                    11 => ivec3(226, 226, 226),        // Iron ore
                    12 => ivec3(47, 47, 47),           // Coal ore
                    13 => ivec3(156, 81, 0),           // Oak log
                    14 => ivec3(0, 163, 33),           // Oak leaves
                    15 => ivec3(27, 96, 243),          // Lapis ore
                    16 => ivec3(245, 241, 169),        // Sand
                    18 => ivec3(116, 243, 255),        // Diamond ore
                    19 => ivec3(196, 151, 80),         // Birch log
                    20 => ivec3(60, 223, 83),          // Birch leaves
                    21 => ivec3(126, 51, 0),           // Dark Oak log
                    22 => ivec3(0, 143, 13),           // Dark Oak leaves
                    _ => ivec3(0, 0, 0),               // Else
                };

                let block_color = block_color.as_vec3().extend(1.) / vec4(256., 256., 256., 1.0);

                if self
                    .get_neighbor(
                        chunk,
                        (pos - chunk.position * chunk.size()).as_ivec3(),
                        (0, 1, 0),
                    )
                    .is_none()
                {
                    let cube = cube::TOP;
                    for vertex in cube {
                        mesh.push(((vertex + pos.as_vec3()), block_color))
                    }
                } else {
                }
                if self
                    .get_neighbor(
                        chunk,
                        (pos - chunk.position * chunk.size()).as_ivec3(),
                        (0, -1, 0),
                    )
                    .is_none()
                {
                    let cube = cube::BOTTOM;
                    for vertex in cube {
                        mesh.push(((vertex + pos.as_vec3()), block_color))
                    }
                }
                if self
                    .get_neighbor(
                        chunk,
                        (pos - chunk.position * chunk.size()).as_ivec3(),
                        (1, 0, 0),
                    )
                    .is_none()
                {
                    let cube = cube::RIGHT;
                    for vertex in cube {
                        mesh.push(((vertex + pos.as_vec3()), block_color))
                    }
                }
                if self
                    .get_neighbor(
                        chunk,
                        (pos - chunk.position * chunk.size()).as_ivec3(),
                        (-1, 0, 0),
                    )
                    .is_none()
                {
                    let cube = cube::LEFT;
                    for vertex in cube {
                        mesh.push(((vertex + pos.as_vec3()), block_color))
                    }
                }
                if self
                    .get_neighbor(
                        chunk,
                        (pos - chunk.position * chunk.size()).as_ivec3(),
                        (0, 0, 1),
                    )
                    .is_none()
                {
                    let cube = cube::FRONT;
                    for vertex in cube {
                        mesh.push(((vertex + pos.as_vec3()), block_color))
                    }
                }
                if self
                    .get_neighbor(
                        chunk,
                        (pos - chunk.position * chunk.size()).as_ivec3(),
                        (0, 0, -1),
                    )
                    .is_none()
                {
                    let cube = cube::BACK;
                    for vertex in cube {
                        mesh.push(((vertex + pos.as_vec3()), block_color))
                    }
                }
            }
        });
        mesh
    }
    /// Side culling and Greedy meshing
    pub fn to_mesh(&self, chunk: &Chunk) -> Mesh {
        let mut mesh = vec![];

        chunk.iter(|pos, block| {
            if block != 0 {
                let cube = cube::FULL;
                for vertex in cube {
                    mesh.push(((vertex + pos.as_vec3()), vec4(1., 1., 1., 1.)))
                }
            }
        });
        mesh
    }
}

// #[test]
// fn test_mesh_creation() {
//     let mut vx = Voxel::new(5, 3, 5);

//     vx.topology.set(uvec3(0, 0, 0), true);
//     vx.topology.set(uvec3(0, 5, 0), true);
//     vx.topology.set(uvec3(1, 1, 0), true);
//     // second chunk
//     vx.topology.set(uvec3(0, 8, 0), true);

//     let chunk = vx.load_chunk(UVec3::ZERO, 0).unwrap();
//     let mesh = vx.to_mesh(&chunk);
//     assert_eq!(mesh.len(), 36 * 3);

//     let chunk = vx.load_chunk(uvec3(0, 1, 0), 0).unwrap();
//     let mesh = vx.to_mesh(&chunk);
//     assert_eq!(mesh.len(), 36);
// }

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
