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

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_, y, _) = tup;
    println!("The value of y is: {}", y);
    println!("The value of x is: {}", tup.0);
    let z = tup.2;
    println!("The value of z is: {}", z);

    let a = [1, 2, 3, 4, 5, 6];
    let first = a[0];
    let second = a[1];
    println!("The value of first is: {}", first);
    println!("The value of second is: {}", second);

    // let index = 10;
    // let element = a[index];
    // println!("The value of element is: {}", element);
}
