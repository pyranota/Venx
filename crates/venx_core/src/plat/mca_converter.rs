use std::path::PathBuf;

use mca_parser::*;

use super::Plat;

impl Plat {
    pub fn load_mca<'a>(dir_path: &'a str) -> Self {
        let rgs = from_directory(PathBuf::from(dir_path));

        assert!(rgs.is_ok(), "Unable to read test dir: {:?}", rgs.err());

        let mut rgs = rgs.unwrap();

        if let Some(rg) = rgs.get_region(RegionPosition::new(5, 5)) {
            rg.parse().unwrap();
        }
        todo!()
    }
}
