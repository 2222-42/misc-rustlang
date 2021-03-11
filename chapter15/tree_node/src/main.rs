use std::rc::Rc;
use std::{borrow::Borrow, cell::RefCell};

pub struct TreeNode {
    pub children: Vec<Rc<RefCell<TreeNode>>>,
    pub parent: Option<Rc<RefCell<TreeNode>>>,
    pub value: f32,
}
impl TreeNode {
    #[inline]
    pub fn new(value: f32) -> Self {
        TreeNode {
            value,
            children: vec![],
            parent: None,
        }
    }
}

fn main() {
    let mut child_nd = Rc::new(RefCell::new(TreeNode::new(2.0)));
    {
        let mut root_nd = Rc::new(RefCell::new(TreeNode::new(5.0)));
        child_nd.borrow_mut().parent = Some(root_nd.clone()); // use Rc::clone to create a new reference to root_nd
        root_nd.borrow_mut().children.push(child_nd.clone());
    }

    // println!(
    //     "grandson parent = {:?}",
    //     child_nd.borrow().parent.borrow().upgrade()
    // );
}
