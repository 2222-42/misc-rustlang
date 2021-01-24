static HELLO_WORLD: &str = "Hello, world!";
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    println!("name is : {}", HELLO_WORLD);

    add_to_count(1);
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}
