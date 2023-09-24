use super::tetree::{TNode, TeTree};

impl TeTree {
    pub fn insert(&mut self, position: u32, amount: u32, (block, state): (i32, i32)) {
        let mut global_counter = 0;
        let _ = self.goto_node(0, &mut global_counter, position, amount, block, state);
    }

    fn goto_node(
        &mut self,
        idx: usize,
        global_counter: &mut u32,
        position: u32,
        amount: u32,
        block: i32,
        state: i32,
    ) -> bool {
        let node = &mut self.nodes[idx];
        if let Some(branch) = node.get_branch() {
            // If position is still larger then
            // Amount of nodes in entire subbranch + all branches before
            // It means, we should go further.
            if position > *global_counter + branch.count {
                *global_counter += branch.count;
                return false;
            }

            // Now we know, that it is the subbranch we were looking for.
            // Lets iterate over it`s children to go deeper.
            for child in branch.children {
                // Well, if child is zero, its bad.
                // It should`h be gone after `optimize()`
                // We will just ignore that
                if child == 0 {
                    continue;
                }
                // We dont only need to set the node on given position,
                // But also change the path counter,
                // Whats why we are using change_count

                let is_changed = self.goto_node(
                    child as usize,
                    global_counter,
                    position,
                    amount,
                    block,
                    state,
                );
                // If 0 it means, that is the wrong branch. lets go further
                if is_changed {
                    // Should access it again, cuz of rust owner-borrowing thing
                    self.nodes[idx].get_branch_mut().unwrap().count += amount as u32;
                    return true;
                }
            }
        } else if let Some(leaf) = node.get_leaf_mut() {
            if position > *global_counter + leaf.count {
                *global_counter += leaf.count;
                return false;
            }
            /*
                If we got to this point, than in 100% we found what we were searching.
                There are some cases:
                1. It splits on the middle
                2. Goes before // Leaves empty child, which is bad // call `optimize()`
                3. Does not split at all
                4. Leaf is filled with air, so we just take it.
            */
            // 4.
            if (leaf.block_id, leaf.state) == (0, 0) {
                leaf.count = amount;
                leaf.block_id = block;
                leaf.state = state;
            }
            // 3.
            else if (leaf.block_id, leaf.state) == (block, state) {
                leaf.count += amount;
            }
            // 2.
            else if position == *global_counter {
                // Spliting the leaf on 2
                // Copy
                let leaf = *leaf;
                let left = self.add_node(TNode::new_leaf(amount, block, state));
                let center = self.add_node(TNode::new_leaf(leaf.count, leaf.block_id, leaf.state));

                // Getting current node again, but this time as branch
                let branch = self.nodes[idx].get_branch_mut_unchecked();
                branch.children[0] = left as i32;
                branch.children[1] = center as i32;
                branch.children[2] = 0;
                branch.count += amount;
            }
            // 1.
            else {
                // Spliting the leaf on 3
                // Copy
                let leaf = *leaf;
                let new_leaf_local_position = position - *global_counter;
                let left = self.add_node(TNode::new_leaf(
                    new_leaf_local_position,
                    leaf.block_id,
                    leaf.state,
                ));
                let center = self.add_node(TNode::new_leaf(amount, block, state));
                let right = self.add_node(TNode::new_leaf(
                    leaf.count - new_leaf_local_position,
                    leaf.block_id,
                    leaf.state,
                ));

                // Getting current node again, but this time as branch
                let branch = self.nodes[idx].get_branch_mut_unchecked();
                branch.children[0] = left as i32;
                branch.children[1] = center as i32;
                branch.children[2] = right as i32;
                branch.count += amount;
            }

            return true;
        }

        panic!("Hey hey, it should not be here");
    }
}

#[test]
fn test_tetree_insert() {
    let mut tree = TeTree::new();
    tree.insert(0, 20, (2, 10));
    tree.insert(5, 10, (1, 11));
    tree.insert(25, 5, (111, 111));
    dbg!(tree);
}
#[test]
fn test_tetree_insert2() {
    let mut tree = TeTree::new();
    tree.insert(0, 1, (1, 0));
    tree.insert(1, 1, (2, 0));
    dbg!(tree);
}
