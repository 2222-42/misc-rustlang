fn main() {
    let mut s = String::from("Hello world!");
    let i = first_word(&s);
    println!("The length of first word is :{}", i);

    s.clear();
    println!("The length of first word is :{}", i);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
