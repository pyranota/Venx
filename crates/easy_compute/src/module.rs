use std::borrow::Cow;

// use naga::Module;
// use naga_oil::compose::{ComposableModuleDescriptor, NagaModuleDescriptor, ShaderLanguage};
use wgpu::ShaderModule;
use wgpu::ShaderModuleDescriptor;

use crate::ComputeServer;
use anyhow::Result;

impl ComputeServer {
    pub fn new_module(&mut self, shader: &str) -> Result<ShaderModule> {
        let shader = self
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Shader"),
                source: wgpu::ShaderSource::Wgsl(Cow::Owned(shader.to_owned())),
            });

        Ok(shader)
    }
    // pub fn new_module(&mut self, src: &str) -> Result<ShaderModule> {
    //     let naga = self.make_naga(src, "")?;
    //     let _ = self.load_composable(src, "");
    //     Ok(self
    //         .device
    //         .create_shader_module(wgpu::ShaderModuleDescriptor {
    //             label: None,
    //             source: ShaderSource::Naga(std::borrow::Cow::Owned(naga)),
    //         }))
    // }

    pub fn new_module_spv(&mut self, shader: ShaderModuleDescriptor) -> Result<ShaderModule> {
        let shader = self.device.create_shader_module(shader);

        Ok(shader)
    }

    // fn make_naga(&mut self, source: &str, file_path: &str) -> anyhow::Result<Module> {
    //     match self.composer.make_naga_module(NagaModuleDescriptor {
    //         source,
    //         file_path,
    //         ..Default::default()
    //     }) {
    //         Ok(module) => {
    //             // println!("{} -> {:#?}", module.name, module)
    //             Ok(module)
    //         }
    //         Err(e) => {
    //             bail!("? -> {e:#?}")
    //         }
    //     }
    // }
    // fn load_composable(&mut self, source: &str, file_path: &str) -> anyhow::Result<()> {
    //     match self
    //         .composer
    //         .add_composable_module(ComposableModuleDescriptor {
    //             source,
    //             file_path,
    //             language: ShaderLanguage::Wgsl,
    //             // as_name: Some("Module Hello".into()),
    //             ..Default::default()
    //         }) {
    //         Ok(_module) => {
    //             // println!("{} -> {:#?}", module.name, module)
    //             Ok(())
    //         }
    //         Err(e) => {
    //             bail!("? -> {e:#?}")
    //         }
    //     }
    // }
}
