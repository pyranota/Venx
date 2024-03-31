use core::ops::{Index, IndexMut};

use crate::plat::{
    node::{AllocatableNode, Node},
    node_l2::NodeL2,
};

/// Alias for layer
pub type Lr<'a> = Layer<'a>;

#[derive(PartialEq)]
pub struct Layer<'a> {
    /// Synced depth with RawPlat
    pub depth: usize,

    /// On level 0 we have single node
    ///
    /// On level 1 we have 256 nodes
    ///
    /// On level 2 we have majority of nodes in entire layer
    ///
    /// But we dont need to have u32 to link each child, since on level 1 only 256 nodes
    ///
    /// Instead we have u8 to link each child.
    ///
    /// There is no level 0 or 1. They are "phantom" levels, which is "packed" in this u8 value
    ///
    /// Can be called `l2`
    ///
    /// [0] is head of `free nodes`
    pub level_2: &'a mut [NodeL2],

    /// [0] is head of `free nodes`
    ///
    /// [1] root
    pub nodes: &'a mut [Node],
}

pub struct ForkIterProps {
    pub drop: bool,
    pub voxel_id: usize,
    pub node_idx: usize,
}

impl<'a> Layer<'a> {
    /// Index to Base layer
    pub const BASE: usize = 0;
    /// Index to Tmp layer
    pub const TMP: usize = 1;
    /// Index to Schem layer
    pub const SCHEM: usize = 2;
    /// Index to Canvas layer
    pub const CANVAS: usize = 3;

    pub fn new(depth: usize, nodes: &'a mut [Node], l2_nodes: &'a mut [NodeL2]) -> Self {
        {
            // first free node head
            nodes[0].flag = -1;
            nodes[0].children[0] = 2;
            // Chain holders
            // First 2 nodes are not holders, the rest is holders
            // TODO: Decouple chaining holders and validating/creating layer
            // TODO: clear every node up
            for (i, free) in nodes.iter_mut().enumerate().skip(2) {
                // Mark as holder
                free.flag = -1;
                // Set link to next holder
                free.children[0] = i as u32 + 1;
            }

            nodes.last_mut().unwrap()[0] = 0;
        }
        // Do the same for level_2 nodes
        {
            // first free node head
            l2_nodes[0].set_child(0, 1);
            // Chain holders
            // First 2 nodes are not holders, the rest is holders
            // TODO: Decouple chaining holders and validating/creating layer
            // TODO: clear every node up
            for (i, free) in l2_nodes.iter_mut().enumerate().skip(1) {
                // Set link to next holder
                free.set_child(0, i as u32 + 1);
            }

            l2_nodes.last_mut().unwrap().set_child(0, 0);
        }

        (nodes, l2_nodes, depth).into()
    }
    #[deprecated(note = "Makes slower to deallocate node")]
    /// Validate if free nodes are in right order and sorted
    pub fn validate(&self) -> Result<(), String> {
        self.validate_base()?;
        self.validate_l2()?;
        Ok(())
    }
    #[deprecated]
    /// Validate only nodes on base level
    pub fn validate_base(&self) -> Result<(), String> {
        // current index
        let mut cidx = 0;
        loop {
            let node = &self.nodes[cidx];
            let next_idx = node.children[0] as usize;

            if next_idx == 0 {
                return Ok(());
            }

            if next_idx < cidx {
                return Err("base nodes are not sorted, it is a bug.".into());
            }

            cidx = next_idx;
        }
    }
    #[deprecated]
    /// Validate only nodes on 2 level
    pub fn validate_l2(&self) -> Result<(), String> {
        // current index
        let mut cidx = 0;
        loop {
            let node = &self.level_2[cidx];
            let next_idx = node.packed_children[0] as usize;

            if next_idx == 0 {
                return Ok(());
            }

            if next_idx < cidx {
                return Err("level-2 nodes are not sorted, it is a bug.".into());
            }

            cidx = next_idx;
        }
    }
    /// Show free space in Nodes
    pub fn free(&self) -> usize {
        let mut idx = 0;
        let mut free = 0;
        loop {
            idx = self[idx][0] as usize;
            if idx == 0 {
                break;
            }
            free += 1;
            assert!(free < self.nodes.len())
        }

        free
    }

    /// Show free space on level 2
    pub fn free_l2(&self) -> usize {
        let mut idx = 0;
        let mut free = 0;
        loop {
            idx = self.level_2[idx].packed_children[0] as usize;
            if idx == 0 {
                break;
            }
            free += 1;
            assert!(free < self.level_2.len())
        }

        free
    }

