fn main() {
    let v: Vec<i32> = Vec::new();
    {
        let v2 = vec![1, 2, 3, 4, 5];
        let third = &v2[2];
        println!("The third element is {}", third);
        let third = v2.get(2);
        // let does_not_exist = &v2[100];
        let does_not_exist = &v2.get(100);
        for i in &v2 {
            println!("{}", i);
        }
    }

    let mut mv = Vec::new();
    mv.push(5);
    mv.push(6);
    mv.push(7);
    mv.push(5);
    for i in &mut mv {
        *i += 50;
    }

    let first = &mv[0];
    // mv.push(6);
    println!("The third element is {}", first);
}
