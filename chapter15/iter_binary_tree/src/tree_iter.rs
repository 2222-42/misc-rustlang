use BinaryTree::*;
struct TreeIter<'a, T: 'a> {
    unvisited: Vec<&'a TreeNode<T>>
}

impl <'a, T: 'a> TreeIter<'a, T> {
    fn push_left_edge(&mut self, mut tree: &'a BinaryTree<T>) {
        while let NonEmpty(ref node) = *tree {
            self.unvisited.push(node);
            tree = &node.left;
        }
    }
}

impl <'a, T: 'a> Iterator for TreeIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        // if let Some(node) = self.unvisited.pop() {
        //     self.push_left_edge(&node.right);
        //     Some(&node.element)
        // } else {
        //     None
        // }
        let node = self.unvisited.pop()?;
        self.push_left_edge(&node.right);
        Some(&node.element)
    }
}

impl<T> BinaryTree<T> {
    fn iter(&self) -> TreeIter<T> {
        let mut iter = TreeIter { unvisited: Vec::new() };
        iter.push_left_edge(self);
        iter
    }
}

impl<'a, T: 'a> IntoIterator for &'a BinaryTree<T> {
    type Item = &'a T;
    type IntoIter = TreeIter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}