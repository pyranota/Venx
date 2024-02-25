/// Level to size
/// If you always forget how to calculate size from level, you are welcome to use it
pub fn l2s(lvl: usize) -> u32 {
    1 << lvl
}
/// Size to level
/// 2^(level) = size
pub fn s2l(size: u32) -> usize {
    size.ilog(2) as usize
}

pub type Grid<const SIZE: usize> = [[[u32; SIZE]; SIZE]; SIZE];

#[cfg(test)]
pub mod test_utils {
    use alloc::{boxed::Box, vec::Vec};
    use rand::Rng;
    use spirv_std::glam::uvec3;

    use crate::plat::{layer::layer::Lr, raw_plat::RawPlat};
    extern crate alloc;
    extern crate std;

    pub fn gen_rand_mtx<const SIZE: usize>(
        empty_probability: u8,
    ) -> std::boxed::Box<Vec<Vec<Vec<u32>>>> {
        let mut rng = rand::thread_rng();
        let mut mtx = Box::new(alloc::vec![alloc::vec![alloc::vec![0; SIZE]; SIZE]; SIZE]);

        for x in 0..SIZE {
            for y in 0..SIZE {
                for z in 0..SIZE {
                    if !rng.gen_ratio(empty_probability as u32, 100) {
                        let voxel_id: u16 = rng.gen();
                        // To prevent 0
                        mtx[x][y][z] = voxel_id as u32 + 1;
                    }
                }
            }
        }
        mtx
    }

    pub fn set_rand_plat<const SIZE: usize>(
        plat: &mut RawPlat,
        empty_probability: u8,
    ) -> std::boxed::Box<Vec<Vec<Vec<u32>>>> {
        let mtx = gen_rand_mtx::<SIZE>(empty_probability);
        for x in 0..SIZE {
            for y in 0..SIZE {
                for z in 0..SIZE {
                    let voxel_id = mtx[x][y][z];

                    plat[Lr::BASE].set(uvec3(x as u32, y as u32, z as u32), voxel_id);
                }
            }
        }
        mtx
    }
}
