use crate::plat::{node::AllocatableNode, node_l2::NodeL2};

impl crate::plat::layer::layer::Layer<'_> {
    /// Rearrange all nodes so `free` nodes can be just cutted of
    /// Returns index, by which user can cut
    /// First index is `nodes` and second for `level2`
    pub fn shrink_to_fit_all() -> (usize, usize) {
        todo!()
    }

    /// Shrinks only `layer.level2`
    pub fn shrink_to_fit_l2(&mut self, helper: &mut [usize]) -> usize {
        /*
            How do we do that?

            We need to move all `free` nodes at the end

            But we cant just use `swap`, it will break linking

            Instead we gonna swap and mark it down

            And on the second pass just relink everything

            Here is the general algorithm:
                1. Loop through all free nodes and reflect those on helper
                2. Iterate through reflected nodes from start to end
                and swap all free nodes with branches on the end.
                > Mark all swapped nodes
                > And clean all "free links"

                > After 2. stage we should have helper with links at the end pointing to new location

                3. Iterate one more time and relink everything according to helper
        */
        assert_eq!(helper.len(), self.level_2.len());
        assert!(self.level_2.len() > 0);
        // cache level_2
        let l2 = &mut self.level_2;
        // current free index
        let mut cidx = 0;
        // Stage 1: Reflecting free nodes
        loop {
            let free = l2[cidx];

            let next = free.packed_children[0] as usize;
            // We cant let next to be 0, we just could not detect last free node in helper slice
            // which could lead to undefined behaviour
            // To solve this we mark that the last free node in helper points to u32::MAX
            let next = if next == 0 { usize::MAX } else { next };

            helper[cidx] = next;

            cidx = next;

            // Exit
            if cidx == usize::MAX {
                break;
            }
        }

        // index which will be returned
        let mut cut_off = l2.len() - 1;
        let mut cidx = 0;
        // Stage 2: Swapping
        loop {
            let next = l2[cidx].packed_children[0];

            let last = &mut helper[cut_off];
            // Check if it is free node or not
            // We dont want to swap free with free
            // We only want free with branch
            // In prev step we only marked free nodes
            // So all branches marked as 0
            if *last != 0 {
                // To prevent from errors on the second stage
                // we need to clean up all marked free nodes
                // that will leave us only swapped links

                *last = 0;
                cut_off -= 1;
            }

            // Once we make sure we dont swap free with free or branch with branch
            l2.swap(cidx, cut_off);
            // cidx = next as usize;

            if cidx == 0 || cidx == usize::MAX || cut_off == 0 {
                break;
            }
        }
        // Stage 3: Relinking
        for node in self.nodes.iter_mut() {
            // USEDFORK
            if node.is_fork() {
                continue;
            }

            for child in node.children.iter_mut() {
                let potential_link = helper[*child as usize];

                if potential_link != 0 {
                    *child = potential_link as u32;
                }
            }
        }
        cut_off
    }

    /// Shrinks only `layer.nodes`
    /// Completely removes all empty nodes
    pub fn freeze_upper(&mut self, helper: &mut [usize]) -> usize {
        /*
            1. Find free node with lowest index at the same time fill all free nodes with zeros

            > We fill all of them with zeros, becuz it will help us find free nodes in future
            > That happens becuz if branch fills with zeros, it simply get deallocated (transformed into free node)
            > So only free nodes can be zeroed and not deallocated

            2. Set working range from smallest free node idx to the end
            3. Iterate through this range and begin swapping until `cut_off_idx` == `current_idx`. Dont forget to write changes in helper
            4. Relink everything by going through all nodes

            > 4. Step is pretty slow, so if you need instant result, use `freeze_fast` instead
        */

        assert_eq!(helper.len(), self.level_2.len());
        assert!(self.level_2.len() > 0);
        // cache level_2
        let l2 = &mut self.level_2;
        // current free index
        let mut cidx = 0;

        let mut smallest_free_node_idx: usize = usize::MAX;
        // Stage 1
        loop {
            let free = &mut l2[cidx];

            let next = free.packed_children[0] as usize;

            if cidx < smallest_free_node_idx && cidx != 0 {
                smallest_free_node_idx = cidx;
            }
            // Fill with zeros
            *free = NodeL2::default();

            cidx = next;
            // Exit
            if cidx == 0 {
                break;
            }
        }
        // cut point
        let mut cut_idx = l2.len() - 1;
        // current idx
        let mut cur_idx = smallest_free_node_idx;
        while cut_idx > cur_idx {
            let cur_node = &l2[cur_idx];
            let cut_node = &l2[cut_idx];
            // Check if it is a free node or not
            if cur_node.packed_children == [0, 0] {
                // Check if last node is branch
                if cut_node.packed_children != [0, 0] {
                    l2.swap(cur_idx, cut_idx);
                    // We need these changes later
                    // So lets save
                    helper[cut_idx] = cur_idx;
                } else {
                    cut_idx -= 1;
                }
            } else {
                cur_idx += 1;
            }
        }
        cut_idx
    }

    /// Shrinks only `layer.nodes`
    /// Almost Zero-cost operation
    /// Removes only free nodes that are on tail and have no fragmentation
    pub fn freeze_upper_fast() -> usize {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::quick_raw_plat;
    extern crate alloc;
    extern crate std;
    use crate::plat::{node::Node, raw_plat::RawPlat};
    use crate::test_utils::set_rand_plat;

    #[test]
    fn freeze_l2_empty() {
        quick_raw_plat!(plat);

        let cut_off =
            plat.layers[0].freeze_upper(&mut alloc::vec![0; plat.layers[0].level_2.len()]);
        assert_eq!(cut_off, 1);
    }

    #[test]
    fn freeze_l2_single() {
        quick_raw_plat!(plat, depth 5);

        plat.layers[0].set((0, 0, 0).into(), 99);

        let cut_off =
            plat.layers[0].freeze_upper(&mut alloc::vec![0; plat.layers[0].level_2.len()]);
        assert_eq!(cut_off, 2);
    }

    #[test]
    fn freeze_l2_many() {
        quick_raw_plat!(plat, depth 10, len 200_060);

        set_rand_plat::<64>(&mut plat, 90);
        plat.layers[0].freeze_upper(&mut alloc::vec![0; plat.layers[0].level_2.len()]);
    }
}
