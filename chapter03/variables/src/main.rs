fn main() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is {}", x);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("The length os spaces is {}", spaces);

    // let mut mspaces = "   ";
    // mspaces = mspaces.len();
    let x = 2.0;
    let y: f32 = 3.0;
    println!("The value of x is {}", x);
    println!("The value of y is {}", y);
    // if calculate ( x + y ), the type of x is infferred to be f32.
}
