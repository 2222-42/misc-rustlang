fn main() {
    loop {
        println!("Hello, world!");
        break;
    }

    let mut number = 3;
    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }
    println!("LIFTOFF!");

    let a = [10, 20, 30, 40, 50];
    for elem in a.iter() {
        println!("the value is {}", elem)
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!");
}
