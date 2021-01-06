use std::rc::Rc;
use std::cell::RefCell;

struct Node {
    value: u32,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node{
        value:3,
        children:RefCell::new(vec![]),
    });

    let branch = Rc::new(Node{
        value: 5,
        children:RefCell::new(vec![Rc::clone(&leaf)])
    });
}
