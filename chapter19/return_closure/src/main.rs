fn return_closure() -> Box<dyn Fn(i32) -> i32>{
    Box::new(|x| x + 1)
}

fn main() {
    let list_of_int: Vec<i32> = vec![1,2,3];
    let closure = return_closure();
    let list_of_plussed : Vec<i32> = list_of_int.iter().map(|x| closure(*x)).collect();
    println!("{:?}", list_of_plussed)
}
