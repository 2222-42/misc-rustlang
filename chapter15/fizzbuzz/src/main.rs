use std::{iter::once, iter::repeat};

fn main() {
    let fizzes = repeat("").take(2).chain(once("fizz")).cycle();
    let buzzes = repeat("").take(4).chain(once("buzz")).cycle();
    let fizzbuzzes = fizzes.zip(buzzes);
    let fizz_buzz = (1..100).zip(fizzbuzzes).map(|tuple| match tuple {
        (i, ("", "")) => i.to_string(),
        (_, (fizz, buzz)) => format!("{}{}", fizz, buzz),
    });

    println!("{}", fizz_buzz.collect::<Vec<_>>().join("\n"));
    // for line in fizz_buzz {
    //     println!("{}", line);
    // }
}
