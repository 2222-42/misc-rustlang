fn foo(_: i32, y: i32) {
    println!("This code only use the y parameter: {}", y);
}

fn main() {
    foo(3, 4);
}
