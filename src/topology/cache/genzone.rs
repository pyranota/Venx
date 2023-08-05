use glam::UVec3;

use super::{
    subzone::{SubZone, SubZoneFlex},
    table::Block,
};

#[derive(Clone)]
pub struct GenZone<const DEPTH: usize, const SIZE: usize> {
    /// Depth 2+
    reg2: Box<SubZone<SIZE, SubZone<2, Block>>>,
    /// Depth 4+
    reg4_opt: Option<SubZoneFlex<SubZone<4, Block>>>,
    /// Depth 8+
    reg8_opt: Option<SubZoneFlex<SubZone<8, Block>>>,
    /// Depth 16+
    reg16_opt: Option<SubZoneFlex<SubZone<16, Block>>>,
    is_complete: bool,
    position: UVec3,
}

impl<const DEPTH: usize, const SIZE: usize> GenZone<DEPTH, SIZE> {
    fn to_slice(&self) -> GenSlice {
        todo!()
    }

    fn init(position: UVec3) -> Self {
        let reg2: SubZone<SIZE, SubZone<2, Block>> = SubZone::default();
        let reg4_opt = if DEPTH >= 4 {
            Some(SubZoneFlex::init(SIZE / 4))
        } else {
            None
        };
        let reg8_opt = if DEPTH >= 8 {
            Some(SubZoneFlex::init(SIZE / 8))
        } else {
            None
        };
        let reg16_opt = if DEPTH >= 16 {
            Some(SubZoneFlex::init(SIZE / 16))
        } else {
            None
        };

        GenZone {
            reg2: Box::new(reg2),
            is_complete: false,
            position,
            reg4_opt,
            reg8_opt,
            reg16_opt,
        }
    }
    fn set(&mut self, x: usize, y: usize, z: usize, block: Block) {
        if DEPTH >= 2 {
            // wow
            // First we need to find the SubZoneFlex to put in.
            // In that SubZoneFlex is Deeper SubZone, so we find the cell where block is located.
            self.reg2.0[x / 2][y / 2][z / 2].0[x % 2][y % 2][z % 2] = block;
        }
        if let Some(reg) = &mut self.reg4_opt {
            reg.0[x / 4][y / 4][z / 4].0[x % 4][y % 4][z % 4] = block;
        }
        if let Some(reg) = &mut self.reg8_opt {
            reg.0[x / 8][y / 8][z / 8].0[x % 8][y % 8][z % 8] = block;
        }
        if let Some(reg) = &mut self.reg16_opt {
            reg.0[x / 16][y / 16][z / 16].0[x % 16][y % 16][z % 16] = block;
        }
    }
}

#[derive(Hash, PartialEq, Eq)]
pub struct GenSlice<'a>(pub &'a [&'a [Block; 2]]);

#[cfg(test)]
mod tests {
    #![feature(test)]
    use ndarray::{arr2, arr3, s, Array3, ArrayBase, Dim, OwnedRepr};
    use rand::Rng;

    use crate::topology::cache::genzone::GenSlice;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn core() {
        let arr = arr2(&[[1, 2, 3], [2, 3, 4], [51, 5, 1]]);
        let slice = arr.slice(s![0..2, 0..2]);

        gen_zone(5);
    }

    // ~9000 secs for initing gen_zones for 100kx100k // on level 8 // 90ms per init
    // lvl 2// 64 size // 50 us per init // ~ 8min in 100kx100k // ~1min in 50kx50x
    #[bench]
    fn init_gen_zone(b: &mut test::Bencher) {
        b.iter(|| {
            let zone: GenZone<2, 32> = GenZone::init(UVec3::ZERO);
        })
    }

    // 2ms 64^3 // lvl 8
    // 20ms 128^3 // lvl 8
    // 150ms 256^3 // lvl 8
    #[bench]
    fn fill_zone(b: &mut test::Bencher) {
        const SIZE: usize = 32;
        let mut zone: GenZone<2, SIZE> = GenZone::init(UVec3::ZERO);
        b.iter(|| {
            for x in 0..SIZE {
                for y in 0..SIZE {
                    for z in 0..SIZE {
                        zone.set(x, y, z, Block::grass());
                    }
                }
            }
        })
    }

    #[bench]
    fn subzone_init(b: &mut test::Bencher) {
        b.iter(|| {
            let a: SubZone<8, Block> = SubZone::default();
        })
    }
    #[bench]
    fn subzoneflex_init(b: &mut test::Bencher) {
        b.iter(|| {
            let a: SubZoneFlex<i32> = SubZoneFlex::init(512);
        })
    }

    #[test]
    fn learn_slicing() {
        let mut arr = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];

        let iter = arr[0..2].iter().map(|s| &s[0..2]);

        for slice in iter {
            for x in slice {
                print!("{} ", x);
            }
            println!();
        }
    }

    // 70us on 64
    #[bench]
    fn slices_again(b: &mut test::Bencher) {
        const size: usize = 64;
        let mut counter: i128 = 0;
        let mut zone = Box::new([[[Block::default(); size]; size]; size]);
        b.iter(|| {
            counter = 0;
            //let mut arr = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
            for x in 0..(size / 2) {
                for y in 0..(size / 2) {
                    for z in 0..(size / 2) {
                        let iter = zone[0..x].iter().map(|s| &s[0..y]);

                        for slice in iter {
                            for slice2 in slice {
                                for x in &slice2[0..z] {
                                    counter += 1;
                                }

                                //print!("{} ", x);
                            }
                            //println!();
                        }
                    }
                }
            }

            //dbg!(counter);
        });

        dbg!(counter);
    }

    #[bench]
    fn init_arr(b: &mut test::Bencher) {
        b.iter(|| {
            let arr = [[[Block::default(); 256]; 256]; 256];
        });
    }

    #[bench]
    fn slicing_gen_zone_1k(b: &mut test::Bencher) {
        // 0.5 secs for 512x512x512 to be sliced

        b.iter(|| {
            let size = 64;
            let zone = gen_zone(size);
            // Iter over it;
            //for div in 1..512 {
            let mut counter = 0;
            for x in 0..((size - 1) / 2) {
                for y in 0..((size - 1) / 2) {
                    for z in 0..((size - 1) / 2) {
                        let slice = zone.slice(s![
                            (x * 2)..(x * 2 + 2),
                            (y * 2)..(y * 2 + 2),
                            (z * 2)..(z * 2 + 2)
                        ]);
                        counter += 1;
                    }
                }
            }
            //dbg!(counter);
        });
    }

    fn gen_zone(size: usize) -> ArrayBase<OwnedRepr<u16>, Dim<[usize; 3]>> {
        let mut rng = rand::thread_rng();
        let mut arr = Array3::<u16>::zeros((size, size, size));
        // for x in 0..size {
        //     for y in 0..size {
        //         for z in 0..size {
        //             arr[[x, y, z]] = rng.gen();
        //         }
        //     }
        // }
        arr
        //dbg!(arr);
    }
}
