use super::tetree::{TNode, TeTree};

impl TeTree {
    pub fn insert(&mut self, position: u32, amount: u32, block: i32) {
        self.insert_rec(0, 0, position, amount, block);
    }

    fn insert_rec(
        &mut self,
        idx: usize,
        mut global_count: u32,
        position: u32,
        amount: u32,
        block: i32,
    ) -> u32 {
        let node = &mut self.nodes[idx];
        if let Some(branch) = node.get_branch_mut() {
            if global_count + branch.count >= position {
                branch.count += amount;

                for child in branch.children {
                    global_count +=
                        self.insert_rec(child as usize, global_count, position, amount, block);
                }
            } else {
                return global_count;
            }
        } else if let Some(leaf) = node.get_leaf_mut() {
            if global_count + leaf.count >= position {
                // leaf.count += position;
                // Insert child here
                if leaf.block_id == block {
                    leaf.count += amount;
                } else {
                    let leaf = leaf.clone();
                    let left_id = self.add_node(TNode::new_leaf(
                        leaf.count - amount,
                        leaf.block_id,
                        leaf.state,
                    ));
                    let center_id = self.add_node(TNode::new_leaf(amount, block, 88));
                    let right_id =
                        self.add_node(TNode::new_leaf(amount, leaf.block_id, leaf.state));

                    let mut blank_node = TNode::new_branch(global_count + leaf.count + amount);
                    let branch = blank_node.get_branch_mut().unwrap();

                    branch.children[0] = left_id as i32;
                    branch.children[1] = center_id as i32;
                    branch.children[2] = right_id as i32;

                    self.nodes[idx] = blank_node;
                }
            } else {
                return global_count;
            }
        }
        global_count
    }
}

#[test]
fn test_tetree_insert() {
    let mut tree = TeTree::new();
    tree.insert(0, 20, 2);
    dbg!(tree);
}
