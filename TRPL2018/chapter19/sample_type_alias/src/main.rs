type Thunk = Box<dyn Fn() + Send + 'static>;

fn takes_long_type(f: Thunk) {}

fn return_long_type() -> Thunk {
    Box::new(|| ())
}

fn main() {
    let f: Thunk = Box::new(|| println!("hi"));
}