    /// Deallocate node with holder-pool
    pub fn deallocate_node<N: AllocatableNode + 'a>(&mut self, node_idx: usize) {
        // Append empty linked list
        let first_free_node_idx = N::get_first_free_link(self);
        let node = N::get_node_mut(self, node_idx);
        node.set_flag(-1);
        node.set_child(0, first_free_node_idx);

        // Change the actual head to new one
        N::set_first_free_link(self, node_idx as u32);
    }

    /// Allocate node from holder-pool
    pub fn allocate_node<N: AllocatableNode + Default + 'a>(&mut self) -> usize {
        self.allocate_node_from(N::default())
    }
    /// Allocate node from pool from given node
    pub fn allocate_node_from<N: AllocatableNode + 'a>(&mut self, node: N) -> usize {
        if N::get_first_free_link(self) != 0 {
            // Taking node linked by head
            let allocated_idx = N::get_first_free_link(self) as usize;
            let allocated_node = N::get_node_mut(self, allocated_idx);
            // Safety check
            assert_eq!(allocated_node.get_flag(), -1);
            // Setting link from allocated_idx to head
            let allocated_node_child = allocated_node.get_child(0);

            N::set_first_free_link(self, allocated_node_child);
            // Set
            let used_noded = N::get_node_mut(self, allocated_idx); // = node;
            *used_noded = node;
            allocated_idx
        } else {
            panic!("You are out of holder-nodes on type: {}", N::name());
        }
    }

    // TODO: Rework and move to [super::op]
    pub fn iter_fork<C: FnMut(&mut ForkIterProps)>(&self, mut fork_idx: usize, callback: &mut C) {
        if !self[fork_idx as usize].is_fork() {
            panic!()
        }

        loop {
            let fork = &self[fork_idx as usize];
            for voxel_id_idx in 0..4 {
                let (found_voxel_id, found_node_idx) =
                    (fork[voxel_id_idx * 2], fork[(voxel_id_idx * 2) + 1]);
                if found_voxel_id == 0 {
                    return;
                }
                let mut props = ForkIterProps {
                    drop: false,
                    voxel_id: found_voxel_id as usize,
                    node_idx: found_node_idx as usize,
                };
                callback(&mut props);

                if props.drop {
                    return;
                }
            }
            let next_opt = fork.flag;
            if next_opt > 0 {
                // Switch context until found or run out of links to branches
                fork_idx = next_opt as usize;
            } else if next_opt == -3 {
                return;
            } else {
                panic!();
            }
        }
    }
    // TODO: Rework and move to [super::op]
    /// Allows to work without even thinking about forks
    /// Usefull if node points to fork, but this abstracts it, and just returns idx to node below this all
    pub fn set_child(
        &mut self,
        from_idx: usize,
        voxel_id: u32,
        child_idx: usize,
        level: usize,
        fork_level: usize,
    ) -> usize {
        if voxel_id == 0 {
            panic!();
        }

        let node = &mut self[from_idx];

        // If children of node on from_idx should point to fork
        if level == fork_level + 1 {
            let node_below_idx = node.children[child_idx];

            if node_below_idx == 0 {
                // Fork does not exist, lets change that
                let new_branch_idx = self.allocate_node::<Node>();

                let fork = Node {
                    flag: -3,
                    children: [voxel_id, new_branch_idx as u32, 0, 0, 0, 0, 0, 0],
                };

                let fork_idx = self.allocate_node_from(fork);

                self[from_idx].children[child_idx] = fork_idx as u32;

                return new_branch_idx;
            } else {
                let mut fork_idx = node_below_idx;

                loop {
                    let fork = &mut self[fork_idx as usize];
                    for voxel_id_idx in 0..4 {
                        let found_voxel_id = fork[voxel_id_idx * 2];
                        if found_voxel_id == 0 {
                            // Nothing here, lets fill it
                            // So we can allocate our new node to voxel_id
                            let new_branch_idx = self.allocate_node::<Node>();

                            self[fork_idx as usize][voxel_id_idx * 2] = voxel_id;
                            self[fork_idx as usize][(voxel_id_idx * 2) + 1] = new_branch_idx as u32;

                            return new_branch_idx;
                        }
                        if found_voxel_id == voxel_id {
                            // Bravo, we found idx to forward!
                            return fork[(voxel_id_idx * 2) + 1] as usize;
                        }
                    }
                    let next_opt = fork.flag;
                    if next_opt > 0 {
                        // Switch context until found or run out of links to branches
                        fork_idx = next_opt as u32;
                    } else {
                        // If you are at this point, its only possible if there is not place left
                        // To fix that, lets extend it with new fork
                        // Fork does not exist, lets change that
                        let new_branch_idx = self.allocate_node::<Node>();

                        let fork = Node {
                            flag: -3,
                            children: [voxel_id, new_branch_idx as u32, 0, 0, 0, 0, 0, 0],
                        };

                        let fork_id = self.allocate_node_from(fork);

                        self[fork_idx as usize].flag = fork_id as i32;

                        return new_branch_idx;
                    }
                }
            }
        } else {
            let node_below_idx = node.children[child_idx];
            if node_below_idx == 0 {
                let new_child_idx = self.allocate_node::<Node>();

                self[from_idx].children[child_idx] = new_child_idx as u32;
                return new_child_idx;
            } else {
                return node.children[child_idx] as usize;
            }
        }
    }
}
/// A little helper
/// let layer: Layer = (nodes, l2_nodes, depth).into();
impl<'a> From<(&'a mut [Node], &'a mut [NodeL2], usize)> for Layer<'a> {
    fn from(value: (&'a mut [Node], &'a mut [NodeL2], usize)) -> Self {
        Layer {
            depth: value.2,
            level_2: value.1,
            nodes: value.0,
        }
    }
}

