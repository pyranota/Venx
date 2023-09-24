use super::tetree::TeTree;

impl TeTree {
    pub fn get(&self, position: u32) -> Option<(i32, i32)> {
        let mut global_counter = 0;

        return visit_node(self, 0, &mut global_counter, position);

        fn visit_node(
            tree: &TeTree,
            idx: usize,
            global_counter: &mut u32,
            position: u32,
        ) -> Option<(i32, i32)> {
            let node = &tree.nodes[idx];
            if let Some(branch) = node.get_branch() {
                if position > *global_counter + branch.count {
                    *global_counter += branch.count;

                    return None;
                }

                for child in branch.children {
                    if child == 0 {
                        continue;
                    }

                    let output = visit_node(tree, child as usize, global_counter, position);

                    if output.is_some() {
                        return output;
                    }
                }
            } else if let Some(leaf) = node.get_leaf() {
                if position > *global_counter + leaf.count {
                    *global_counter += leaf.count;
                    return None;
                } else {
                    return Some((leaf.block_id, leaf.state));
                }
            }

            None
        }
    }
}

#[test]
fn test_attr_get() {
    let mut tree = TeTree::new();
    tree.insert(0, 6, (1, 1));
    tree.insert(4, 3, (2, 2));

    dbg!(tree.get(6));
}
