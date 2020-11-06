fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!");

    println!("{}", s);

    let x = 5;
    let y = x;
    println!("The value of x is {}, and one of y is {}", x, y);

    let s1 = String::from("hello");
    let s2 = s1;
    // println!("The value of s1 is {}", s1);
    println!("The value of s2 is {}", s2);

    let s3 = String::from("world");
    let s4 = s3.clone();
    println!("The value of s3 is {}", s3);
    println!("The value of s4 is {}", s4);

    let s = String::from("hello");
    takes_ownership(s);
    // println!("{}", s);

    makes_copy(x);
    println!("{}", x);

    let s1 = give_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    println!("The value of s1 is {}", s1);
    // println!("The value of s2 is {}", s2);
    println!("The value of s3 is {}", s3);

    let (s4, len) = calculate_length(s3);
    println!("The length of {} is: {}", s4, len);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn give_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}
