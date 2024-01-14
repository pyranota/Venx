use anyhow::bail;

use super::{level::Level, node::Node, Plat};

#[derive(Debug)]
pub struct Layer {
    pub name: String,
    /// Nodes are organized in levels. That helps to instantly get all nodes at same level
    /// Each level contains only nodes that are referenced only one time
    /// You can safely edit this graph aslong it does not contain link to shared storage
    pub levels: Vec<Level>,
}

impl Layer {
    pub fn new(name: &str, depth: u8) -> Self {
        let mut levels = vec![
            Level {
                nodes: vec![
                    // Reserve with 999 nodes
                    Node {
                        flag: 9,
                        children: [999; 8]
                    }
                ],
                holder_head: 0,
            };
            depth as usize + 1
        ];
        // Push leaf node
        levels[0].nodes.push(Node {
            flag: 3,
            children: [1; 8],
        });
        Layer {
            name: name.to_string(),
            levels,
        }
    }
}

impl Plat {
    pub fn add_layer(&mut self, name: &str) -> anyhow::Result<()> {
        self.verify_layer_limit()?;
        self.layers.push(Layer::new(name, self.depth));
        Ok(())
    }

    pub fn insert_layer(&mut self, index: usize, name: &str) -> anyhow::Result<()> {
        self.verify_layer_limit()?;
        self.layers.insert(index, Layer::new(name, self.depth));
        Ok(())
    }

    pub fn remove_layer(&mut self, index: usize) -> anyhow::Result<()> {
        self.verify_layer_limit()?;
        self.layers.remove(index);
        Ok(())
    }

    pub fn remove_named_layer(&mut self, name: &str) -> anyhow::Result<()> {
        self.verify_layer_limit()?;
        self.layers.retain(|e| &e.name != name);
        Ok(())
    }

    fn verify_layer_limit(&self) -> anyhow::Result<()> {
        if self.layer_limit < self.layers.len() as u8 {
            bail!("You can not have more layers than {}", self.layer_limit);
        }
        Ok(())
    }
}
