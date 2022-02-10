extern crate rand;

use rand::Rng;
use std::{cmp::Ordering, io};
// use std::cmp::Ordering;
// use std::io;

pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value mus be greater than or equal to 1, got: {}",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value mus be less than or equal to 100, got: {}",
                value
            );
        }
        Guess { value }
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Guess value mus be less than or equal to 100")]
    fn test_greater_than_100() {
        Guess::new(200);
    }
}

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => Guess::new(num).value(),
            Err(_) => continue, // The type of continue is `!`, so the type of this match is `u32`
        };
        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
