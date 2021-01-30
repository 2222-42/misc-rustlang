fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);

    println!("The answer is {}", answer);

    let list_of_numbers = vec![1,2,3];
    let list_of_strings1: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
    let list_of_strings2: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();
    println!("The list_of_string is {:?}", list_of_strings1);
    assert_eq!(list_of_strings1, list_of_strings2);
}
