fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "turbo")]
    {
        use spirv_builder::{MetadataPrintout, SpirvBuilder};
        SpirvBuilder::new("crates/venx_shaders", "spirv-unknown-vulkan1.2")
            .print_metadata(MetadataPrintout::Full)
            .build()?;
    }
    Ok(())
}
