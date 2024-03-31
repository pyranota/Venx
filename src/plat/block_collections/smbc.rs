use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    hash::Hash,
};

use anyhow::bail;
use ron::de::SpannedError;
use serde::{Deserialize, Serialize};

use super::{
    bc::{BCIdent, BCs, BlockCollection},
    BLOCK_LIMIT,
};

pub type SmallBlockCollection = SMBC;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
/// Small Block Collection
/// Low quality reflection of BC
pub struct SMBC {
    /// Required [BlockCollection]s
    requires: HashSet<BCIdent>,
    blocks: Vec<BlockReflection>,
    //               name     id
    name_id: HashMap<String, usize>,

    /// Alloc index. Everything before is used. Everything after can be used for new allocations
    alloc: usize,
}

impl SMBC {
    pub fn serialize(&self) -> Result<String, ron::Error> {
        ron::ser::to_string_pretty(&self, ron::ser::PrettyConfig::default())
    }
    pub fn deserialize(content: String) -> Result<Self, SpannedError> {
        ron::from_str(&content)
    }
    pub fn new() -> Self {
        let mut name_id = HashMap::new();

        name_id.insert("Air".to_owned(), 0);
        SMBC {
            blocks: vec![BlockReflection {
                block_collection: "None".into(),
                name: "Air".into(),
                hardness: 0,
                color: [0.; 4],
                is_solid: false,
                lod_offset: 0,
            }],
            requires: HashSet::new(),
            alloc: 1,
            name_id,
        }
    }
    // TODO: Write remove(shrink) function.
    /// Extends with new block collection.
    pub fn extend(&mut self, bc: &BlockCollection) -> anyhow::Result<()> {
        let len = bc.blocks.len();

        if len + self.alloc >= BLOCK_LIMIT {
            bail!("Cannot add new block collection. Max amount of block types ({BLOCK_LIMIT}) would be exceeded. Free up space first!");
        }

        self.requires.insert(bc.ident.clone());

        for (name, block) in &bc.blocks {
            // Reflecting block
            let reflection = BlockReflection {
                block_collection: bc.ident.name.clone(),
                name: name.clone(),
                hardness: block.hardness,
                color: [1.; 4], // TODO: Texture to color
                is_solid: block.is_solid,
                lod_offset: block.lod_offset,
            };

            self.blocks.push(reflection);
            self.name_id.insert(name.clone(), self.alloc);
            self.alloc += 1;
        }

        Ok(())
    }

    /// Bind plat's SMBC with loaded BCs by string names. Even though string operations is not the best idea in the world,
    ///
    /// But it allows to seemlesly replace block collections without breaking anything.
    ///
    /// Hash could be used to speed it up. But collisions are possible and i find it less intuitive.
    ///
    /// For example block with name "stone" in smbc may occupy id 14, so it will find block in loaded BC(s) with same string name and
    ///
    /// It will return ordered array of textures, attributes and other things
    ///
    /// If corresponding block was not found in Block Collections, it will fallback to values in SMBC
    pub fn map(&self, bcs: BCs) {
        let mut view = Vec::with_capacity(self.blocks.len());

        for (id, block_desc) in self.blocks.iter().enumerate() {
            if let Some((.., block)) = bcs.find_block(&block_desc.name) {
                view[id] = block;
            }
        }
    }

    pub fn id(&self, name: &str) -> Option<&usize> {
        self.name_id.get(name)
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
// TODO: Implement `state`
/// Block Descriptor
pub struct BlockReflection {
    pub block_collection: String,
    /// Bind this block with block in Block Collection. Yes its string
    pub name: String,
    /// Fallback values if BC is not found. May be deprecated
    ///
    /// 0..100 Where 0 is oneshot and 100 is unbreakable
    pub hardness: u8,
    /// RGB color
    pub color: [f32; 4],
    /// false if its transparent block or has another shape other than 1x1x1 block
    pub is_solid: bool,
    /// Default is 0. LoD of loaded chunk where this block located at will be `lod` of chunk + `lod_offset` of this block
    pub lod_offset: i32,
    // TODO: Separete
    // Create FallbackBlockDesc
    // Put color, original bc_name
}

// TODO:

// // Bind this block with block in Block Collection. Yes its string
// identifier: stone,
// // Id which used by this block in this particular Platform
// id: 1,
// hardness: 10,
// color: [0.5, 1., 0., 1.],
// blending_mod: Add,
// is_solid: true,
// lod_offset: 0,
// emissive: None,
// // Help AI with understanding how to use this block
// naturally_spawned: true,
// type: non_underwater,
// rarety: 1/100,
// description: "Stone",

#[cfg(test)]
mod tests {
    use crate::plat::block_collections::{
        bc::{Block, BlockCollection},
        smbc::SmallBlockCollection,
    };

    use super::SMBC;
    #[ignore = "Not testing anything, just print serialized value"]
    #[test]
    fn serialize() {
        let mut smbc = SmallBlockCollection::new();

        let bc = BlockCollection::testing();

        smbc.extend(&bc).unwrap();
        println!("{}", smbc.serialize().unwrap());
    }

    #[test]
    fn get_id() {
        let mut smbc = SmallBlockCollection::new();

        let bc = BlockCollection::testing();

        smbc.extend(&bc).unwrap();

        let id = smbc.id("Air").unwrap();
        assert_eq!(&smbc.blocks[*id].name, "Air");

        let id = smbc.id("Stone").unwrap();
        assert_eq!(&smbc.blocks[*id].name, "Stone");

        let id = smbc.id("Water").unwrap();
        assert_eq!(&smbc.blocks[*id].name, "Water");

        let id = smbc.id("Dirt").unwrap();
        assert_eq!(&smbc.blocks[*id].name, "Dirt");
    }

    #[test]
    fn ser_deser_empty() {
        let smbc = SmallBlockCollection::new();

        assert_eq!(smbc, SMBC::deserialize(smbc.serialize().unwrap()).unwrap());
    }

    #[test]
    fn ser_deser() {
        let mut smbc = SmallBlockCollection::new();

        let bc = BlockCollection::testing();

        smbc.extend(&bc).unwrap();

        assert_eq!(smbc, SMBC::deserialize(smbc.serialize().unwrap()).unwrap());
    }
}
