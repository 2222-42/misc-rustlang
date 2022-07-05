use crate::binary_tree::{BinaryTree::{self, *}, TreeNode};
pub struct TreeIter<'a, T: 'a> {
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
        let node = self.unvisited.pop()?;
        self.push_left_edge(&node.right);
        Some(&node.element)
    }

    // ベクタへの変換の実装をするなら、size_hint()を実装したほうがバッファの拡張のコストが下がる。
}

impl<T> BinaryTree<T> {
    pub fn iter(&self) -> TreeIter<T> {
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

// impl<'a, T: 'a> IntoIterator for &'a mut BinaryTree<T> {
//     type Item = T;
//     type IntoIter = TreeIterMut<'a, T>;
//     fn into_iter(self) -> Self::IntoIter {
//         self.iter_mut()
//     }
// }

impl <T: Ord> FromIterator<T> for BinaryTree<T> {
    fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
        let mut tree = Empty;
        for x in iter {
            tree.add(x);
        }
        tree
    }
}