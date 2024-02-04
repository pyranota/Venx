use spirv_std::glam::*;

use crate::{
    plat::{chunk::chunk::Chunk, raw_plat::RawPlat},
    utils::{l2s, Grid},
};

type MeshHelper = Chunk;

pub type Mesh<'a> = &'a mut [(Vec3, Vec4, Vec3)]; // Position, Color, Normal

impl RawPlat {
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
        let scale = l2s(chunk.lod_level) as f32;
        let scale2 = l2s(chunk.lod_level) as f32;
        let mut line_direction = Vec3::ZERO;

        line_direction[line_idx] = 1.;

        if !mesh_helper.get(block_position).is_some()
            && chunk
                .get_neighbor(self, block_position.as_ivec3(), neighbor_direction)
                .is_none()
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
                if mesh_helper.get(next_pos).is_some() {
                    break;
                }
                // Is there a block at this position?
                if let Some(run_block) = chunk.get(next_pos) {
                    // Is it the same as origin block?
                    if run_block == block {
                        // Is it visible?
                        if chunk
                            .get_neighbor(self, (next_pos).as_ivec3(), neighbor_direction)
                            .is_none()
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
                    new_pos[width_idx] = block_position[width_idx] + line_width;
                    new_pos[line_idx] = block_position[line_idx] + c;
                    //dbg!((new_pos, pos));

                    // Is it used
                    if mesh_helper.get(new_pos).is_some() {
                        flag = false;
                        break;
                    }
                    // Is there a block at this position?
                    if let Some(run_block) = chunk.get(new_pos) {
                        // Is it the same as origin block?
                        if run_block == block {
                            // Is it visible?
                            if chunk
                                .get_neighbor(self, (new_pos).as_ivec3(), neighbor_direction)
                                .is_none()
                            {
                                //dbg!("Found");
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
                        new_pos[width_idx] = block_position[width_idx] + line_width;
                        new_pos[line_idx] = block_position[line_idx] + c;
                        mesh_helper.set(new_pos, 1);
                    }

                    line_width += 1;
                } else {
                    break;
                }
            }
            // Fill the mesh

            for mut vertex in face_vertices {
                // vertex.x *= stretcher.x;
                // vertex.y *= stretcher.y;

                vertex[line_idx] *= line_len as f32;
                vertex[width_idx] *= (line_width) as f32;

                mesh[*mesh_idx] = ((
                    ((vertex * scale2)
                        + (block_position + (chunk.position * chunk.size() * (scale as u32)))
                            .as_vec3()),
                    block_color,
                    neighbor_direction.as_vec3(),
                ));

                *mesh_idx += 1;
            }
        }
    }
}
