fn five() -> i32 {
    5
}

fn main() {
    println!("Hello, world!");

    // let x = (let y = 6);
    let x = five();
    let x = plus_one(x);

    let y = {
        let x = 3;
        x + 1
    };
    another_function(x, y);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
