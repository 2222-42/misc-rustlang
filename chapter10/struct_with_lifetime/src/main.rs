struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentne = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentne };
}
