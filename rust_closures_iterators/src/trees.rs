/// `BinaryTreeNode` is a node in a `BinaryTree`.
pub struct BinaryTreeNode<T> {
    value: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

impl<T> BinaryTreeNode<T> {
    pub fn new(value: T) -> Self {
        BinaryTreeNode {
            value,
            left: BinaryTree::Empty,
            right: BinaryTree::Empty,
        }
    }
}

/// `BinaryTree` is a simple implementation of a binary tree.
pub enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<BinaryTreeNode<T>>),
}

impl<T: Ord> BinaryTree<T> {
    pub fn new() -> Self {
        BinaryTree::Empty
    }

    pub fn add(&mut self, value: T) {
        match *self {
            BinaryTree::Empty => {
                let node = BinaryTreeNode::new(value);
                *self = BinaryTree::NonEmpty(Box::new(node))
            }
            BinaryTree::NonEmpty(ref mut node) => {
                if value <= node.value {
                    node.left.add(value);
                } else {
                    node.right.add(value);
                }
            }
        }
    }
}

impl<T> BinaryTree<T> {
    fn iter(&self) -> BinaryTreeIter<T> {
        let mut iter = BinaryTreeIter {
            unvisited: Vec::new(),
        };
        iter.push_left_edge(self);
        iter
    }
}

impl<'a, T: 'a> IntoIterator for &'a BinaryTree<T> {
    type Item = &'a T;
    type IntoIter = BinaryTreeIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

// ------ The Iterator ------

/// An iterator for the `BinaryTree`, keeping the state of the in-order traversal of the tree.
pub struct BinaryTreeIter<'a, T> {
    /// A stack of references to the tree nodes, the top of the stack is the end of the vector.
    unvisited: Vec<&'a BinaryTreeNode<T>>,
}

impl<'a, T: 'a> BinaryTreeIter<'a, T> {
    fn push_left_edge(&mut self, mut tree: &'a BinaryTree<T>) {
        while let BinaryTree::NonEmpty(ref node) = *tree {
            self.unvisited.push(node);
            tree = &node.left;
        }
    }
}

impl<'a, T> Iterator for BinaryTreeIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.unvisited.pop()?;
        self.push_left_edge(&node.right);
        Some(&node.value)
    }
}
