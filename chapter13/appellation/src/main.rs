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

fn main() {
    let mut app = Appellation {
        name: String::from("Stephen"),
        nicknames: vec!["2222-42".to_string(), "Daioh".to_string()],
    };
    println!("before assingment");
    app = Appellation {
        name: String::from("Hora"),
        nicknames: vec![],
    };
    println!("at end of block")
}
