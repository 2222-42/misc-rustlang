fn main() {
    let v: Vec<i32> = Vec::new();
    {
        let v2 = vec![1, 2, 3];
    }

    let mut mv = Vec::new();
    mv.push(5);
    mv.push(6);
    mv.push(7);
    mv.push(5);

    println!("Hello, world!");
}
