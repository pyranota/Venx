use anyhow::{bail, Result};
use fastanvil::{complete, Chunk, Region};
use glam::{uvec3, Vec2, Vec3};
use std::{fs, ops::Range, path::PathBuf};

use crate::{plat::minecraft_blocks::match_block, voxel::segment::Segment};

use super::Plat;

pub type RegionX = i32;
pub type RegionZ = RegionX;

impl Plat {
    pub fn load_mca<'a>(
        dir_path: &'a str,
        region_range: (Range<RegionX>, Range<RegionZ>),
    ) -> Result<Self> {
        let rgs = from_dir(PathBuf::from(dir_path), region_range)?;

        let mut plat = Plat::new(12, 4, 9);

        for (rg_pos, mut region) in rgs {
            let mut segment = Segment::new(9);
            for ch_x in 0..32 {
                for ch_z in 0..32 {
                    if let Ok(Some(data)) = region.read_chunk(ch_x, ch_z) {
                        let complete_chunk = complete::Chunk::from_bytes(&data).unwrap();

                        for x in 0..16 {
                            for y in 0..380 {
                                for z in 0..16 {
                                    if let Some(block) = complete_chunk.block(x, y - 60, z) {
                                        if block.name() != "minecraft:air" {
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
                                                _ => 404,
                                            };

                                            // let block_id = 1;

                                            segment.set(
                                                uvec3(x as u32, y as u32, z as u32)
                                                    + uvec3(ch_x as u32 * 16, 0, ch_z as u32 * 16),
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
            dbg!("Set Segment");
            plat.controller.get_voxel_mut().set_segment(
                0,
                segment,
                uvec3(rg_pos[0] as u32, 0, rg_pos[1] as u32),
            );
            dbg!("Segment is inserted");
        }
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
