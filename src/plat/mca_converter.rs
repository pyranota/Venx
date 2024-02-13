use anyhow::{bail, Result};
use fastanvil::{complete, Chunk, Region};
use glam::{uvec3, Vec2, Vec3};
use pollster::block_on;
use std::{collections::HashMap, fs, ops::Range, path::PathBuf, usize};
use venx_core::utils::s2l;

use super::{interfaces::layer::LayerInterface, Plat, VenxPlat};

pub type RegionX = i32;
pub type RegionZ = RegionX;

impl VenxPlat {
    pub fn load_mca<'a>(
        dir_path: &'a str,
        region_range: (Range<RegionX>, Range<RegionZ>),
    ) -> Result<Self> {
        let rr = region_range.clone();
        let max_width = i32::max(rr.0.end - rr.0.start, rr.1.end - rr.1.start) * 512;

        let rgs = from_dir(PathBuf::from(dir_path), region_range)?;
        let mut plat = VenxPlat::new(s2l(max_width as u32), 5, 9);

        for (rg_pos, mut region) in rgs {
            //let mut segment = Segment::new(9);
            for ch_x in 0..32 {
                for ch_z in 0..32 {
                    if let Ok(Some(data)) = region.read_chunk(ch_x, ch_z) {
                        let complete_chunk = complete::Chunk::from_bytes(&data).unwrap();

                        for x in 0..16 {
                            for y in 0..380 {
                                for z in 0..16 {
                                    if let Some(block) = complete_chunk.block(x, y - 60, z) {
                                        // if let Some(amount) = hashmap.get_mut(block.name()) {
                                        //     *amount += 1u32;
                                        // } else {
                                        //     hashmap.insert(block.name().to_owned(), 1);
                                        // }

                                        if block.name() != "minecraft:air"
                                            && block.name() != "minecraft:grass"
                                        // && block.name() == "minecraft:stone"
                                        {
                                            // dbg!(block.name());
                                            //let block_id = match_block(block.name());
                                            let block_id = match block.name() {
                                                "minecraft:dirt" => 1,
                                                "minecraft:grass_block" => 2,
                                                "minecraft:stone" => 3,
                                                "minecraft:granite" => 4,
                                                "minecraft:diorite" => 5,
                                                "minecraft:andesite" => 6,
                                                "minecraft:bedrock" => 7,
                                                "minecraft:water" | "minecraft:flowing_water" => 8,
                                                "minecraft:gravel" => 9,
                                                "minecraft:gold_ore" => 10,
                                                "minecraft:iron_ore" => 11,
                                                "minecraft:coal_ore" => 12,
                                                "minecraft:oak_log" => 13,
                                                "minecraft:oak_leaves" => 14,
                                                "minecraft:lapis_ore" => 15,
                                                "minecraft:sand" => 16,
                                                "minecraft:grass" => 17,
                                                "minecraft:diamond_ore" => 18,
                                                "minecraft:birch_log" => 19,
                                                "minecraft:birch_leaves" => 20,
                                                "minecraft:dark_oak_log" => 21,
                                                "minecraft:dark_oak_leaves" => 22,
                                                _ => 1,
                                            };

                                            //let block_id = 1;

                                            plat.set_voxel(
                                                0,
                                                uvec3(x as u32, y as u32, z as u32)
                                                    + uvec3(ch_x as u32, 0, ch_z as u32) * 16
                                                    + uvec3(rg_pos[0] as u32, 0, rg_pos[1] as u32)
                                                        * 512,
                                                block_id,
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            // let segment_level = segment.level;
            // dbg!("Set Segment");
            // plat.controller.get_voxel_mut().set_segment(
            //     0,
            //     segment,
            //     uvec3(rg_pos[0] as u32, 0, rg_pos[1] as u32),
            // );
            // dbg!("Segment is inserted");

            // dbg!("Merging", (rg_pos[0] as u32, 0, rg_pos[1] as u32));
            // let v: &mut Voxel = plat.controller.get_voxel_mut().downcast_mut().unwrap();
            // v.layers[0].graph.merge_segment(
            //     (rg_pos[0] as u32, 0, rg_pos[1] as u32).into(),
            //     segment_level,
            // );
            // dbg!("Merged");
        }

        //   println!("{:?}", hashmap);
        //   panic!();

        Ok(plat)
    }
    pub fn load_mca_untyped<'a>(
        dir_path: &'a str,
        region_range: (Range<RegionX>, Range<RegionZ>),
    ) -> Result<Self> {
        let rr = region_range.clone();
        let max_width = i32::max(rr.0.end - rr.0.start, rr.1.end - rr.1.start) * 512;

        let rgs = from_dir(PathBuf::from(dir_path), region_range)?;
        let mut plat = VenxPlat::new(s2l(max_width as u32), 5, 9);

        let mut last_id = 1;
        let mut register = HashMap::new();

        for (rg_pos, mut region) in rgs {
            for ch_x in 0..32 {
                for ch_z in 0..32 {
                    if let Ok(Some(data)) = region.read_chunk(ch_x, ch_z) {
                        let complete_chunk = complete::Chunk::from_bytes(&data).unwrap();

                        for x in 0..16 {
                            for y in 0..380 {
                                for z in 0..16 {
                                    if let Some(block) = complete_chunk.block(x, y - 60, z) {
                                        if block.name() != "minecraft:air" {
                                            let block_id;

                                            if let Some(id) = register.get(block.name()) {
                                                block_id = *id;
                                            } else {
                                                register.insert(block.name().to_owned(), last_id);
                                                block_id = last_id;
                                                last_id += 1;
                                            }

                                            plat.set_voxel(
                                                0,
                                                uvec3(x as u32, y as u32, z as u32)
                                                    + uvec3(ch_x as u32, 0, ch_z as u32) * 16
                                                    + uvec3(rg_pos[0] as u32, 0, rg_pos[1] as u32)
                                                        * 512,
                                                block_id,
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        //   println!("{:?}", hashmap);
        //   panic!();

        Ok(plat)
    }
}

fn pos_from_name(name: &str) -> Option<[f32; 2]> {
    let parts: Vec<_> = name.split(".").collect();

    if parts.len() >= 3
        && parts[0] == "r"
        && parts[1].parse::<i32>().is_ok() // confirm that the second and third parts are nums
        && parts[2].parse::<i32>().is_ok()
    {
        Some([
            parts[1].parse().expect("Checked in the conditional"),
            parts[2].parse().expect("Checked in the conditional"),
        ])
    } else {
        None
    }
}
fn from_dir(
    dir: PathBuf,
    region_range: (Range<RegionX>, Range<RegionZ>),
) -> anyhow::Result<Vec<([i32; 2], Region<std::fs::File>)>> {
    let start = (region_range.0.start, region_range.1.start);
    let end = (region_range.0.end, region_range.1.end);

    let dir = fs::read_dir(dir)?;
    let mut out = Vec::new();
    for path in dir {
        let path = path?.path();
        let name = path.file_name();
        if let Some(name) = name {
            let coords = pos_from_name(name.to_str().unwrap());
            if let Some(coords) = coords {
                let x = coords[0] as i32;
                let z = coords[1] as i32;
                if (x, z) >= start && (x, z) < end {
                    let file = std::fs::File::open(path).unwrap();

                    let region = Region::from_stream(file).unwrap();
                    out.push(([x - start.0, z - start.1], region));
                }

                continue;
            }
        }
        bail!("File path did not contain coords: {:?}", path);
    }
    Ok(out)
}
