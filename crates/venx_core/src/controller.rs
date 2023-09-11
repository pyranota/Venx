use crate::voxel_data::VXdata;

/**
 *
 */
pub(crate) struct Controller {
    data: VXdata,
}

#[cfg(feature = "gpu")]
impl Controller {
    pub(crate) async fn transfer_to_gpu(self) -> Self {
        todo!()
    }
    pub(crate) async fn transfer_to_cpu(self) -> Self {
        todo!()
    }
    pub(crate) async fn toggle(self) -> Self {
        todo!()
    }
}
