fn main() {
    let x = fib(20);
    println!("{}", x);
    let y = fast_fib(20);
    println!("{}", y);
}

fn fib(n: i64) -> i64 {
    if n < 2 {
        2
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

fn sub_fib(n: i64, f: i64, g: i64) -> i64 {
    if n == 0 {
        f
    } else if n == 1 {
        g
    } else {
        sub_fib(n - 1, g, f + g)
    }
}

fn fast_fib(n: i64) -> i64 {
    sub_fib(n, fib(0), fib(1))
}
