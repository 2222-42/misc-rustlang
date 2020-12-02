struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, annoucement: &str) -> &str {
        println!("Attention please: {}", annoucement);
        self.part
    }
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentne = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentne };

    println!("level: {}", i.level());
    let s = i.announce_and_return_part("");
    println!("{}", s);
}
