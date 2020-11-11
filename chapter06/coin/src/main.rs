enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
fn main() {
    let coin = Coin::Nickel;
    println!("{}", value_in_cents(coin));
    let coin = Coin::Penny;
    println!("{}", value_in_cents(coin));
}
