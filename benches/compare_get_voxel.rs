/*
    If you want to get a voxel on given position and if you know what entry (essentially type of block) is there,
    you can just plat.get_node(position, 0, LayerOpts::All, EntryOpts::Single(entry)). This is the most efficient way to do this.
    But if you have already noticed, getting voxel information which you already know, is not the use case.
    So instead we use plat.get_voxel(position) which is essentially plat.get_node(position, 0, LayerOpts::All, EntryOpts::All).
    And that can be harmfull in terms of performance. It iterates over all entries (which can be thousands).
    This benchmark aimed to find out what is exact difference between those methods in time and what is most expencive in those operations.
    Hopefully it will provide any ideas how to improve the situation.

    Potential outcomes:
        Use forked structure? (Accepted)
        Lookup tables?
        Dont do anything?


*/

// TODO: compare get_node(position, LayerOpts::Single(0), EntryOpts::Single(n)) and NodeAddr::from_position(position);

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use log::warn;
use rand::Rng;
use venx::plat::{interfaces::layer::LayerInterface, VenxPlat};
use venx_core::{glam::uvec3, plat::node::NodeAddr};

fn criterion_benchmark(c: &mut Criterion) {
    // let plat = VenxPlat::load("mca_small_all_blocks").unwrap_or_else(|e| {
    //     warn!("Plat wasnt found on device, creating new and saving ({e})");
    //     // Convert from minecraft map

    //     plat.save("mca_small_all_blocks").unwrap();
    //     plat
    // });
    let plat = VenxPlat::load_mca_untyped("./assets/mca/1/", (0..1, 0..1)).unwrap();

    // let mut rand_plat;

    //let mut rng = rand::thread_rng();

    //let mtx: Box<[[[u16; 512]; 224]; 512]> = rng.gen();

    // for x in 0..16 {
    //     for y in 0..16 {
    //         for z in 0..16 {
    //             let voxel_id = mtx[x][y][z] as u32 + 1;
    //             plat[0].set(uvec3(x as u32, y as u32, z as u32), voxel_id);
    //         }
    //     }
    // }
    // Just find all entries before the start

    let mut mtx = [[[0; 16]; 7]; 16];
    for x in 0..16 {
        for y in 0..7 {
            for z in 0..16 {
                let entry_opt = plat.get_voxel((x * 32, y * 32, z * 32).into());
                if let Some(entry) = entry_opt {
                    mtx[x as usize][y as usize][z as usize] = entry.voxel_id;
                }
            }
        }
    }

    let mut full_mtx = Box::new([[[0; 512]; 224]; 512]);
    for x in 0..512 {
        for y in 0..224 {
            for z in 0..512 {
                let entry_opt = plat.get_voxel((x, y, z).into());
                if let Some(entry) = entry_opt {
                    full_mtx[x as usize][y as usize][z as usize] = entry.voxel_id;
                }
            }
        }
    }

    let mut rng = rand::thread_rng();

    c.bench_function("gen_random_position", |b| {
        b.iter(|| {
            let random_position = (
                rng.gen_range(0..512),
                rng.gen_range(0..224),
                rng.gen_range(0..512),
            );

            // To make sure compiler wont remove this
            let cached = full_mtx[random_position.0 as usize][random_position.1 as usize]
                [random_position.2 as usize];

            if cached != 0 {}
        });
    });

    c.bench_function("get_node_rand_pos_unknown_layer_unknown_entry", |b| {
        b.iter(|| {
            let random_position = (
                rng.gen_range(0..512),
                rng.gen_range(0..224),
                rng.gen_range(0..512),
            );

            let entry_opt = plat.get_voxel(random_position.into());
            let cached = full_mtx[random_position.0 as usize][random_position.1 as usize]
                [random_position.2 as usize];

            if let Some(entry) = &entry_opt {
                if entry.voxel_id != cached {
                    if entry_opt.is_none() && cached == 0 {
                    } else {
                        panic!();
                    }
                }
            }
        });
    });
    c.bench_function("get_node_rand_pos_known_layer_unknown_entry", |b| {
        b.iter(|| {
            let random_position = (
                rng.gen_range(0..512),
                rng.gen_range(0..224),
                rng.gen_range(0..512),
            );

            let entry_opt = plat.get_normal_unchecked().borrow_raw_plat()[0].get_node(
                random_position.into(),
                0,
                None,
            );
            let cached = full_mtx[random_position.0 as usize][random_position.1 as usize]
                [random_position.2 as usize];

            if cached != entry_opt.voxel_id {
                panic!()
            }
        });
    });

    c.bench_function("get_node_rand_pos_known_layer_known_entry", |b| {
        b.iter(|| {
            let random_position = (
                rng.gen_range(0..512),
                rng.gen_range(0..224),
                rng.gen_range(0..512),
            );
            let voxel_id = full_mtx[random_position.0 as usize][random_position.1 as usize]
                [random_position.2 as usize];

            let res = plat.get_normal_unchecked().borrow_raw_plat()[0].get_node(
                random_position.into(),
                0,
                Some(voxel_id),
            );

            if res.is_none() && voxel_id != 0 {
                panic!()
            }
        });
    });

    // c.bench_function("get_voxel_unknown_entry", |b| {
    //     b.iter(|| {
    //         for x in 0..16 {
    //             for y in 0..7 {
    //                 for z in 0..16 {
    //                     let entry_opt = plat.get_voxel((x * 32, y * 32, z * 32).into());
    //                     let cached = mtx[x as usize][y as usize][z as usize];
    //                     if let Some(entry) = &entry_opt {
    //                         if entry.voxel_id != cached {
    //                             if entry_opt.is_none() && cached == 0 {
    //                             } else {
    //                                 panic!();
    //                             }
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     })
    // });

    // c.bench_function("get_voxel_known_entry", |b| {
    //     b.iter(|| {
    //         for x in 0..16 {
    //             for y in 0..7 {
    //                 for z in 0..16 {
    //                     let entry_opt = plat.get_normal_unchecked().borrow_raw_plat()[0].get_node(
    //                         (x * 32, y * 32, z * 32).into(),
    //                         0,
    //                         None, // EntryOpts::Single(mtx[x as usize][y as usize][z as usize] as u32),
    //                               // LayerOpts::All,
    //                     );
    //                     let cached = mtx[x as usize][y as usize][z as usize];

    //                     if cached != entry_opt.voxel_id {
    //                         panic!()
    //                     }
    //                 }
    //             }
    //         }
    //     })
    // });

    c.bench_function("create_rand_address", |b| {
        b.iter(|| {
            let random_position = (
                rng.gen_range(0..512),
                rng.gen_range(0..224),
                rng.gen_range(0..512),
            );

            let addr = NodeAddr::from_position(random_position.into(), 9, 0);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
