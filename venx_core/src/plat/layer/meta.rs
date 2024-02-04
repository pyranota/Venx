/// Storing what types of voxels are present in specific regions
/// Used to speed up traversing for prise of additional RAM usage
/// Deeper level results more memory used, but faster raycasting + traversing
pub struct LayerMeta{
    otree: OTree<&[BlockType]>
}

