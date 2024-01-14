#[derive(Debug)]
pub struct Layer<'a> {
    /// Link to first node which is empty (flag == -1)
    /// If there is no empty nodes its 0
    pub holder_head: usize,
    /// Every node on level(depth) is entry node
    /// Each entry represents root of graph
    /// That means, that in single `Graph` struc, you can have multiple graphs
    /// That is used to have voxel types in graph
    /// All graphs are merged
    /// By creating new entry you create new graph

    /// Keep in mind that anything on 0 is reserved and not usable
    /// You can identify this types of nodes with 9 in every field of it
    /// This is in that way because if there would be node at 0 index,
    /// that would conflict with 0 as "no child" interpretation
    pub nodes: &'a mut [super::node::Node],
}

// impl Layer {
//     pub fn new(name: &str, depth: u8) -> Self {
//         let mut levels = vec![
//             Level {
//                 nodes: vec![
//                     // Reserve with 999 nodes
//                     Node {
//                         flag: 9,
//                         children: [999; 8]
//                     }
//                 ],
//                 holder_head: 0,
//             };
//             depth as usize + 1
//         ];
//         // Push leaf node
//         levels[0].nodes.push(Node {
//             flag: 3,
//             children: [1; 8],
//         });
//         Layer {
//             name: name.to_string(),
//             levels,
//         }
//     }
// }

// impl Plat {
//     pub fn add_layer(&mut self, name: &str) -> anyhow::Result<()> {
//         self.verify_layer_limit()?;
//         self.layers.push(Layer::new(name, self.depth));
//         Ok(())
//     }

//     pub fn insert_layer(&mut self, index: usize, name: &str) -> anyhow::Result<()> {
//         self.verify_layer_limit()?;
//         self.layers.insert(index, Layer::new(name, self.depth));
//         Ok(())
//     }

//     pub fn remove_layer(&mut self, index: usize) -> anyhow::Result<()> {
//         self.verify_layer_limit()?;
//         self.layers.remove(index);
//         Ok(())
//     }

//     pub fn remove_named_layer(&mut self, name: &str) -> anyhow::Result<()> {
//         self.verify_layer_limit()?;
//         self.layers.retain(|e| &e.name != name);
//         Ok(())
//     }

//     fn verify_layer_limit(&self) -> anyhow::Result<()> {
//         if self.layer_limit < self.layers.len() as u8 {
//             bail!("You can not have more layers than {}", self.layer_limit);
//         }
//         Ok(())
//     }
// }
