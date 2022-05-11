use rust_closures_iterators::trees::BinaryTree;

fn main() {
    let mut tree = BinaryTree::new();
    tree.add("D");
    tree.add("A");
    tree.add("B");
    tree.add("C");

    // Let's iterate over it.
    for value in &tree {
        print!(" {}", *value);
    }
}
