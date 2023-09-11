use anyhow::bail;

use naga_oil::compose::{
    ComposableModuleDescriptor, NagaModuleDescriptor, ShaderDefValue, ShaderLanguage,
};
use wgpu::{ShaderModule, ShaderSource};

use crate::ComputeServer;
use anyhow::Result;

impl ComputeServer {
    pub fn new_module(&mut self, src: &str) -> Result<ShaderModule> {
        dbg!("Before");
        let naga = self.make_naga(src, "")?;
        dbg!("After make naga");

        let _ = self.load_composable(src, "");

        dbg!("After");
        Ok(self
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: ShaderSource::Naga(std::borrow::Cow::Owned(naga)),
            }))
    }

    fn make_naga(&mut self, source: &str, file_path: &str) -> anyhow::Result<Module> {
        match self.composer.make_naga_module(NagaModuleDescriptor {
            source,
            file_path,
            ..Default::default()
        }) {
            Ok(module) => {
                // println!("{} -> {:#?}", module.name, module)
                Ok(module)
            }
            Err(e) => {
                bail!("? -> {e:#?}")
            }
        }
    }
    fn load_composable(&mut self, source: &str, file_path: &str) -> anyhow::Result<()> {
        match self
            .composer
            .add_composable_module(ComposableModuleDescriptor {
                source,
                file_path,
                language: ShaderLanguage::Wgsl,
                // as_name: Some("Module Hello".into()),
                ..Default::default()
            }) {
            Ok(_module) => {
                // println!("{} -> {:#?}", module.name, module)
                Ok(())
            }
            Err(e) => {
                bail!("? -> {e:#?}")
            }
        }
    }
}
