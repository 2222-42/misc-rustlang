fn call_twice<F>(mut closure: F)
where
    F: FnMut(),
{
    closure();
    closure();
}

fn main() {
    let my_str = "hello".to_string();
    let kill_closure = || drop(my_str);
    kill_closure();
    // call_twice(kill_closure);

    let mut i = 0;
    let incr = || {
        i += 1;
        println!("i: {}", i);
    };
    call_twice(incr);
    assert_eq!(i, 2);
}
