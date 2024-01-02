#define_import_path TeTree
@group(3)
@binding(0)
var<storage, read_write> tree: TeTree;

struct TeTree {
    nodes: array<TNode>,
}

struct TNode {
    count: u32,
    shared_data: array<i32, 3> // First value indicats whether it leaf or not
}

fn is_leaf(node: ptr<function, TNode>) -> bool {
    return true;
}
fn is_branch(node: ptr<function, TNode>) -> bool {
    return true;
}
