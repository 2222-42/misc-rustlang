use iter_binary_tree::binary_tree::{BinaryTree, TreeNode};

fn main() {
    use BinaryTree::*;
    let jupiter_tree = NonEmpty(Box::new(TreeNode {
        element: "Jupiter",
        left: Empty,
        right: Empty,
    }));

    let mercury_tree = NonEmpty(Box::new(TreeNode {
        element: "Mercury",
        left: Empty,
        right: Empty,
    }));

    let mars_tree = NonEmpty(Box::new(TreeNode {
        element: "Mars",
        left: jupiter_tree,
        right: mercury_tree,
    }));

    let uranus_tree = NonEmpty(Box::new(TreeNode {
        element: "Uranus",
        left: Empty,
        right: Empty,
    }));

    let tree = NonEmpty(Box::new(TreeNode {
        element: "Saturn",
        left: mars_tree,
        right: uranus_tree,
    }));

    println!("{:?}", tree);

    let mut tree = BinaryTree::Empty;
    tree.add("Mercury");
    tree.add("Saturn");
    tree.add("Mars");
    tree.add("Jupiter");

    println!("{:?}", tree);
}
