use anyhow::{bail, Result};
use bevy::utils::HashMap;
use fastanvil::{complete, Chunk, Region};
use glam::{uvec3, Vec2, Vec3};
use std::{fs, path::PathBuf};

use crate::voxel::{segment::Segment, vx_trait::VoxelTrait};

use super::Plat;

impl Plat {
    pub fn load_mca<'a>(dir_path: &'a str) -> Result<Self> {
        let rgs = from_dir(PathBuf::from(dir_path))?;

        let mut plat = Plat::new(10, 4, 9);

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
                                            segment.set(
                                                uvec3(x as u32, y as u32, z as u32)
                                                    + uvec3(ch_x as u32 * 16, 0, ch_z as u32 * 16),
                                                1,
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            dbg!("Inserting");
            plat.insert_segment(segment, uvec3(0, 0, 0));
            return Ok(plat);
        }
        todo!()
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
fn from_dir(dir: PathBuf) -> anyhow::Result<Vec<([f32; 2], Region<std::fs::File>)>> {
    let dir = fs::read_dir(dir)?;
    let mut out = Vec::new();
    for path in dir {
        let path = path?.path();
        let name = path.file_name();
        if let Some(name) = name {
            let coords = pos_from_name(name.to_str().unwrap());
            if let Some(coords) = coords {
                let file = std::fs::File::open(path).unwrap();

                let region = Region::from_stream(file).unwrap();
                out.push((coords, region));
                continue;
            }
        }
        bail!("File path did not contain coords: {:?}", path);
    }
    Ok(out)
}
