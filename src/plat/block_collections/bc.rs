use std::collections::HashMap;

use ron::de::SpannedError;
use serde::{Deserialize, Serialize};

use super::smbc::{BlockReflection, SMBC};

/*
    name.plat.bc
        default.tex
            stone.png
            iron.png
            wood.png
            leafes.png
            volumetric_texture.ron

        manifest.ron
*/

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct BlockCollection {
    // #[serde(flatten)]
    pub(super) ident: BCIdent,
    pub(super) blocks: HashMap<String, Block>,
}

impl BlockCollection {
    /// Generates template for BlockCollection
    pub fn template(name: &str) -> Self {
        Self {
            ident: BCIdent {
                name: name.into(),
                version: "0.1.0".into(),
                link: None,
            },
            blocks: HashMap::new(),
        }
    }

    pub(super) fn testing() -> Self {
        let mut bc = BlockCollection::template("testing");

        bc.add_block(Block {
            name: "Stone".into(),
            hardness: 60,
            is_solid: true,
            lod_offset: 0,
            texture: vec![],
        });

        bc.add_block(Block {
            name: "Water".into(),
            hardness: 100,
            is_solid: false,
            lod_offset: 0,
            texture: vec![],
        });

        bc.add_block(Block {
            name: "Dirt".into(),
            hardness: 30,
            is_solid: true,
            lod_offset: 0,
            texture: vec![],
        });

        bc
    }

    pub fn add_block(&mut self, block: Block) {
        self.blocks.insert(block.name.clone(), block);
    }

    pub fn serialize(&self) -> Result<String, ron::Error> {
        ron::ser::to_string_pretty(&self, ron::ser::PrettyConfig::default())
    }
    pub fn deserialize(content: String) -> Result<Self, SpannedError> {
        ron::from_str(&content)
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct Block {
    pub name: String,
    /// 0..100 Where 0 is oneshot and 100 is unbreakable
    pub hardness: u8,
    /// false if its transparent block or has another shape other than 1x1x1 block
    pub is_solid: bool,
    /// Default is 0. LoD of loaded chunk where this block located at will be `lod` of chunk + `lod_offset` of this block
    pub lod_offset: i32,
    // TODO: Make serializable + Enum to just clear color
    #[serde(skip)]
    pub texture: Vec<u8>,
}

// Name -> BlockCollection
pub struct BCs(pub(super) HashMap<BCIdent, Box<BlockCollection>>);

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug)]
pub struct BCIdent {
    pub name: String,
    /// Test?
    pub version: String,
    pub link: Option<String>,
}

impl BCs {
    pub fn load(&mut self, bc: BlockCollection) {
        todo!()
    }
    // TODO: Optimize with BCIdent as input in arguments
    /// Find block and its properties by given string
    pub fn find_block(&self, name: &str) -> Option<(BCIdent, Block)> {
        for (ident, bc) in &self.0 {
            if let Some(block) = bc.blocks.get(name) {
                return Some((ident.clone(), block.clone()));
            }
        }
        None
    }
}

/*

plat.set_voxel(0, plat.bc_view("Stone"));


*/

#[cfg(test)]
mod tests {
    use super::{Block, BlockCollection, SMBC};
    #[ignore = "Not testing anything, just print serialized value"]
    #[test]
    fn new() {
        let bc = BlockCollection::testing();

        println!("{}", bc.serialize().unwrap());
    }

    #[test]
    fn ser_deser() {
        let bc = BlockCollection::testing();

        assert_eq!(
            bc,
            BlockCollection::deserialize(bc.serialize().unwrap()).unwrap()
        );
    }
}