impl<'a> Index<usize> for Layer<'a> {
    type Output = Node;

    fn index(&self, index: usize) -> &Self::Output {
        &self.nodes[index]
    }
}

impl<'a> IndexMut<usize> for Layer<'a> {
    fn index_mut(&mut self, index_mut: usize) -> &mut Self::Output {
        &mut self.nodes[index_mut]
    }
}

#[cfg(feature = "std")]
#[cfg(test)]
mod tests {

    use rand::Rng;

    use crate::plat::node_l2::NodeL2;
    use crate::plat::raw_plat::RawPlat;
    use crate::{plat::node::Node, quick_raw_plat, test_utils::set_rand_plat};
    extern crate alloc;
    extern crate std;
    #[test]
    fn count_free() {
        quick_raw_plat!(plat, depth 10, len 600_060);

        set_rand_plat::<64>(&mut plat, 50);

        let mut free_count = 0;

        for node in plat[0].nodes.iter().skip(1) {
            if node.flag == -1 {
                free_count += 1;
            }
        }

        // TODO:
        // let mut free_l2_count = 0;
        // for node in plat[0].level_2.iter() {
        //     if node.flag == -1 {
        //         free_count += 1;
        //     }
        // }

        assert_eq!(free_count, plat[0].free());
    }

    #[test]
    fn count_free_l2_after_creation() {
        quick_raw_plat!(plat, depth 10, len 600_060, len2 600_000, lenrest 10);

        assert_eq!(plat.layers[0].level_2.len() - 1, plat[0].free_l2());
    }

    #[ignore]
    #[test]
    fn check_if_sorted() {
        quick_raw_plat!(plat, depth 10, len 1000, len2 1000, lenrest 1000);

        let layer = &mut plat.layers[0];

        let mut rng = rand::thread_rng();

        for _ in 0..rng.gen_range(1..1000) {
            layer.allocate_node::<Node>();
            layer.allocate_node::<NodeL2>();
        }

        for _ in 0..100 {
            layer.deallocate_node::<Node>(rng.gen_range(0..1000));
            layer.deallocate_node::<NodeL2>(rng.gen_range(0..1000));
        }

        // for _ in 0..100 {
        //     layer.allocate_node::<Node>();
        //     layer.allocate_node::<NodeL2>();
        // }

        assert_eq!(layer.validate(), Ok(()));
    }
    #[test]
    fn deallocate_non_allocated_node() {
        quick_raw_plat!(plat, depth 10, len 1000, len2 1000, lenrest 1000);
        let layer = &mut plat.layers[0];

        layer.deallocate_node::<Node>(10);
        layer.deallocate_node::<NodeL2>(10);
        // TODO
        panic!("This actually can lead to undefined behaviour, deallocator can never know if node allocated or not")
    }
    #[test]
    fn deallocate_non_existing() {
        quick_raw_plat!(plat, depth 10, len 1000, len2 1000, lenrest 1000);
        let layer = &mut plat.layers[0];

        // TODO: Make some nice error wrapper
        layer.deallocate_node::<Node>(1002);
        layer.deallocate_node::<NodeL2>(1002);
    }
}
