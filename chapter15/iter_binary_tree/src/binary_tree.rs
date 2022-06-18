#[derive(Debug)]
pub enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}

#[derive(Debug)]
pub struct TreeNode<T> {
    pub element: T,
    pub left: BinaryTree<T>,
    pub right: BinaryTree<T>,
}

impl<T: Ord> BinaryTree<T> {
    pub fn add(&mut self, value: T) {
        match *self {
            BinaryTree::Empty => {
                *self = BinaryTree::NonEmpty(Box::new(TreeNode {
                    element: value,
                    left: BinaryTree::Empty,
                    right: BinaryTree::Empty,
                }));
            }
            BinaryTree::NonEmpty(ref mut node) => {
                if value <= node.element {
                    node.left.add(value);
                } else {
                    node.right.add(value);
                }
            }
        }
    }
}