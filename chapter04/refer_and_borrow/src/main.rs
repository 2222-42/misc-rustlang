fn main() {
    let s1 = String::from("Hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}", s1, len);

    change(&s1);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &String) {
    some_string.push_str(", world")
}
