use iter_binary_tree::binary_tree::{BinaryTree};

fn main() {

    let mut tree = BinaryTree::Empty;
    tree.add("10");
    tree.add("04");
    tree.add("08");
    tree.add("03");
    tree.add("07");
    tree.add("09");
    tree.add("05");
    tree.add("06");
    tree.add("13");
    tree.add("11");
    tree.add("14");

    println!("{:?}", tree);

    let mut v = Vec::new();
    for x in &tree {
        v.push(*x);
    }
    assert_eq!(v, vec!["03", "04","05", "06", "07", "08", "09", "10", "11", "13", "14"]);

    let added_prefix = tree.iter().map(|x| format!("number-{}", x)).collect::<Vec<_>>();
    assert_eq!(
        added_prefix,
        vec!["number-03", "number-04", "number-05", "number-06", "number-07", "number-08", "number-09", "number-10", "number-11", "number-13", "number-14"]
    );

    let vec = [10,4,8,3,7,9,5,6,13,11,14];
    let tree = BinaryTree::from_iter(vec.iter().cloned());
    // let tree: BinaryTree<i32> = vec.iter().cloned().collect();
    assert_eq!(
        tree.iter().map(|x| format!("number-{}", x)).collect::<Vec<_>>(), 
        vec!["number-3", "number-4", "number-5", "number-6", "number-7", "number-8", "number-9", "number-10", "number-11", "number-13", "number-14"]);

    let tree2: BinaryTree<i32> = vec.iter().cloned().collect();
    assert_eq!(90, tree2.iter().fold(0, |n, i| n + i));

    // tree.iter().map(|x| format!("number-{}", x));
}
