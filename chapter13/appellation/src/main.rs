use rand::prelude::*;
use std::vec;

struct Appellation {
    name: String,
    nicknames: Vec<String>,
}

impl Drop for Appellation {
    fn drop(&mut self) {
        println!("Dropping {}", self.name);
        if !self.nicknames.is_empty() {
            println!("(AKA {})", self.nicknames.join(", "));
        }
        println!("");
    }
}

fn random_condition() -> bool {
    let mut rng = rand::thread_rng();
    let y: i32 = rng.gen();
    y % 2 == 0
}

fn main() {
    let mut _app = Appellation {
        name: String::from("Stephen"),
        nicknames: vec!["2222-42".to_string(), "Daioh".to_string()],
    };
    println!("before assingment");
    _app = Appellation {
        name: String::from("Hora"),
        nicknames: vec![],
    };
    println!("at end of block");

    let _p;
    {
        let q = Appellation {
            name: String::from("Zeus"),
            nicknames: vec!["cloud collector".to_string(), "king of gods".to_string()],
        };
        if random_condition() {
            _p = q;
        }
        println!("at end of inner block");
    }
    println!("at end of block");
}
