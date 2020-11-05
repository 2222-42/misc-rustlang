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
}
