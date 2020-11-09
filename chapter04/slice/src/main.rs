fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    println!("The first word is: {}", word);
    let word2 = first_word(word);
    println!("The first word is: {}", word2);
    let world = &s[6..];
    println!("{}", world);
    println!("{}", &s[..]);

    s.clear();
    // println!("The first word is :{}", word);

    let my_string_literal = "hello world";
    let word = first_word(&my_string_literal[..]);
    println!("The first word is: {}", word);
    let word = first_word(my_string_literal);
    println!("The first word is: {}", word);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    for (_, &item) in slice.iter().enumerate() {
        println!("{}", item)
    }
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
