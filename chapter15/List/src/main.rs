enum List {
    Cons(i32, List),
    Nil,
}

fn main() {
    use List::{Cons, Nil};
    let list = Cons(1, Cons(2, Cons(3, Nil)));
}
