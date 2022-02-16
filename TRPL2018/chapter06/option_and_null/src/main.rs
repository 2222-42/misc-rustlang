fn main() {
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    let x = 5;

    // let sum = some_number + 5;
    let sum = some_number.unwrap() + x;
    println!("the value is {}", sum);
    let sum2 = absent_number.unwrap_or(0) + x;
    println!("the value is {}", sum2);
}
