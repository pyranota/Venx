impl Plat{

    pub fn get_node(&self, position: Vec3, level: u8, entry: Entry, layer: Layer){
        todo!();
    }

    pub fn get_voxel(&self, position: Vec3, layer: Layer) {
        todo!();
    }


    pub fn at(&self, position: Vec3, level: u8, entry: Entry, layer: Layer) -> bool {
        todo!()
    }

    // solid_at -> solid_at_specific. Solid at has no more entry and layer
    pub fn solid_at(&self, position: Vec3, level: u8, entry: Entry, layer: Layer) -> bool {
        todo!()
    }
}

enum Entry<'a>{
    All,
    Single(u32),
    List(&'a [u32])
}

enum Layer<'a>{
    All,
    Single(u32),
    List(&'a [u32]),
}