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

    // let t = s[0];
}
