fn main() {
    let mut ms = String::new();

    let data = "initial contents";
    let s = data.to_string();
    println!("{}", s);
    let s = "initial contents".to_string();
    println!("{}", s);
    let s = String::from("initial contents");
    println!("{}", s);
}
