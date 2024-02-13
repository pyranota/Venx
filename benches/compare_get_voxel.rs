/*
    If you want to get a voxel on given position and if you know what entry (essentially type of block) is there,
    you can just plat.get_node(position, 0, LayerOpts::All, EntryOpts::Single(entry)). This is the most efficient way to do this.
    But if you have already noticed, getting voxel information which you already know, is not the use case.
    So instead we use plat.get_voxel(position) which is essentially plat.get_node(position, 0, LayerOpts::All, EntryOpts::All).
    And that can be harmfull in terms of performance. It iterates over all entries (which can be thousands).
    This benchmark aimed to find out what is exact difference between those methods in time and what is most expencive in those operations.
    Hopefully it will provide any ideas how to improve the situation.

    Potential outcomes:
        Use forked structure?
        Lookup tables?
        Dont do anything?


*/

// TODO: compare get_node(position, LayerOpts::Single(0), EntryOpts::Single(n)) and NodeAddr::from_position(position);

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use log::warn;
use venx::plat::{interfaces::layer::LayerInterface, VenxPlat};
use venx_core::plat::{
    node::NodeAddr,
    op::{EntryOpts, LayerOpts},
};

fn criterion_benchmark(c: &mut Criterion) {
    let plat = VenxPlat::load("mca_small_all_blocks").unwrap_or_else(|e| {
        warn!("Plat wasnt found on device, creating new and saving ({e})");
        // Convert from minecraft map
        let plat = VenxPlat::load_mca_untyped("./assets/mca/1/", (0..1, 0..1)).unwrap();
        plat.save("mca_small_all_blocks").unwrap();
        plat
    });

    // Just find all entries before the start

    let mut mtx = [[[0; 16]; 7]; 16];
    for x in 0..16 {
        for y in 0..7 {
            for z in 0..16 {
                let entry_opt = plat.get_voxel((x * 32, y * 32, z * 32).into());
                if let Some(entry) = entry_opt {
                    mtx[x as usize][y as usize][z as usize] = entry;
                }
            }
        }
    }

    c.bench_function("get_voxel_unknown_entry", |b| {
        b.iter(|| {
            for x in 0..16 {
                for y in 0..7 {
                    for z in 0..16 {
                        let entry_opt = plat.get_voxel((x * 32, y * 32, z * 32).into());
                        let cached = mtx[x as usize][y as usize][z as usize];
                        if Some(cached) != entry_opt {
                            if entry_opt.is_none() && cached == 0 {
                            } else {
                                panic!();
                            }
                        }
                    }
                }
            }
        })
    });

    c.bench_function("get_voxel_known_entry", |b| {
        b.iter(|| {
            for x in 0..16 {
                for y in 0..7 {
                    for z in 0..16 {
                        let entry_opt = plat.get_normal_unchecked().borrow_raw_plat().get_node(
                            (x * 32, y * 32, z * 32).into(),
                            0,
                            EntryOpts::Single(mtx[x as usize][y as usize][z as usize] as u32),
                            LayerOpts::All,
                        );
                        let cached = mtx[x as usize][y as usize][z as usize];
                        if let Some((.., (.., entry))) = entry_opt {
                            if cached != entry {
                                panic!()
                            }
                        } else {
                            if entry_opt.is_none() && cached == 0 {
                            } else {
                                panic!();
                            }
                        }
                    }
                }
            }
        })
    });

    c.bench_function("create_addresses", |b| {
        b.iter(|| {
            for x in 0..16 {
                for y in 0..7 {
                    for z in 0..16 {
                        let addr = NodeAddr::from_position((x * 32, y * 32, z * 32).into(), 0);
                        // let entry_opt = plat.get_normal_unchecked().borrow_raw_plat().get_node(
                        //     (x * 32, y * 32, z * 32).into(),
                        //     0,
                        //     EntryOpts::Single(mtx[x as usize][y as usize][z as usize] as u32),
                        //     LayerOpts::All,
                        // );
                        // let cached = mtx[x as usize][y as usize][z as usize];
                        // if let Some((.., (.., entry))) = entry_opt {
                        //     if cached != entry {
                        //         panic!()
                        //     }
                        // } else {
                        //     if entry_opt.is_none() && cached == 0 {
                        //     } else {
                        //         panic!();
                        //     }
                        // }
                    }
                }
            }
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
