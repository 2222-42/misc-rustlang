fn main() {
    let s1 = String::from("Hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}", s1, len);

    let mut s = String::from("Hello");
    change(&mut s);
    println!("{}", s);

    let mut s = String::from("hello");
    {
        let r1 = &mut s;
        // let r2 = &mut s;
        change(r1);
    }
    let r2 = &mut s;
    change(r2);
    println!("{}", s);

    mut_and_immut_borrow();
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world")
}

fn mut_and_immut_borrow() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    // let r3 = &mut s;
    println!("{}", r1);
    println!("{}", r2);
    // change(r3);
    // println!("{}", s);
}
