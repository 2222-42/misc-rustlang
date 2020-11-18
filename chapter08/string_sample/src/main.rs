fn main() {
    let mut ms1 = String::new();
    ms1.push_str("foo");
    let s2 = "bar";
    ms1.push_str(s2);
    println!("ms1: {}", ms1);
    println!("s2: {}", s2);

    let data = "initial contents";
    let s = data.to_string();
    println!("{}", s);
    let s = "initial contents".to_string();
    println!("{}", s);
    let s = String::from("initial contents");
    println!("{}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    // println!("{}", s1);
    println!("{}", s2);
    println!("{}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{}", s);
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s1);
    println!("{}", s2);
    println!("{}", s3);
    println!("{}", s);
    let len = String::from("Hola").len();
    println!("{}", len);
    let hello = String::from("Здравствуйте");
    let len = hello.len();
    println!("{}", len);
    // let t = &hello[0];
    let s1 = &hello[0..4];
    println!("{}", s1);
    // let s2 = &hello[0..1];
    // println!("{}", s2);

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
