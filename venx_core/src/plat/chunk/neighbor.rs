use spirv_std::glam::{IVec3, UVec3};

use crate::{
    plat::{
        op::{EntryOpts, LayerOpts},
        raw_plat::RawPlat,
    },
    utils::l2s,
};

use super::chunk::{self, Chunk};

impl Chunk {
    pub fn get_neighbor(
        &self,
        plat: &RawPlat,
        local_block_position: impl Into<IVec3>,
        neighbor_direction: impl Into<IVec3>,
    ) -> Option<u32> {
        let real_chunk_size = l2s(self.level());
        let chunk_size = self.size();

        let dir: IVec3 = neighbor_direction.into();
        let pos: IVec3 = local_block_position.into();
        let sum = pos + dir;

        // TODO: Make better
        // Now its just saying that there is no one on border if chunk lod_level is above 0
        // It forces to draw every single border
        // It should do that only on borders with chunks on lover lod_level
        if self.lod_level() == 0
            && (sum.min_element() < 0 || sum.max_element() >= chunk_size as i32)
        {
            if let Some((.., (.., entry))) = plat.get_node(
                ((self.position() * real_chunk_size).as_ivec3() + sum).as_uvec3(),
                self.lod_level(),
                EntryOpts::All,
                LayerOpts::All,
            ) {
                return Some(entry as u32);
            }

            None
        } else {
            return self.get(sum.as_uvec3());
        }
    }
}

#[test]
fn neighbor_test() {
    let chunk = Chunk::new(UVec3::ZERO, 0, 4);

    // chunk.get_neighbor((13, 1, 3), (7, 1, 1));
}
