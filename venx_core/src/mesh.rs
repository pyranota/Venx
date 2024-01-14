use std::collections::HashSet;

use glam::*;

pub type Mesh = Vec<(Vec3, Vec4, Vec3)>; // Position, Color, Normal

fn greedy_runner(
    mesh_helper: &mut HashSet<UVec3>,
    chunk: &Chunk,
    block: i32,
    block_position: UVec3,
    line_idx: usize,
    width_idx: usize,
    neighbor_direction: IVec3,
    mesh: &mut Mesh,
    block_color: Vec4,
    face_vertices: [Vec3; 6],
) {
    let scale = lvl_to_size(chunk.lod_level) as f32;
    let scale2 = lvl_to_size(chunk.lod_level) as f32;
    let mut line_direction = Vec3::ZERO;

    line_direction[line_idx] = 1.;

    if !mesh_helper.contains(&block_position)
        && self
            .get_neighbor(chunk, block_position.as_ivec3(), neighbor_direction)
            .is_none()
    {
        // Create run
        // 1 it is just our first block
        let mut line_len: u32 = 1;
        mesh_helper.insert(block_position);
        // Iter over entire line
        // Positive
        loop {
            // Local position of next block in line
            let next_pos =
                (block_position.as_vec3() + line_direction * (line_len as f32)).as_uvec3();

            // Is it used
            if mesh_helper.contains(&next_pos) {
                break;
            }
            // Is there a block at this position?
            if let Some(run_block) = chunk.get(next_pos) {
                // Is it the same as origin block?
                if run_block == block {
                    // Is it visible?
                    if self
                        .get_neighbor(chunk, (next_pos).as_ivec3(), neighbor_direction)
                        .is_none()
                    {
                        //dbg!("Found");
                        // Marking this block as
                        mesh_helper.insert(next_pos);
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
                if mesh_helper.contains(&new_pos) {
                    flag = false;
                    break;
                }
                // Is there a block at this position?
                if let Some(run_block) = chunk.get(new_pos) {
                    // Is it the same as origin block?
                    if run_block == block {
                        // Is it visible?
                        if self
                            .get_neighbor(chunk, (new_pos).as_ivec3(), neighbor_direction)
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
                    mesh_helper.insert(new_pos);
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

            mesh.push((
                ((vertex * scale2)
                    + (block_position + (chunk.position * chunk.size() * (scale as u32)))
                        .as_vec3()),
                block_color,
                neighbor_direction.as_vec3(),
            ))
        }
    }
}
