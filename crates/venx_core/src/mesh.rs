use spirv_std::glam::*;

use crate::{
    plat::{chunk::chunk::Chunk, raw_plat::RawPlat},
    utils::{l2s, Grid},
};

type MeshHelper = Chunk;

pub type Mesh<'a> = &'a mut [[f32; 10]]; // Position, Color, Normal

impl RawPlat<'_> {
    pub fn greedy_runner(
        &self,
        mesh_helper: &mut MeshHelper,
        chunk: &Chunk,
        block: u32,
        block_position: UVec3,
        line_idx: usize,
        width_idx: usize,
        neighbor_direction: IVec3,
        mesh: Mesh,
        mesh_idx: &mut usize,
        block_color: Vec4,
        face_vertices: [Vec3; 6],
    ) {
        let scale = l2s(chunk.lod_level()) as f32;

        let mut line_direction = Vec3::ZERO;

        match line_idx {
            0 => line_direction.x = 1.,
            1 => line_direction.y = 1.,
            2 => line_direction.z = 1.,
            _ => panic!(),
        }

        if !mesh_helper.get_unchecked(block_position) != 0
            && chunk.get_neighbor_unchecked(self, block_position.as_ivec3(), neighbor_direction)
                == 0
        {
            // Create run
            // 1 it is just our first block
            let mut line_len: u32 = 1;
            mesh_helper.set(block_position, 1);
            // Iter over entire line
            // Positive
            loop {
                // Local position of next block in line
                let next_pos =
                    (block_position.as_vec3() + line_direction * (line_len as f32)).as_uvec3();

                // Is it used
                if mesh_helper.get_unchecked(next_pos) != 0 {
                    break;
                }
                // // Is there a block at this position?
                let run_block = chunk.get_unchecked(next_pos);
                if run_block != 0 {
                    // Is it the same as origin block?
                    if run_block == block {
                        // // Is it visible?
                        if chunk.get_neighbor_unchecked(
                            self,
                            (next_pos).as_ivec3(),
                            neighbor_direction,
                        ) == 0
                        {
                            //dbg!("Found");
                            // Marking this block as
                            mesh_helper.set(next_pos, 1);
                            // It continues only if all conditions were met
                            line_len += 1;
                            continue;
                        }
                    }
                }
                // Breaking a loop if any condition returned false
                break;
            }

            // Calculating width

            let mut line_width = 1;
            let mut flag = true;

            loop {
                let mut new_pos = block_position.clone();

                for c in 0..line_len {
                    match width_idx {
                        0 => new_pos.x = block_position.x + line_width,
                        1 => new_pos.y = block_position.y + line_width,
                        2 => new_pos.z = block_position.z + line_width,
                        _ => panic!(),
                    }

                    match line_idx {
                        0 => new_pos.x = block_position.x + c,
                        1 => new_pos.y = block_position.y + c,
                        2 => new_pos.z = block_position.z + c,
                        _ => panic!(),
                    }

                    // Is it used
                    if mesh_helper.get_unchecked(new_pos) != 0 {
                        flag = false;
                        break;
                    }
                    // Is there a block at this position?
                    let run_block = chunk.get_unchecked(new_pos);
                    if run_block != 0 {
                        // Is it the same as origin block?
                        if run_block == block {
                            // Is it visible?
                            if chunk.get_neighbor_unchecked(
                                self,
                                (new_pos).as_ivec3(),
                                neighbor_direction,
                            ) == 0
                            {
                                // Marking this block as
                                // It continues only if all conditions were met
                                continue;
                            }
                        }
                    }

                    // Breaking a loop if any condition returned false
                    flag = false;
                    break;
                }
                // If this line can be used
                if flag {
                    for c in 0..line_len {
                        let mut new_pos = block_position.clone();

                        match width_idx {
                            0 => new_pos.x = block_position.x + line_width,
                            1 => new_pos.y = block_position.y + line_width,
                            2 => new_pos.z = block_position.z + line_width,
                            _ => panic!(),
                        }

                        match line_idx {
                            0 => new_pos.x = block_position.x + c,
                            1 => new_pos.y = block_position.y + c,
                            2 => new_pos.z = block_position.z + c,
                            _ => panic!(),
                        }
                        mesh_helper.set(new_pos, 1);
                    }

                    line_width += 1;
                } else {
                    break;
                }
            }
            // Fill the mesh

            for vertex_idx in 0..6 {
                let mut vertex = face_vertices[vertex_idx];

                match line_idx {
                    0 => vertex.x *= line_len as f32,
                    1 => vertex.y *= line_len as f32,
                    2 => vertex.z *= line_len as f32,
                    _ => panic!(),
                }

                match width_idx {
                    0 => vertex.x *= line_width as f32,
                    1 => vertex.y *= line_width as f32,
                    2 => vertex.z *= line_width as f32,
                    _ => panic!(),
                }

                // TODO: Optimize
                let position = ((vertex * scale)
                    + (block_position * scale as u32 + (chunk.position() * chunk.width()))
                        .as_vec3())
                .to_array();

                let nd = neighbor_direction.as_vec3().to_array();
                mesh[*mesh_idx] = [
                    position[0],
                    position[1],
                    position[2],
                    block_color.x,
                    block_color.y,
                    block_color.z,
                    block_color.w,
                    nd[0],
                    nd[1],
                    nd[2], //neighbor_direction,
                ];

                *mesh_idx += 1;
                // todo!()
            }
        }
    }
}
