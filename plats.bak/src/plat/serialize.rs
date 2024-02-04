use serde::{ser::SerializeStruct, Serialize, Serializer};

use super::{layer::Layer, level::Level};

// impl Serialize for Layer {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         serializer.serialize_str(&format!("./{}.layer.ron", self.name))
//     }
// }

// impl Serialize for Level {
//     fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         let mut state = s.serialize_struct("Level", 2)?;
//         state.serialize_field("holder_head", &self.holder_head)?;
// let encoded: Vec<u8> = bitcode::encode(&self.nodes).unwrap();
// state.serialize_field("nodes", &encoded)?;
//         state.end()
//     }
// }
