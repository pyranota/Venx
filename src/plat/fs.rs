use std::{fs::*, io::Write};

use glam::Quat;
use log::info;
use venx_core::plat::{node::Node, node_l2::NodeL2};

use crate::plat::{
    loader::{external_buffer::FakeBuffer, VenxLoader},
    normal::cpu_plat::CpuPlat,
    MetaSerDeser, Plat,
};

use super::VenxPlat;

impl VenxPlat {
    /// Save plat to .cache directory
    pub fn save(&self, name: &str) -> anyhow::Result<()> {
        info!("Saving {name}.plat");
        let path = ".cache/".to_owned() + name;

        // TODO: Allow .cache in custom location
        create_dir_all(format!("{}.plat", path))?;
        create_dir_all(format!("{}.plat/layers/", path))?;
        create_dir_all(format!("{}.plat/report", path))?;

        self.chart_node_destibution(&format!("{}.plat/report/node_destribution", path), name)?;

        let mut file = File::create(format!("{}.plat/meta.ron", path))?;

        // TODO: make use of transfer.    XXXXXXXXXXXXXXXXXXXXX
        let raw_plat = self.get_normal_unchecked().borrow_raw_plat();
        //                                ^^^^^^^^^^^^^^^^^^^^^-- Will fail if plat lives on GPU
        let meta: String = ron::ser::to_string_pretty(
            &MetaSerDeser {
                depth: raw_plat.depth,
                position: (0., 0., 0.),
                rotation: (0., 0., 0.),
            },
            ron::ser::PrettyConfig::default(),
        )?;
        file.write_all(meta.as_bytes())?;

        // Create layers dirs
        for (layer_name, layer) in raw_plat.layers() {
            let layer_path = format!("{path}.plat/layers/{layer_name}");

            create_dir_all(&layer_path)?;

            let encoded_entries: Vec<u8> = bitcode::encode(layer.level_2).unwrap();
            let encoded_nodes: Vec<u8> = bitcode::encode(layer.nodes).unwrap();

            let mut l2_file = File::create(format!("{}/level_2", layer_path))?;
            l2_file.write_all(&encoded_entries)?;

            let mut nodes_file = File::create(format!("{}/nodes", layer_path))?;
            nodes_file.write_all(&encoded_nodes)?;
        }

        Ok(())
    }

    pub fn load(path: &str, vertex_pool: VertexPool) -> anyhow::Result<Self> {
        info!("Loading {path}.plat");
        let path = ".cache/".to_owned() + path;
        let meta: MetaSerDeser = ron::from_str(&read_to_string(format!("{path}.plat/meta.ron"))?)?;

        let mut components = [
            (vec![], vec![]),
            (vec![], vec![]),
            (vec![], vec![]),
            (vec![], vec![]),
        ];

        for (i, layer_name) in ["base", "tmp", "schem", "canvas"].iter().enumerate() {
            let l2_path = format!("{path}.plat/layers/{layer_name}/level_2");
            let nodes_path = format!("{path}.plat/layers/{layer_name}/nodes");

            let l2: Vec<NodeL2> = bitcode::decode(&read(l2_path)?)?;
            let nodes: Vec<Node> = bitcode::decode(&read(nodes_path)?)?;

            components[i] = (nodes, l2);
        }

        // TODO: overcome bottleneck. Handle this without cloning
        Ok(VenxPlat {
            plat: Plat::Cpu(CpuPlat::from_existing(
                meta.depth,
                5,
                5,
                components[0].clone(),
                components[1].clone(),
                components[2].clone(),
                components[3].clone(),
            )),
            loader: VenxLoader::new(
                ([0., 0., 0.].into(), Quat::default(), 50),
                10,
                10,
                Box::new(FakeBuffer),
                Box::new(FakeBuffer),
            ),
            smbcs: vec![],
        })
    }
}
